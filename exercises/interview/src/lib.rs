#[cfg(test)]
mod tests {
    use lambdaworks_math::cyclic_group::IsGroup;
    use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::curve::BLS12381Curve;
    use lambdaworks_math::elliptic_curve::traits::IsEllipticCurve;
    use lambdaworks_math::field::element::FieldElement;
    use lambdaworks_math::field::fields::u64_goldilocks_field::Goldilocks64Field;

    /// Using lambdaworks, compute the public key associated with the secret key 0x6C616D6264617370 with the BLS12-381 elliptic curve.
    #[test]
    fn test_chal_bls12_381() {
        let gen = BLS12381Curve::generator();
        let privkey = 0x6C616D6264617370u64; // lambdasp
        let pubkey = gen.operate_with_self(privkey).to_affine();
        println!("Public key:\n({}, {})", pubkey.x(), pubkey.y());
        assert_eq!("0x67f9ffc5eaf6c19292112eadf50c11e7460e7568493141676f6ba1374badd9f6ab1f2f5e155b0e3d2f4c1de79554f80", pubkey.x().to_string());
        assert_eq!("0x18509d22f2107b667a8f75de737a4fb967f6c3e745a7c2361868515402318f006bd360b8a8763d7844381c6e510799cc", pubkey.y().to_string());
    }

    /// Give the multiplicative inverse of 2 modulo 2^{64} - 2^{32} + 1 (the so-called mini-Goldilocks prime)
    #[test]
    fn test_chal_goldilocks() {
        type Goldilocks64FieldElement = FieldElement<Goldilocks64Field>;

        let two = Goldilocks64FieldElement::from_raw(2u64);
        let two_inv = two.inv().expect("expected inverse");
        println!("Inverse of two in Goldilocks64:\n{}", two_inv);
        assert_eq!(two * two_inv, Goldilocks64FieldElement::one());
    }
}
