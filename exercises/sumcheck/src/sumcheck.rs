use lambdaworks_math::{
    field::{element::FieldElement, traits::IsField},
    polynomial::{dense_multilinear_poly::DenseMultilinearPolynomial, Polynomial},
};

pub struct SumCheck<F: IsField>
where
    <F as IsField>::BaseType: Send + Sync,
{
    pub poly: DenseMultilinearPolynomial<F>,
    pub sum: FieldElement<F>,
}

impl<F: IsField> SumCheck<F>
where
    <F as IsField>::BaseType: Send + Sync,
{
    pub fn new(poly: DenseMultilinearPolynomial<F>) -> Self {
        let sum = poly
            .evals()
            .iter()
            .fold(FieldElement::<F>::zero(), |acc, y| acc + y);

        log::info!("Sum: {:?}", sum);
        Self { poly, sum }
    }

    pub fn round(&self, evals: Vec<FieldElement<F>>) {
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
