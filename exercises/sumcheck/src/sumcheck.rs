#![allow(non_snake_case)]

use std::fmt::Display;

use lambdaworks_crypto::fiat_shamir::{
    default_transcript::DefaultTranscript, is_transcript::IsTranscript,
};
use lambdaworks_math::{
    field::{element::FieldElement as FE, traits::IsField},
    polynomial::{dense_multilinear_poly::DenseMultilinearPolynomial, Polynomial},
    traits::{AsBytes, ByteConversion},
};

/// Given a number `n`, return a vector of `len` binary values in the field.
pub fn to_binary_felts<F: IsField>(n: usize, len: usize) -> Vec<FE<F>>
where
    FE<F>: ByteConversion,
{
    (0..len)
        .map(|b| {
            if n & (1 << b) != 0 {
                FE::<F>::one()
            } else {
                FE::<F>::zero()
            }
        })
        .rev()
        .collect()
}

/// This struct will apply the SumCheck protocol prover using a given polynomial, along with a
/// verifier instantiated using the transcript (Fiat-Shamir transform).
pub struct SumCheck<F: IsField>
where
    <F as IsField>::BaseType: Send + Sync,
{
    /// Multilinear polynomial to be Sumchecked.
    g: DenseMultilinearPolynomial<F>,
    /// Sum of the polynomial evaluations.
    sum: FE<F>,
    /// Transcript for Fiat-Shamir transform of the verifier.
    pub transcript: DefaultTranscript<F>,
}

