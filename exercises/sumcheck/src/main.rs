use std::env;

use lambdaworks_math::field::element::FieldElement;
use lambdaworks_math::field::fields::u64_prime_field::U64PrimeField;
use lambdaworks_math::polynomial::dense_multilinear_poly::DenseMultilinearPolynomial;
use sumcheck::sumcheck::SumCheck;
use sumcheck::utils::to_binary_felts;

type F = U64PrimeField<17>;
type FE = FieldElement<F>;

// A 3-variate poly x_1*x_2*x_3 + 2*x_2 + 3*x_1^2 + x_2^4*x_3 + 5*x_1*x_2 + 2*x_3
fn g(xs: Vec<FE>) -> FE {
    vec![
        // x_1*x_2*x_3
        xs[0].clone() * xs[1].clone() * xs[2].clone(),
        // 2*x_2
        FE::from(2) * xs[1].clone(),
        // 3*x_1^2
        FE::from(3) * xs[0].clone() * xs[0].clone(),
        // x_2^4*x_3
        xs[1].pow(4_u64) * xs[2].clone(),
        // 5*x_1*x_2
        FE::from(5) * xs[0].clone() * xs[1].clone(),
        // 2*x_3
        FE::from(2) * xs[2].clone(),
    ]
    .iter()
    .fold(FE::zero(), |acc, y| acc + y)
}

// MLE of the polynomial above, redundant terms written for clarity
// I handwrite this to show in clear how MLE works
fn g_mle(xs: Vec<FE>) -> FE {
    #[inline(always)]
    fn _1(x: &FE) -> FE {
        x.clone()
    }
    #[inline(always)]
    fn _0(x: &FE) -> FE {
        FE::one() - x.clone()
    }

    vec![
        FE::from(00) * _0(&xs[0]) * _0(&xs[1]) * _0(&xs[2]), // (000): -> 0
        FE::from(02) * _0(&xs[0]) * _0(&xs[1]) * _1(&xs[2]), // (001): -> 2
        FE::from(02) * _0(&xs[0]) * _1(&xs[1]) * _0(&xs[2]), // (010): -> 2
        FE::from(05) * _0(&xs[0]) * _1(&xs[1]) * _1(&xs[2]), // (011): -> 5
        FE::from(03) * _1(&xs[0]) * _0(&xs[1]) * _0(&xs[2]), // (100): -> 3
        FE::from(05) * _1(&xs[0]) * _0(&xs[1]) * _1(&xs[2]), // (101): -> 5
        FE::from(10) * _1(&xs[0]) * _1(&xs[1]) * _0(&xs[2]), // (110): -> 10
        FE::from(14) * _1(&xs[0]) * _1(&xs[1]) * _1(&xs[2]), // (111): -> 14
    ]
    .iter()
    .fold(FE::zero(), |acc, y| acc + y)
}

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    const N: usize = 3; // number of variables

    // evaluate over boolean hypercube
    let mut evals = vec![];
    for i in 0..(1 << N) {
        let xs = to_binary_felts(i, N);
        let y = g(xs.clone());
        assert_eq!(y, g_mle(xs.clone()), "g_mle and g differ");
        log::debug!(
            "{} ({}{}{}): -> {}",
            i,
            xs[0].representative(),
            xs[1].representative(),
            xs[2].representative(),
            y.representative()
        );
        evals.push(y);
    }

    // create a dense multilienar poly from the evaluations
    let poly = DenseMultilinearPolynomial::new(evals);
    assert_eq!(poly.len(), 1 << N);
    assert_eq!(poly.num_vars(), N);

    // create sumcheck proof
    let mut sumcheck = SumCheck::new(poly);
    sumcheck.prove()

    // verify proof
    // TODO: !!!
}
