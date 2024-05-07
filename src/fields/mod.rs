use lambdaworks_math::{
    field::fields::montgomery_backed_prime_fields::{IsModulus, U64PrimeField},
    unsigned_integer::element::U64,
};

#[allow(unused)]
use rand::random;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MontgomeryLucky13PrimeField;

/// Just a single limb is enough for our purposes
type Lucky13MontgomeryBackendPrimeField<T> = U64PrimeField<T>;
pub type Lucky13PrimeField = Lucky13MontgomeryBackendPrimeField<MontgomeryLucky13PrimeField>;

impl IsModulus<U64> for MontgomeryLucky13PrimeField {
    const MODULUS: U64 = U64::from_u64(13);
}

#[cfg(test)]
mod tests {
    use lambdaworks_math::field::element::FieldElement;

    use super::*;
    const MODULUS: u64 = 13;
    type F = Lucky13PrimeField;
    type FE = FieldElement<F>;

    #[test]
    fn test_addition() {
        for a in 0..MODULUS {
            for b in 0..MODULUS {
                let expected = (a + b) % MODULUS;
                let a = FE::from(a);
                let b = FE::from(b);
                let expected = FE::from(expected);
                assert_eq!(a + b, expected);
            }
        }
    }

    // #[test]
    // fn test_random() {
    //     let x = random::<u64>();
    //     println!("{}", FE::from(x));
    // }
}
