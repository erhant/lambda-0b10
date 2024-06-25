#[cfg(test)]
mod tests {
    use baby_snark::common::FrElement;
    use baby_snark::scs::SquareConstraintSystem;
    use baby_snark::ssp::SquareSpanProgram;
    use baby_snark::utils::i64_vec_to_field;
    use baby_snark::{setup, verify, Prover};

    /// AND gate for two inputs `a, b` and output `c` has the following constraints:
    ///
    /// - `(2a - 1)^2 = 1`: `a` is a bit
    /// - `(2b - 1)^2 = 1`: `b` is a bit
    /// - `(2c - 1)^2 = 1`: `c` is a bit
    /// - `(2a + 2b - 4c - 1)^2 = 1`: `c = a AND b`
    #[test]
    fn test_and_gate() {
        let u = vec![
            i64_vec_to_field(&[-1, 2, 0, 0]),  // -1 2 0 0  || 1
            i64_vec_to_field(&[-1, 0, 2, 0]),  // -1 0 2 0  || a
            i64_vec_to_field(&[-1, 0, 0, 2]),  // -1 0 0 2  || b
            i64_vec_to_field(&[-1, 2, 2, -4]), // -1 2 2 -4 || c
        ];

        for (a, b) in [(0, 0), (0, 1), (1, 0), (1, 1)].into_iter() {
            let c = a & b;
            println!("a: {}, b: {}, c: {}", a, b, c);
            let witness = i64_vec_to_field(&[1, a, b]);
            let public = i64_vec_to_field(&[c]);
            verify_integration(u.clone(), witness, public);
        }
    }

    /// utility to be used by multiple tests
    fn verify_integration(u: Vec<Vec<FrElement>>, witness: Vec<FrElement>, public: Vec<FrElement>) {
        let mut input = witness;
        input.extend(public.clone());

        let ssp = SquareSpanProgram::from_scs(SquareConstraintSystem::from_matrix(u, public.len()));
        let (proving_key, verifying_key) = setup(&ssp);

        let proof = Prover::prove(&input, &ssp, &proving_key).unwrap();

        let verified = verify(&verifying_key, &proof, &public);
        assert!(verified);
    }
}
