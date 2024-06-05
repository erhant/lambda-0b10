use lambdaworks_crypto::commitments::{
    kzg::{KateZaveruchaGoldberg, StructuredReferenceString},
    traits::IsCommitmentScheme,
};
use lambdaworks_math::{
    cyclic_group::IsGroup,
    elliptic_curve::short_weierstrass::{
        curves::bls12_381::{
            curve::BLS12381Curve,
            default_types::{FrConfig, FrElement},
            field_extension::BLS12381PrimeField,
            pairing::BLS12381AtePairing,
            twist::BLS12381TwistCurve,
        },
        point::ShortWeierstrassProjectivePoint,
    },
    field::{
        element::FieldElement, fields::montgomery_backed_prime_fields::MontgomeryBackendPrimeField,
    },
    polynomial::Polynomial,
    unsigned_integer::element::UnsignedInteger,
};
use rand::random;

type G1Point = ShortWeierstrassProjectivePoint<BLS12381Curve>;
type G2Point = ShortWeierstrassProjectivePoint<BLS12381TwistCurve>;

#[allow(clippy::upper_case_acronyms)]
type KZG = KateZaveruchaGoldberg<MontgomeryBackendPrimeField<FrConfig, 4>, BLS12381AtePairing>;
pub type Fq = FieldElement<BLS12381PrimeField>;

/// This function creates the polynomial as given in the problem. We don't touch it.
fn challenge_polynomial() -> Polynomial<FrElement> {
    Polynomial::<FrElement>::new(&[
        FieldElement::from(69),
        FieldElement::from(78),
        FieldElement::from(32),
        FieldElement::from(65),
        FieldElement::from(82),
        FieldElement::from(71),
        FieldElement::from(69),
        FieldElement::from(78),
        FieldElement::from(84),
        FieldElement::from(73),
        FieldElement::from(78),
        FieldElement::from(65),
        FieldElement::from(32),
        FieldElement::from(78),
        FieldElement::from(65),
        FieldElement::from(67),
        FieldElement::from(73),
        FieldElement::from(32),
        FieldElement::from(84),
        FieldElement::from(73),
        FieldElement::from(69),
        FieldElement::from(82),
        FieldElement::from(65),
    ])
}

/// Utility to read the SRS from the file.
fn read_srs() -> StructuredReferenceString<G1Point, G2Point> {
    let base_dir = env!("CARGO_MANIFEST_DIR");
    let srs_path = base_dir.to_owned() + "/srs.bin";
    StructuredReferenceString::<G1Point, G2Point>::from_file(&srs_path).unwrap()
}

/// Finds the toxic waste given the generators and their scalar multiples.
///
/// Note that we use $g$ and $sg$ here, but any $s^ig$ and $s^{i+1}g$ would work too.
fn find_toxic_waste(g1: &G1Point, sg1: &G1Point, g2: &G2Point, sg2: &G2Point) -> FrElement {
    // infinite loop, but we are SURE about this
    loop {
        // find a primitive root of unity
        let s = find_primitive_root();

        // see if it matches the secret
        if g1.operate_with_self(s.representative()) == *sg1
            && g2.operate_with_self(s.representative()) == *sg2
        {
            return s;
        }
    }
}
/// Finds a primitive 64th root of unity in the scalar field of the BLS12-381 curve.
fn find_primitive_root() -> FrElement {
    loop {
        // random element within the scalar field of order r
        let g = FrElement::from(random::<u64>());

        // (r - 1) / 64
        let cofactor: UnsignedInteger<6> = UnsignedInteger::from_hex_unchecked(
            "0x01CFB69D4CA675F520CCE76020268760154EF6900BFFF96FFBFFFFFFFC000000",
        );

        // obtain root of unity via cofactor clearing
        let root = g.pow(cofactor);
        debug_assert_eq!(root.pow(64u64), FrElement::one());

        // check that its indeed primitive
        if root.pow(32u64) != FrElement::one() {
            return root;
        }
    }
}

fn main() {
    let srs = read_srs();
    let kzg = KZG::new(srs.clone());
    let p = challenge_polynomial();

    // the commitment is just a point on the curve, computed via MSM
    let p_commitment: G1Point = kzg.commit(&p);

    // find the toxic waste
    let (g1, sg1) = (&srs.powers_main_group[0], &srs.powers_main_group[1]);
    let (g2, sg2) = (
        &srs.powers_secondary_group[0],
        &srs.powers_secondary_group[1],
    );
    let s = find_toxic_waste(g1, sg1, g2, sg2);

    // compute q(s) via the fake proof method = (P(s) - 3) / (s - 1)
    let q_s = (p.evaluate(&s) - FrElement::from(3))
        * (s - FrElement::one()).inv().expect("should invert");

    // find the commitment as g * q(s)
    // normally we would do MSM for this using SRS, but we know the toxic waste :)
    let q_commitment = g1.operate_with_self(q_s.representative());

    let fake_proof = q_commitment;
    println!("Fake proof for submission:");
    println!("{:?}", &fake_proof.to_affine().x().to_string());
    println!("{:?}", &fake_proof.to_affine().y().to_string());

    // verify the proof that P(1) = 3
    assert!(kzg.verify(
        &FrElement::from(1),
        &FrElement::from(3),
        &p_commitment,
        &fake_proof
    ));

    println!("Faked succesfully!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examine_srs() {
        let srs = read_srs();

        // find the repeating point
        let mut ctr = 1;
        let g = srs.powers_main_group[0].clone();
        for p in srs
            .powers_main_group
            .iter()
            .skip(1)
            .take_while(|p| *p != &g)
        {
            println!("{}\t[{} : {} : {}]", ctr, p.x(), p.y(), p.z());
            ctr += 1;
        }

        println!("Repeat found at: {}", ctr); // ctr turns out to be 64
    }

    #[test]
    fn test_primitive_root() {
        let root = find_primitive_root();
        assert_eq!(root.pow(64u64), FrElement::one());
        println!("Primitive 64th root of unity: {}", root);
    }

    #[test]
    fn test_toxic_waste() {
        let srs = read_srs();
        let (g1, sg1) = (&srs.powers_main_group[0], &srs.powers_main_group[1]);
        let (g2, sg2) = (
            &srs.powers_secondary_group[0],
            &srs.powers_secondary_group[1],
        );

        let s = find_toxic_waste(g1, sg1, g2, sg2);

        println!("Toxic waste: {}", s);
        // 0xe4840ac57f86f5e293b1d67bc8de5d9a12a70a615d0b8e4d2fc5e69ac5db47f
    }
}
