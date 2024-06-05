use lambdaworks_math::{
    field::{
        element::FieldElement,
        fields::montgomery_backed_prime_fields::{IsModulus, U64PrimeField},
        traits::IsField,
    },
    unsigned_integer::element::U64,
};
#[allow(unused)]
use rand::random;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MontgomeryLucky17PrimeField;

/// Just a single limb is enough for our purposes
type Lucky17MontgomeryBackendPrimeField<T> = U64PrimeField<T>;
pub type Lucky17PrimeField = Lucky17MontgomeryBackendPrimeField<MontgomeryLucky17PrimeField>;

impl IsModulus<U64> for MontgomeryLucky17PrimeField {
    const MODULUS: U64 = U64::from_u64(17);
}

#[cfg(test)]
mod tests {
    use lambdaworks_math::{field::element::FieldElement, traits::AsBytes};
    use std::collections::{HashMap, HashSet};

    use super::*;
    const MODULUS: u64 = 17;
    type FE = FieldElement<Lucky17PrimeField>;

    #[test]
    fn test_multiplicative_subgroups() {
        let mut groups = HashMap::new();
        for g in 1..MODULUS {
            let generator = FE::from(g);
            let mut cur = generator.clone();
            let mut members = vec![cur.clone()];
            loop {
                cur = cur * generator.clone();
                if cur == FE::one() {
                    break;
                } else {
                    members.push(cur.clone())
                }
            }

            let mut set = HashSet::new();
            for m in members {
                let value64 = u64::from_str_radix(&m.to_hex(), 16).unwrap();
                set.insert(value64);
                print!("{} -> ", m.representative().to_hex());
            }
            groups.insert(g, set);
            println!("1");
        }

        println!("{:?}", groups);
    }
}
