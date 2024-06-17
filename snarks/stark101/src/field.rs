use lambdaworks_crypto::{
    fiat_shamir::default_transcript::DefaultTranscript,
    hash::sha3::Sha3Hasher,
    merkle_tree::backends::{
        field_element::FieldElementBackend,
        types::{Keccak256Backend, Sha2_256Backend},
    },
};
use lambdaworks_math::{
    field::{
        element::FieldElement,
        fields::montgomery_backed_prime_fields::{IsModulus, U64PrimeField},
        traits::IsFFTField,
    },
    unsigned_integer::element::{UnsignedInteger, U64},
};

#[derive(Clone, Debug, Hash, Copy)]
pub struct MontgomeryConfigStark101PrimeField;
impl IsModulus<U64> for MontgomeryConfigStark101PrimeField {
    const MODULUS: U64 = U64::from_hex_unchecked("c0000001");
}

pub type Stark101PrimeField = U64PrimeField<MontgomeryConfigStark101PrimeField>;

pub type Stark101PrimeFieldBackend = Sha2_256Backend<Stark101PrimeField>;

pub type Stark101PrimeFieldTranscript = DefaultTranscript<Stark101PrimeField>;

// impl IsFFTField for Stark101PrimeField {
//     const TWO_ADICITY: u64 = 30;
//     // Change this line for a new function like `from_limbs`.
//     const TWO_ADIC_PRIMITVE_ROOT_OF_UNITY: U64 = UnsignedInteger::from_hex_unchecked(
//         "5282db87529cfa3f0464519c8b0fa5ad187148e11a61616070024f42f8ef94", // TODO: !!!
//     );

//     fn field_name() -> &'static str {
//         "stark101"
//     }
// }

pub type Stark101PrimeFieldElement = FieldElement<Stark101PrimeField>;

/// Returns a generator for a subgroup of the given order a power of two.
///
/// 1. Generate a random element `r` in the field.
/// 2. Compute `g = r^(order_u128 / order)` (co-factor clearing).
/// 3. If `g^order == 1` and `g^(order >> 1) != 1`, return `g`.
/// The second check ensures that the order of `g` is exactly `order`, and not that of
/// some smaller sub-group.
///
/// ## Panics
///
/// If the order does not divide the multiplicative field order.
pub fn get_subgroup_generator(order: u128) -> Stark101PrimeFieldElement {
    let order_u128: u128 = 3u128 * (1u128 << 30u128);
    assert!(
        order_u128 % order == 0,
        "order must divide the multiplicative field order"
    );

    loop {
        let r = Stark101PrimeFieldElement::from(rand::random::<u64>());

        // co-factor clearing
        let g = r.pow(order_u128 / order);

        if g.pow(order) == Stark101PrimeFieldElement::one()
            && g.pow(order >> 1) != Stark101PrimeFieldElement::one()
        {
            return g;
        }
    }
}

/// Given a generator `g`, generates the group that it belongs to.
pub fn generate_subgroup(g: Stark101PrimeFieldElement) -> Vec<Stark101PrimeFieldElement> {
    let mut subgroup = Vec::new();
    subgroup.push(Stark101PrimeFieldElement::one());

    let mut next = g.clone();
    while next != Stark101PrimeFieldElement::one() {
        subgroup.push(next);
        next = next * g.clone();
    }

    subgroup
}

pub fn generate_generator() -> Stark101PrimeFieldElement {
    loop {
        let r = Stark101PrimeFieldElement::from(rand::random::<u64>());

        if r.pow(3u128) != Stark101PrimeFieldElement::one()
            && r.pow(1u128 << 30u128) != Stark101PrimeFieldElement::one()
        {
            return r;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Stark101PrimeFieldElement as FE;
    use super::*;

    #[test]
    fn test_field_mul() {
        let a = FE::from(4u64);
        let b = FE::from(2u64);
        let c = FE::from(8u64);
        assert_eq!(a * b, c);
    }

    #[test]
    fn test_subgroup_1024() {
        let order = 1024u128;
        let g = get_subgroup_generator(order);
        let subgroup = generate_subgroup(g);
        assert_eq!(subgroup.len(), order as usize);
    }

    #[test]
    fn test_subgroup_8192() {
        let order = 8192u128;
        let g = get_subgroup_generator(order);
        let subgroup = generate_subgroup(g);
        assert_eq!(subgroup.len(), order as usize);
    }

    #[test]
    fn test_subgroup_1() {
        let subgroup = generate_subgroup(FE::one());
        assert_eq!(subgroup.len(), 1);
    }

    #[test]
    fn test_generator() {
        let order_u128: u128 = 3u128 * (1u128 << 30u128);
        let g = generate_generator();
        assert_eq!(g.pow(order_u128), FE::one());
    }
}
