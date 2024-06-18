use lambdaworks_math::field::{element::FieldElement, traits::IsField};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stark101Commitment {
    pub trace_root: [u8; 32],
    pub cp_roots: Vec<[u8; 32]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stark101Decommitment<F: IsField> {
    pub evals: Vec<FieldElement<F>>,
    pub paths: Vec<Vec<[u8; 32]>>,
}

/// A Stark101 proof, based on [this video](https://www.youtube.com/watch?v=CxP28qM4tAc) at 11:15.
///
/// - `commitment`: the commitment to the proof, which includes the trace root and the composition polynomial roots.
/// - `decommitments`: the decommitments to the proof, which includes the evaluations of the trace and composition
/// polynomial at the given index and their sibling along with Merkle authentication paths, for each query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stark101Proof<F: IsField> {
    pub commitment: Stark101Commitment,
    pub decommitments: Vec<Stark101Decommitment<F>>,
}

impl<F: IsField + Serialize> Stark101Proof<F> {
    pub fn write_to_file(&self) -> String {
        let proof_str = serde_json::to_string(&self).expect("should serialize");
        let proof_filepath = std::env::current_dir().expect("should get current dir");
        let proof_filepath = if proof_filepath.ends_with("lambda-0b10") {
            proof_filepath.join("snarks/stark101/proof.json")
        } else {
            proof_filepath.join("proof.json")
        };

        let path_str = proof_filepath.to_str().unwrap().to_string();
        std::fs::write(proof_filepath, proof_str).expect("unable to write");

        path_str
    }
}