impl<F: IsField> SumCheck<F>
where
    <F as IsField>::BaseType: Send + Sync + AsBytes,
    FE<F>: ByteConversion,
{
    pub fn new(poly: DenseMultilinearPolynomial<F>) -> Self {
        // sum-reduce the polynomial over all its evaluations
        let sum = poly.evals().iter().fold(FE::<F>::zero(), |acc, y| acc + y);

        log::info!(
            "Sumcheck starting for {}-variate multilinear polynomial",
            poly.num_vars()
        );

        Self {
            g: poly,
            sum,
            transcript: DefaultTranscript::new("sumcheck".as_bytes()),
        }
    }

    // Run the initialization round and return the claimed sum check value
    pub fn prove(&mut self) {
        let mut round = 1usize;

        log::info!("Starting round: {}", round);
        let mut rs = vec![]; // first interpolation has no random variables
        let mut polys = vec![self.interpolate(&rs)]; // rs is empty here
        let mut check = self.sum.clone(); // first check is made against the sum itself

        let (one, zero) = (FE::<F>::one(), FE::<F>::zero());

        while rs.len() <= self.g.num_vars() {
            let last_poly = polys.last().unwrap();
            // verifier checks the sum & degree
            log::info!("Checking round: {}", round);
            assert_eq!(check, last_poly.evaluate(&zero) + last_poly.evaluate(&one));
            assert_eq!(last_poly.degree(), 1);

            // verifier adds a random query
            let r = self.transcript.sample_field_element();
            log::debug!("Evaluating g_{}(r_{})", round, round);
            check = last_poly.evaluate(&r); // check is updated to g_{j-1}(r_{j-1})
            rs.push(r); // random query is added to history

            // when a round is done, and the random variable is added,
            // the number of polynomials and random variables should match
            assert_eq!(polys.len(), round);
            assert_eq!(rs.len(), round);
            round += 1;

            if rs.len() == self.g.num_vars() {
                log::info!("Final check");
                let final_sum = self.g.evaluate(rs).unwrap(); // check is updated to g(r_1, r_2, ..., r_n)
                assert_eq!(final_sum, check);
                break;
            } else {
                // interpolation is made for the next fixed variable
                log::info!("Commencing Round: {}", round);
                polys.push(self.interpolate(&rs));
            }
        }

        log::info!("Sumcheck completed successfully!");
    }

    /// Given a list of evaluations, interpolate the polynomial at the next index.
    /// For instance, for `g(x_1, x_2, ..., x_n)` with random variables `r_1, r_2, ..., r_{k-1}`
    /// we interpolate a polynomial `g_k(X_k) = sum(g(r_1, r_2, ..., r_{k-1}, X_k, x_{k+1}, ..., x_n))`
    /// where `x_{k+1}, ..., x_n` are evaluated over 0s and 1s.
    ///
    /// As a concrete example, consider `g(x_1, x_2, x_3)` with random variable `r_1`. This function will interpolate
    /// a univariate polynomial `g_2(X_2) = g(r_1, X_2, 0) + g(r_1, X_2, 1)`.
    ///
    /// There are probably clever ways to do this, but here we are working with MLE's so all terms have degree 1. With that, we only need 2 evaluations
    /// for every term in the sum to interpolate the polynomial for a term, and we can sum all polys.
    pub fn interpolate(&self, rs: &Vec<FE<F>>) -> Polynomial<FE<F>> {
        // we need (0, 1) pair for each input besides the fixed term & random variables.
        let num_vars = self.g.num_vars() - rs.len() - 1;
        log::debug!("Interpolating for X_{}", rs.len() + 1);

        // iterate over all combinations of 0s and 1s for the remaining variables
        // interpolate the polynomial for each setting, and sum them all
        (0..1 << num_vars)
            .map(|i| {
                // convert `i` to 0s and 1s
                let xs = to_binary_felts(i, num_vars);

                // to interpolate the currently fixed setting, e.g. g'(X) = g(rs..., X, xs...), we first need to evaluate at some points
                // just 0 and 1 is enough because all terms are degree 1 in each variable (due to MLE)
                let eval_xs = vec![FE::<F>::zero(), FE::<F>::one()];

                let eval_ys = eval_xs
                    .clone()
                    .into_iter()
                    .map(|X| {
                        // prepare parameters
                        let mut inputs = rs.clone();
                        inputs.push(X);
                        inputs.extend(xs.clone());

                        // evaluate the polynomial
                        self.g.evaluate(inputs).unwrap()
                    })
                    .collect::<Vec<_>>();

                // interpolate the univariate polynomial using these evaluations
                let poly = Polynomial::interpolate(&eval_xs, &eval_ys).unwrap();
                assert_eq!(poly.degree(), 1, "expected 1 degree");

                poly
            })
            .fold(Polynomial::zero(), |acc, poly| acc + poly)
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use lambdaworks_math::field::element::FieldElement as FE;
    use lambdaworks_math::field::fields::u64_prime_field::U64PrimeField;

    use super::*;

    type F = U64PrimeField<17>;

    #[test]
    fn test_2_vars() {
        env::set_var("RUST_LOG", "DEBUG");
        let _ = env_logger::try_init();

        const N: usize = 2; // number of variables

        // consider a 2-variate poly x_1*x_2 + 3*x_1^2 + x_2^4
        // it evaluates to 0, 3, 1, 5 for 00, 01, 10, 11
        fn g(xs: Vec<FE<F>>) -> FE<F> {
            xs[0].clone() * xs[1].clone() + FE::<F>::from(3) * xs[0].pow(2u64) + xs[1].pow(4u64)
        }

        // MLE of the polynomial above, redundant terms written for clarity
        fn g_mle(xs: Vec<FE<F>>) -> FE<F> {
            let one = FE::<F>::one();
            vec![
                // (00) -> 0
                FE::<F>::from(0) * (one.clone() - xs[0].clone()) * (one.clone() - xs[1].clone()),
                // (10) -> 3
                FE::<F>::from(3) * xs[0].clone() * (one.clone() - xs[1].clone()),
                // (01) -> 1
                FE::<F>::from(1) * (one.clone() - xs[0].clone()) * xs[1].clone(),
                // (11) -> 5
                FE::<F>::from(5) * xs[0].clone() * xs[1].clone(),
            ]
            .iter()
            .fold(FE::<F>::zero(), |acc, y| acc + y)
        }

        // evaluate over boolean hypercube
        let mut evals = vec![];
        for i in 0..(1 << N) {
            let xs = to_binary_felts(i, N);
            let y = g(xs.clone());
            let y_mle = g_mle(xs.clone());
            assert_eq!(y, y_mle);
            evals.push(y);
        }

        let poly = DenseMultilinearPolynomial::new(evals);
        assert_eq!(poly.num_vars(), N);

        let mut sumcheck = SumCheck::new(poly);
        sumcheck.prove();
    }

    #[test]
    fn test_3_vars() {
        env::set_var("RUST_LOG", "DEBUG");
        let _ = env_logger::try_init();

        const N: usize = 3; // number of variables

        let evals = vec![
            FE::<F>::from(10),
            FE::<F>::from(101),
            FE::<F>::from(220),
            FE::<F>::from(33),
            FE::<F>::from(456),
            FE::<F>::from(567),
            FE::<F>::from(6),
            FE::<F>::from(7),
        ];
        assert_eq!(evals.len(), 1 << N);

        let poly = DenseMultilinearPolynomial::new(evals);
        assert_eq!(poly.num_vars(), N);

        let mut sumcheck = SumCheck::new(poly);
        sumcheck.prove();
    }
}
