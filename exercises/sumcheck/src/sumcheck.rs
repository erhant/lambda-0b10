#![allow(non_snake_case)]

use lambdaworks_crypto::fiat_shamir::{
    default_transcript::DefaultTranscript, is_transcript::IsTranscript,
};
use lambdaworks_math::{
    field::{element::FieldElement as FE, traits::IsField},
    polynomial::{dense_multilinear_poly::DenseMultilinearPolynomial, Polynomial},
    traits::{AsBytes, ByteConversion},
};

use crate::utils::to_binary_felts;

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

        // use the polynomial evaluations to initialize the transcript
        let init_bytes = poly
            .evals()
            .iter()
            .flat_map(|y| y.to_bytes_be())
            .collect::<Vec<_>>();

        Self {
            g: poly,
            sum,
            transcript: DefaultTranscript::new(&init_bytes),
        }
    }

    // Run the initialization round and return the claimed sum check value
    pub fn prove(&mut self) {
        let mut round = 1usize;

        log::info!("Round: {}", round);
        let (one, zero) = (FE::<F>::one(), FE::<F>::zero());

        // first polynomial has no random variables
        let mut last_poly = self.interpolate(&vec![]);
        let mut last_poly_name = "g_1".to_string();

        // first check is made against the sum itself
        let mut check = self.sum.clone();
        let mut check_name = "C_1".to_string();

        // there is a round for each random variable
        let mut random_vars = vec![];
        while random_vars.len() <= self.g.num_vars() {
            // verifier checks the sum & degree
            log::info!(
                "Checking {} = {}(0) + {}(1)",
                check_name,
                last_poly_name,
                last_poly_name
            );
            assert_eq!(check, last_poly.evaluate(&zero) + last_poly.evaluate(&one));
            assert!(last_poly.degree() <= 1, "degree should be at most 1");

            // verifier adds a random query
            let r = self.transcript.sample_field_element();

            log::debug!("Evaluating {} at r_{}", last_poly_name, round);
            check = last_poly.evaluate(&r); // check is updated to g_{j-1}(r_{j-1})
            check_name = format!("g_{}(r_{})", round, round);
            random_vars.push(r); // random query is added to history

            // when a round is done, and the random variable is added,
            // the number of polynomials and random variables should match
            assert_eq!(random_vars.len(), round);
            round += 1;

            if random_vars.len() == self.g.num_vars() {
                // we have all random variables we need, we can make the final check
                log::info!(
                    "Checking g_{}(r_{}) = g(r_1, r_2, ..., r_n)",
                    round - 1,
                    round - 1
                );
                let final_sum = self.g.evaluate(random_vars).unwrap(); // check is updated to g(r_1, r_2, ..., r_n)
                assert_eq!(final_sum, check);
                break;
            } else {
                // interpolation is made for the next fixed variable
                log::info!("Round: {}", round);
                last_poly_name = format!("g_{}", round);
                log::debug!("Interpolating g_{} for variable X_{}", round, round);
                last_poly = self.interpolate(&random_vars);
            }
        }

        log::info!("Sumcheck completed successfully!");
        // TODO: export proof
    }

    /// Given a list of random variables, interpolate the polynomial at the next index.
    ///
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
                assert!(poly.degree() <= 1, "degree must be at most 1");

                poly
            })
            .fold(Polynomial::zero(), |acc, poly| acc + poly)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::random_evals;
    use lambdaworks_math::field::fields::u64_prime_field::U64PrimeField;

    use super::*;

    const ORDER: u64 = 17;
    type F = U64PrimeField<ORDER>;

    fn run_test(n: usize) {
        let evals = random_evals::<F>(n);
        assert_eq!(evals.len(), 1 << n);

        let poly = DenseMultilinearPolynomial::new(evals);
        assert_eq!(poly.num_vars(), n);

        let mut sumcheck = SumCheck::new(poly);
        sumcheck.prove();
    }

    #[test]
    fn test_2_vars() {
        run_test(2);
    }

    #[test]
    fn test_3_vars() {
        run_test(3);
    }

    #[test]
    fn test_7_vars() {
        run_test(7);
    }
}
