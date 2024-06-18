use lambdaworks_crypto::{
    fiat_shamir::is_transcript::IsTranscript,
    merkle_tree::{
        backends::{field_element::FieldElementBackend, types::Sha2_256Backend},
        merkle::MerkleTree,
        traits::IsMerkleTreeBackend,
    },
};
use lambdaworks_math::{
    field::{element::FieldElement, traits::IsField},
    polynomial::Polynomial,
    traits::AsBytes,
};

/// Given a domain of length `n`, returns the first half of it with each element squared.
pub fn next_fri_domain<F: IsField>(domain: Vec<FieldElement<F>>) -> Vec<FieldElement<F>> {
    domain
        .iter()
        .take(domain.len() >> 1)
        .map(|x| x.square())
        .collect()
}

/// Given a polynomial `poly` and a field element `beta`, returns the folding operator applied to `poly`.
///
/// What happens here is that `poly` is split into even and odd coefficients, and the odd part is multiplied
/// by `beta`, therefore reducing the degree of the polynomial by half.
pub fn next_fri_polynomial<F: IsField>(
    poly: Polynomial<FieldElement<F>>,
    beta: FieldElement<F>,
) -> Polynomial<FieldElement<F>> {
    let even_coeffs = poly
        .coefficients
        .clone()
        .into_iter()
        .step_by(2)
        .collect::<Vec<_>>();
    let even = Polynomial::new(&even_coeffs);

    let odd_coeffs = poly
        .coefficients
        .clone()
        .into_iter()
        .skip(1)
        .step_by(2)
        .collect::<Vec<_>>();
    let odd = Polynomial::new(&odd_coeffs);

    even + beta * odd
}

pub fn next_fri_layer<F: IsField>(
    poly: Polynomial<FieldElement<F>>,
    domain: Vec<FieldElement<F>>,
    beta: FieldElement<F>,
) -> (
    Polynomial<FieldElement<F>>,
    Vec<FieldElement<F>>,
    Vec<FieldElement<F>>,
) {
    let next_poly = next_fri_polynomial(poly, beta);
    let next_domain = next_fri_domain(domain);
    let next_layer = next_domain
        .iter()
        .map(|x| next_poly.evaluate(x))
        .collect::<Vec<_>>();

    (next_poly, next_domain, next_layer)
}

pub fn fri_commit<F: IsField + Send + Sync, T: IsTranscript<F>>(
    cp: Polynomial<FieldElement<F>>,
    domain: Vec<FieldElement<F>>,
    cp_eval: Vec<FieldElement<F>>,
    cp_merkle: MerkleTree<Sha2_256Backend<F>>,
    mut channel: T,
) -> (
    Vec<Polynomial<FieldElement<F>>>,
    Vec<Vec<FieldElement<F>>>,
    Vec<Vec<FieldElement<F>>>,
    Vec<MerkleTree<Sha2_256Backend<F>>>,
)
where
    FieldElement<F>: AsBytes + Send + Sync,
{
    let mut fri_polys = vec![cp];
    let mut fri_domains = vec![domain];
    let mut fri_layers = vec![cp_eval];
    let mut fri_merkles = vec![cp_merkle];

    // apply FRI until you end up with a constant polynomial
    while fri_polys.last().unwrap().degree() > 0 {
        // sample randomness
        let beta = channel.sample_field_element();

        // apply FRI operator
        let (next_poly, next_domain, next_layer) = next_fri_layer(
            fri_polys.last().unwrap().clone(),
            fri_domains.last().unwrap().clone(),
            beta,
        );

        fri_polys.push(next_poly);
        fri_domains.push(next_domain);
        fri_layers.push(next_layer.clone());

        // commit to layer & add root to transcript
        let tree = MerkleTree::<Sha2_256Backend<F>>::build(&next_layer);
        channel.append_bytes(&tree.root);
        fri_merkles.push(tree);
    }

    // add constant polynomial to transcript
    assert_eq!(fri_polys.last().unwrap().degree(), 0);
    channel.append_field_element(&fri_layers.last().unwrap()[0]);

    (fri_polys, fri_domains, fri_layers, fri_merkles)
}
