use lambdaworks_math::cyclic_group::IsGroup;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::curve::BLS12381Curve;
use lambdaworks_math::elliptic_curve::traits::IsEllipticCurve;
use lambdaworks_math::field::element::FieldElement;
use lambdaworks_math::field::fields::u64_goldilocks_field::Goldilocks64Field;

/// > Using lambdaworks, compute the public key associated with the secret key 0x6C616D6264617370 with the BLS12-381 elliptic curve.
/// > Provide link to repo
fn chal_bls12_381() {
    let gen = BLS12381Curve::generator();
    let privkey = 0x6C616D6264617370u64; // lambdasp
    let pubkey = gen.operate_with_self(privkey).to_affine();
    println!("Public key:\n({}, {})", pubkey.x(), pubkey.y());
}

type Goldilocks64FieldElement = FieldElement<Goldilocks64Field>;

/// > Give the multiplicative inverse of 2 modulo 2^{64} - 2^{32} + 1 (the so-called mini-Goldilocks prime)
fn chal_goldilocks() {
    let two = Goldilocks64FieldElement::from_raw(2u64);
    let two_inv = two.inv().expect("expected inverse");
    println!("Inverse of two in Goldilocks64:\n{}", two_inv);
    assert_eq!(two * two_inv, Goldilocks64FieldElement::one());
}

fn main() {
    chal_bls12_381();
    chal_goldilocks();
}
