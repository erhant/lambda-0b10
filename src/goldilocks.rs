use lambdaworks_math::field::element::FieldElement;
use lambdaworks_math::field::fields::u64_goldilocks_field::Goldilocks64Field;

type Goldilocks64FieldElement = FieldElement<Goldilocks64Field>;

/// > Give the multiplicative inverse of 2 modulo 2^{64} - 2^{32} + 1 (the so-called mini-Goldilocks prime)
pub fn chal_goldilocks() {
    let two = Goldilocks64FieldElement::from_raw(2u64);
    let two_inv = two.inv().expect("expected inverse");
    println!("Inverse of two in Goldilocks64: {:?}\n", two_inv);
    assert_eq!(two * two_inv, Goldilocks64FieldElement::one());
}
