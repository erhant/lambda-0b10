use lambdaworks_math::{
    field::{element::FieldElement, traits::IsField},
    traits::ByteConversion,
};

/// Given a number `n`, return a vector of `len` binary values in the field.
pub fn to_binary_felts<F: IsField>(n: usize, len: usize) -> Vec<FieldElement<F>>
where
    FieldElement<F>: ByteConversion,
{
    (0..len)
        .map(|b| {
            if n & (1 << b) != 0 {
                FieldElement::<F>::one()
            } else {
                FieldElement::<F>::zero()
            }
        })
        .rev()
        .collect()
}

/// Generate random evaluations for a given number of variables.
pub fn random_evals<F: IsField>(num_vars: usize) -> Vec<FieldElement<F>> {
    (0..1 << num_vars)
        .map(|_| FieldElement::<F>::from(rand::random::<u64>()))
        .collect()
}
