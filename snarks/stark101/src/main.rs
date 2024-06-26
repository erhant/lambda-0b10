#![allow(non_snake_case, unused_variables)]

use std::env;

use lambdaworks_crypto::{
    fiat_shamir::is_transcript::IsTranscript, merkle_tree::merkle::MerkleTree,
};
use lambdaworks_math::polynomial::Polynomial;
use stark101::{
    field::{
        generate_generator, generate_subgroup, get_subgroup_generator, Stark101PrimeFieldBackend,
        Stark101PrimeFieldElement as FE, Stark101PrimeFieldTranscript,
    },
    fri::{decommit_fri, fri_commit, BLOWUP_FACTOR},
    program::fibonacci_square,
    proof::{Stark101Commitment, Stark101Proof},
};

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    /////////////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////  PART 1  ////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////
    log::info!("Computing trace of FibonacciSq program");
    let n = 1023;
    let a_0 = FE::from(1u64);
    let a_1 = FE::from(3141592u64);
    let a = fibonacci_square(a_0, a_1, n);
    assert_eq!(a.len(), n);
    assert_eq!(*a.last().unwrap(), FE::from(2338775057u64));

    log::info!("Creating transcript");
    let mut channel = Stark101PrimeFieldTranscript::default();

    log::info!("Creating evaluation domain");
    let G_order = n + 1;
    let g = get_subgroup_generator(G_order as u128);
    let G = generate_subgroup(g);
    assert!(G.len() == G_order);
    log::debug!("Evaluation domain has {} elements", G.len());

    log::info!("Interpolating the trace (may take some time)");
    let f = Polynomial::interpolate(&G.as_slice()[..G_order - 1], &a).expect("should interpolate");
    assert_eq!(f.evaluate(&G[0]), a[0]);
    assert_eq!(f.evaluate(&G[1]), a[1]);
    assert_eq!(f.evaluate(&G[345]), a[345]);
    log::debug!("Trace polynomial has degree {}", f.degree());

    log::info!("Extending to a larger domain");
    let w = generate_generator();
    let H_order = (n + 1) * BLOWUP_FACTOR; // extend to a larger domain
    let h = get_subgroup_generator(H_order as u128);
    let H = generate_subgroup(h);
    let eval_domain = H.clone().into_iter().map(|x| w * x).collect::<Vec<_>>();
    log::debug!("Coset has {} elements", eval_domain.len());

    log::info!("Evaluating the trace polynomial on the coset");
    let f_eval = eval_domain
        .iter()
        .map(|x| f.evaluate(x))
        .collect::<Vec<_>>();

    log::info!("Merkle committing to evaluations");
    let f_merkle = MerkleTree::<Stark101PrimeFieldBackend>::build(&f_eval);
    let f_merkle_root = f_merkle.root;
    log::debug!("Merkle Root: {}", hex::encode(f_merkle_root));
    channel.append_bytes(&f_merkle_root);

    /////////////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////  PART 2  ////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////
    log::info!("Constructing the first constraint: a_0 = 1 ==> f(0) = 1");
    let numer0 = f.clone() - Polynomial::new_monomial(FE::from(1u64), 0); // f - 1
    let denom0 = Polynomial::new(&[-FE::from(1u64), FE::from(1u64)]); // X - g^0 = X - 1
    let p0 = numer0 / denom0;
    assert_eq!(p0.degree(), 1021); // 1022 - 1

    log::info!("Constructing the final constraint: a_1022 = 2338775057 ==> f(1022) = 2338775057");
    let numer1 = f.clone() - Polynomial::new_monomial(FE::from(2338775057u64), 0); // f - 2338775057
    let denom1 = Polynomial::new(&[-g.pow(1022u64), FE::from(1u64)]); // X - g^1022
    let p1 = numer1 / denom1;
    assert_eq!(p1.degree(), 1021); // 1022 - 1

    log::info!("Constructing the transition constraints: a_n = a_(n-1)^2 + a_(n-2)^2 ==> f(g^2 . x) = f(g . x)^2 + f(x)^2");
    let fg2 = f.scale(&g.pow(2u64)); // f(g^2 . x)
    let fg = f.scale(&g); // f(g. x)
    let numer2 = fg2 - (fg.clone() * fg) - (f.clone() * f); // f(g^2 . x) - f(g . x)^2 - f(x)^2
    let x_1024 = Polynomial::new_monomial(FE::one(), 1024) - Polynomial::new_monomial(FE::one(), 0); // X^1024 - 1
    let x_m_1021 = Polynomial::new(&[-g.pow(1021u64), FE::one()]); // X - g^1021
    let x_m_1022 = Polynomial::new(&[-g.pow(1022u64), FE::one()]); // X - g^1022
    let x_m_1023 = Polynomial::new(&[-g.pow(1023u64), FE::one()]); // X - g^1023
    let denom2 = x_1024 / (x_m_1021 * x_m_1022 * x_m_1023);
    let p2 = numer2 / denom2;
    assert_eq!(p2.degree(), 1023); // (1022 * 2) - (1024 - 3) = 2044 - 1021 = 1023

    log::info!("Creating the composition polynomial");
    let alpha0 = channel.sample_field_element();
    let alpha1 = channel.sample_field_element();
    let alpha2 = channel.sample_field_element();
    let cp = p0 * alpha0 + p1 * alpha1 + p2 * alpha2;

    log::info!("Evaluating over the composition polynomial");
    let cp_eval = eval_domain
        .iter()
        .map(|x| cp.evaluate(x))
        .collect::<Vec<_>>();

    log::info!("Merkle committing to the evaluations");
    let cp_merkle = MerkleTree::<Stark101PrimeFieldBackend>::build(&cp_eval);
    let cp_merkle_root = cp_merkle.root;
    log::debug!("Merkle Root: {}", hex::encode(cp_merkle_root));
    channel.append_bytes(&cp_merkle_root);

    /////////////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////  PART 3  ////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////
    log::info!("FRI committing to the composition polynomial");
    let (fri_polys, fri_domains, fri_layers, fri_merkles) =
        fri_commit(cp, eval_domain, cp_eval, cp_merkle, &mut channel);
    assert_eq!(fri_layers.len(), 11);
    assert_eq!(fri_layers.last().unwrap().len(), BLOWUP_FACTOR);
    assert_eq!(fri_polys.last().unwrap().degree(), 0);

    /////////////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////  PART 4  ////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////
    log::info!("Generating queries and decommitments to FRI");
    let num_queries = 3usize;
    let decommitments = decommit_fri(
        num_queries,
        &mut channel,
        &f_eval,
        &f_merkle,
        &fri_layers,
        &fri_merkles,
    );

    let final_state = hex::encode(channel.state());
    log::debug!("Final transcript state: {}", final_state);

    /////////////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////  PROOF   ////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////
    log::info!("Creating proof object");
    assert_eq!(decommitments.len(), num_queries);
    assert_eq!(fri_merkles.len(), 11);
    assert_eq!(decommitments[0].evals.len(), 23); // 3 (trace) + 9 * 2 + 1 (constant)
    assert_eq!(decommitments[0].paths.len(), 23); // 3 (trace) + 9 * 2 + 1 (constant)
    let proof = Stark101Proof {
        commitment: Stark101Commitment {
            trace_root: f_merkle_root,
            cp_roots: fri_merkles.iter().map(|m| m.root).collect(),
        },
        decommitments,
    };

    let path_str = proof.write_to_file();
    log::info!("Proof created at {}", path_str);
}
