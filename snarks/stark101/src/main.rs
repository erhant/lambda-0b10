#![allow(non_snake_case)]

use std::env;

use lambdaworks_crypto::{
    fiat_shamir::is_transcript::IsTranscript, merkle_tree::merkle::MerkleTree,
};
use lambdaworks_math::polynomial::Polynomial;
use log;
use stark101::{
    field::{
        generate_generator, generate_subgroup, get_subgroup_generator, Stark101PrimeFieldBackend,
        Stark101PrimeFieldElement as FE, Stark101PrimeFieldTranscript,
    },
    fri::fri_commit,
    program::fibonacci_square,
};

fn part_1() -> (
    Vec<FE>,
    FE,
    Vec<FE>,
    FE,
    Vec<FE>,
    Vec<FE>,
    Polynomial<FE>,
    Vec<FE>,
    MerkleTree<Stark101PrimeFieldBackend>,
    Stark101PrimeFieldTranscript,
) {
    log::info!("Computing trace of FibonacciSq program");
    let n = 1023;
    let a_0 = FE::from(1u64);
    let a_1 = FE::from(3141592u64);
    let a = fibonacci_square(a_0, a_1, n);
    assert_eq!(a.len(), n);
    assert_eq!(*a.last().unwrap(), FE::from(2338775057u64));

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
    let H_order = (n + 1) * 8; // 8 times larger domain
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

    log::info!("Adding root to transcript.");
    let mut channel = Stark101PrimeFieldTranscript::default();
    channel.append_bytes(&f_merkle_root);
    log::debug!("Transcript state: {:?}", channel.state());

    (a, g, G, h, H, eval_domain, f, f_eval, f_merkle, channel)
}

fn part_2() -> (
    Polynomial<FE>,
    Vec<FE>,
    MerkleTree<Stark101PrimeFieldBackend>,
    Stark101PrimeFieldTranscript,
    Vec<FE>,
) {
    let (a, g, G, h, H, eval_domain, f, f_eval, f_merkle, mut channel) = part_1();

    log::info!("Constructing the first constraint: a_0 = 1 ==> f(0) = 1");
    let numer0 = f.clone() - Polynomial::new_monomial(FE::from(1u64), 0); // f - 1
    let denom0 = Polynomial::new(&[-FE::from(1u64), FE::from(1u64)]); // X - g^0 = X - 1
    let p0 = numer0 / denom0;
    assert_eq!(p0.degree(), 1021); // 1022 - 1

    log::info!("Constructing the last constraint: a_1022 = 2338775057 ==> f(1022) = 2338775057");
    let numer1 = f.clone() - Polynomial::new_monomial(FE::from(2338775057u64), 0); // f - 2338775057
    let denom1: Polynomial<lambdaworks_math::field::element::FieldElement<lambdaworks_math::field::fields::montgomery_backed_prime_fields::MontgomeryBackendPrimeField<stark101::field::MontgomeryConfigStark101PrimeField, 1>>> = Polynomial::new(&[-g.pow(1022u64), FE::from(1u64)]); // X - g^1022
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
    assert_eq!(p2.degree(), 1023); //

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

    log::info!("Adding root to transcript.");
    channel.append_bytes(&cp_merkle_root);

    (cp, cp_eval, cp_merkle, channel, eval_domain)
}

fn part_3() {
    let (cp, cp_eval, cp_merkle, channel, eval_domain) = part_2();

    log::info!("FRI committing to the composition polynomial");
    let (fri_polys, fri_domains, fri_layers, fri_merkles) =
        fri_commit(cp, eval_domain, cp_eval, cp_merkle, channel);
    println!("FRI layers: {}", fri_polys.len());
}

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    part_3();
}
