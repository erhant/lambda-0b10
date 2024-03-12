use lambdaworks_math::cyclic_group::IsGroup;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::curve::BLS12381Curve;
use lambdaworks_math::elliptic_curve::traits::IsEllipticCurve;

/// > Using lambdaworks, compute the public key associated with the secret key 0x6C616D6264617370 with the BLS12-381 elliptic curve.
/// > Provide link to repo
pub fn chal_pubkey() {
    let gen = BLS12381Curve::generator();
    let privkey = 0x6C616D6264617370u64;
    let pubkey = gen.operate_with_self(privkey).to_affine();
    println!("Public key: ({}, {})\n", pubkey.x(), pubkey.y());
}
