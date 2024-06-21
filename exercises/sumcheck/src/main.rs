use core::num;
use std::env;

use lambdaworks_math::{
    field::{element::FieldElement, fields::u64_prime_field::U64PrimeField},
    polynomial::{
        dense_multilinear_poly::DenseMultilinearPolynomial,
        sparse_multilinear_poly::SparseMultilinearPolynomial,
    },
};
use sumcheck::sumcheck::SumCheck;

const ORDER: u64 = 131;
type F = U64PrimeField<ORDER>;
type FE = FieldElement<F>;

fn get_evals() -> (Vec<(usize, FE)>, usize) {
    /// A 3-variate polynomial over a given field.
    ///
    /// Equals `2x^3 + xy + yz`, as in the example given in the PAZK book by Thaler.
    fn evaluate(xs: (usize, usize, usize)) -> FE {
        FE::from((2 * xs.0.pow(3) + xs.0 * xs.2 + xs.1 * xs.2) as u64)
    }

    const NUM_VARS: usize = 3;

    let evals = (0..(1 << NUM_VARS))
        .map(|x| {
            let xs = ((x >> 2) & 1, (x >> 1) & 1, x & 1);
            let y = evaluate(xs);
            log::debug!(
                "{}: ({} {} {}) -> {}",
                x,
                xs.0,
                xs.1,
                xs.2,
                y.representative()
            );

            (x, y)
        })
        .collect::<Vec<_>>();

    (evals, NUM_VARS)
}

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let (evals, num_vars) = get_evals();
    let sparse_poly = SparseMultilinearPolynomial::new(num_vars, evals.clone());
    assert_eq!(sparse_poly.num_vars(), 3);

    let evals_y = evals.iter().map(|(_, y)| *y).collect();
    let dense_poly = DenseMultilinearPolynomial::new(evals_y);
    assert_eq!(dense_poly.len(), 1 << num_vars);
    assert_eq!(dense_poly.num_vars(), 3);
    // let sumcheck = SumCheck::new(poly);
    // sumcheck.round(evals.iter().map(|(_, y)| *y).collect());
}
