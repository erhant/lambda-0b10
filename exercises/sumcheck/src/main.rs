use std::env;

use lambdaworks_math::{
    field::{element::FieldElement, fields::u64_prime_field::U64PrimeField},
    polynomial::dense_multilinear_poly::DenseMultilinearPolynomial,
};
use sumcheck::sumcheck::SumCheck;

const ORDER: u64 = 131;
type F = U64PrimeField<ORDER>;
type FE = FieldElement<F>;

fn get_evals() -> Vec<(u64, FE)> {
    /// For this example, I will consider the polynomial: 10 * x_1 + x_2 * x3 + 2*x_3
    fn evaluate(x1: u64, x2: u64, x3: u64) -> FE {
        FE::from(x1.pow(4) + x2 * x3 + 2 * x3)
    }

    (0..8)
        .map(|x| {
            let (x1, x2, x3) = ((x >> 2) & 1, (x >> 1) & 1, x & 1);
            let y = evaluate(x1, x2, x3);
            log::debug!("{}: ({} {} {}) -> {}", x, x1, x2, x3, y.representative());

            (x, y)
        })
        .collect::<Vec<_>>()
}

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let evals = get_evals();
    let poly = DenseMultilinearPolynomial::new(evals.iter().map(|(_, y)| *y).collect());
    assert_eq!(poly.num_vars(), 3);
    assert_eq!(poly.len(), 8);

    let sumcheck = SumCheck::new(poly);
    sumcheck.round(evals.iter().map(|(_, y)| *y).collect());
}
