use std::env;

use lambdaworks_crypto::merkle_tree::{backends::types::Sha2_256Backend, merkle::MerkleTree};
use lambdaworks_math::polynomial::Polynomial;
use log;
use stark101::{
    field::{generate_subgroup, get_subgroup_generator, Stark101PrimeFieldElement as FE},
    program::fibonacci_square,
};

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    log::info!("Computing trace of FibonacciSq program");
    let n = 1023;
    let a_0 = FE::from(1u64);
    let a_1 = FE::from(3141592u64);
    let a = fibonacci_square(a_0, a_1, n);

    log::info!("Creating evaluation domain");
    let order_eval = n + 1;
    let g_eval = get_subgroup_generator(order_eval as u128);
    let dom_eval = generate_subgroup(g_eval);
    assert!(dom_eval.len() == order_eval);

    log::info!("Extending to a larger domain");
    let g_ext = FE::from(3); // generator of the field
    let order_eval_ext = (n + 1) * 8; // 8 times larger domain
    let g_eval_ext = g_ext.pow(order_eval as u128 / order_eval_ext as u128);
    let dom_eval_ext = generate_subgroup(g_eval_ext);
    let coset = dom_eval_ext
        .into_iter()
        .map(|x| x * g_ext)
        .collect::<Vec<_>>();

    log::info!("Interpolating the trace (may take some time)");
    let trace_poly = Polynomial::interpolate(&coset.as_slice()[..coset.len() - 1], &a)
        .expect("should interpolate");
    assert_eq!(trace_poly.evaluate(&dom_eval[0]), a[0]);
    assert_eq!(trace_poly.evaluate(&dom_eval[1]), a[1]);
    assert_eq!(trace_poly.evaluate(&dom_eval[345]), a[345]);

    log::info!("Merkle committing to coset");
    // let tree = MerkleTree::build(&trace_poly);
}
