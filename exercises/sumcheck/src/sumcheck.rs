use lambdaworks_crypto::fiat_shamir::{
    default_transcript::DefaultTranscript, is_transcript::IsTranscript,
};
use lambdaworks_math::{
    field::{element::FieldElement, traits::IsField},
    polynomial::{dense_multilinear_poly::DenseMultilinearPolynomial, Polynomial},
    traits::{AsBytes, ByteConversion},
};

/// This struct will apply the SumCheck protocol using a given polynomial.
///
/// The sum is over all inputs:

///
/// Every round, a random `r_i` will be given and a multivariate will be provided:
///
/// - `g(x_1, x_2, ..., x_{n-1}, x_n)`
/// - `g(r_1, x_2, ..., x_{n-1}, x_n)`
/// - `g(r_1, r_2, ..., x_{n-1}, x_n)`
/// - ...
/// - `g(r_1, r_2, ..., r_{n-1}, x_n)`
/// - `g(r_1, r_2, ..., r_{n-1}, r_n)`
///
pub struct SumCheck<F: IsField>
where
    <F as IsField>::BaseType: Send + Sync,
{
    pub poly: DenseMultilinearPolynomial<F>,
    pub sum: FieldElement<F>,
    pub channel: DefaultTranscript<F>,
}

impl<F: IsField> SumCheck<F>
where
    <F as IsField>::BaseType: Send + Sync + AsBytes,
    FieldElement<F>: ByteConversion,
{
    pub fn new(poly: DenseMultilinearPolynomial<F>) -> Self {
        let sum = poly
            .evals()
            .iter()
            .fold(FieldElement::<F>::zero(), |acc, y| acc + y);

        log::info!("Sum: {:?}", sum);
        Self {
            poly,
            sum,
            channel: DefaultTranscript::default(),
        }
    }

    /// Simulate a verifier query
    pub fn new_query(&mut self) -> FieldElement<F> {
        self.channel.sample_field_element()
    }

    /// The first round is as follows:
    ///
    /// 1. Prover sends the verifier value `c_1`, which is claimed to equal the sum `H`.
    /// 2. Prover sends `g_1(x_1)`, a univariate polynomial of degree less than `d`, such that
    /// `c_0 = g_1(0) + g_1(1)`.
    pub fn first_round(&self, evals: Vec<FieldElement<F>>) {}

    /// The intermediate rounds are as follows:
    ///
    /// 1. Prover sends the verifier a univariate polynomial `g_j(X_j)` claimed to equal
    /// the sum over the boolean hypercube subset for `g(r_1, ..., r_{j-1}, X_j, X_{j+1}, ..., X_n)`.
    /// In other words, fix all values of `r` so far,
    pub fn final_round(&self, evals: Vec<FieldElement<F>>) {}

    pub fn intermediate_round(&self, evals: Vec<FieldElement<F>>) {
        let left_evals = evals.iter().take(evals.len() >> 1);
        let left_sum = left_evals.fold(FieldElement::<F>::zero(), |acc, y| acc + y);

        let right_evals = evals.iter().skip(evals.len() >> 1);
        let right_sum = right_evals.fold(FieldElement::<F>::zero(), |acc, y| acc + y);

        let sum = left_sum.clone() + right_sum.clone();

        // g(x) = (1 - x) * left_sum + (x) * right_sum = left_sum + x * (right_sum - left_sum)
        // g(0) = left_sum
        // g(1) = right_sum
        let poly = Polynomial::new(&[left_sum.clone(), right_sum - left_sum]);
        assert_eq!(
            poly.evaluate(&FieldElement::<F>::zero()) + poly.evaluate(&FieldElement::<F>::one()),
            sum
        );
    }
}
