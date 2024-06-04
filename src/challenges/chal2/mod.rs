use lambdaworks_crypto::commitments::{
    kzg::{KateZaveruchaGoldberg, StructuredReferenceString},
    traits::IsCommitmentScheme,
};
use lambdaworks_math::{
    elliptic_curve::{
        short_weierstrass::{
            curves::bls12_381::{
                curve::BLS12381Curve,
                default_types::{FrConfig, FrElement},
                field_extension::BLS12381PrimeField,
                pairing::BLS12381AtePairing,
                twist::BLS12381TwistCurve,
            },
            point::ShortWeierstrassProjectivePoint,
        },
        traits::FromAffine,
    },
    field::{
        element::FieldElement, fields::montgomery_backed_prime_fields::MontgomeryBackendPrimeField,
    },
    polynomial::Polynomial,
    unsigned_integer::element::UnsignedInteger,
};

type G1Point = ShortWeierstrassProjectivePoint<BLS12381Curve>;
type G2Point = ShortWeierstrassProjectivePoint<BLS12381TwistCurve>;

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

pub fn read_srs() -> StructuredReferenceString<G1Point, G2Point> {
    let base_dir = env!("CARGO_MANIFEST_DIR");
    let srs_path = base_dir.to_owned() + "/src/challenges/chal2/srs.bin";
    StructuredReferenceString::<G1Point, G2Point>::from_file(&srs_path).unwrap()
}

pub fn solve() {
    let srs = read_srs();
    let kzg = KZG::new(srs.clone());

    let p = challenge_polynomial();

    let p_commitment: G1Point = kzg.commit(&p);

    // If you need to write a bigger number, you can use
    // If you are writing the solution in rust you shouldn't need this
    let big_number = UnsignedInteger::<6>::from_limbs([0, 0, 0, 0, 0, 2]);
    let y = Fq::new(big_number);

    // TO DO: Make your own fake proof
    let fake_proof =
        ShortWeierstrassProjectivePoint::<BLS12381Curve>::from_affine(Fq::from(0), y).unwrap();

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
}

#[cfg(test)]
mod tests {
    use lambdaworks_math::cyclic_group::IsGroup;

    use super::*;

    #[test]
    fn test_challenge() {
        solve();
    }

    #[test]
    fn test_examine_srs() {
        let srs = read_srs();
        // for p in srs.powers_main_group.iter().take(100) {
        //     println!("[{} : {} : {}]", p.x(), p.y(), p.z());
        // }

        let g = srs.powers_main_group[0].clone();
        let mut cur = g.clone();
        let mut ctr = 0;
        while !cur.is_neutral_element() {
            cur = cur.operate_with(&g);
            ctr += 1;
        }
        println!("Order of the generator: {}", ctr);
    }
}
