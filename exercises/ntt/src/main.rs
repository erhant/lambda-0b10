use std::env;

use lambdaworks_math::{
    field::{element::FieldElement, test_fields::u64_test_field::U64Field},
    polynomial::Polynomial,
};
use ntt::NTT;

// define a prime field of order 17
type F = U64Field<17>;
type FE = FieldElement<F>;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // 13 is a primitive 4-th root of unity
    // and 4 is the max we can have here because 17 - 1 = 2^4
    let w = FE::from(13u64);

    let coeffs = (0..4).map(|i| FE::from(i as u64)).collect::<Vec<_>>();
    let poly = Polynomial::new(&coeffs);
    let ntt = NTT::new(w.clone(), 4);

    log::info!("Twiddle factors:");
    for (i, w_i) in ntt.twiddles.iter().enumerate() {
        log::info!("w^{} = {}", i, w_i.representative());
    }
    let evals = ntt.forward(&coeffs);

    // confirm evaluations
    for (i, e) in evals.iter().enumerate() {
        let y = poly.evaluate(&ntt.twiddles[i]);
        assert_eq!(e, &y);
        log::debug!("A_{} = {}", i, e.representative());
    }

    // inverse
    // TODO: !!!
}
