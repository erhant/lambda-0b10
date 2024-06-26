use std::env;
use std::error::Error;
use csv::ReaderBuilder;

use lambdaworks_math::field::element::FieldElement;
use lambdaworks_math::field::fields::u64_prime_field::U64PrimeField;
use lambdaworks_math::polynomial::dense_multilinear_poly::DenseMultilinearPolynomial;
use sumcheck::sumcheck::SumCheck;

const FIELD_MODULUS: u64 = 65537;
type F = U64PrimeField<FIELD_MODULUS>;
type FE = FieldElement<F>;

fn read_matrix_from_csv(file_path: &str) -> Result<Vec<Vec<FE>>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().has_headers(false).from_path(file_path)?;
    let mut matrix = Vec::new();

    for result in reader.records() {
        let record = result?;
        let row: Vec<FE> = record.iter()
            .map(|s| {
                match s.parse::<u64>() {
                    Ok(v) => FE::from(v),
                    Err(_) => panic!("Unable to parse field matrix from file.")
                }
            })
            .collect();
        matrix.push(row);
    }

    Ok(matrix)
}

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let file_path = "examples/adjacency_matrix.csv";
    let adj_matrix = read_matrix_from_csv(file_path)?;

    let num_vertices = adj_matrix.len(); // number of vertices
    assert!(num_vertices.is_power_of_two(), "The number of vertices must be a power of two");
    // check that chosen field is large enough for the counting triangles problem
    // (2023 - Thaler - Proof, Arguments and Zero-Knowledge, Section 4.3, page 44)
    assert!(FIELD_MODULUS > 6 * (num_vertices.pow(3) as u64), "Field is not large enough.");

    let num_bits = num_vertices.trailing_zeros() as usize;
    let num_vars = 3 * num_bits; // number of variables
    let num_evals = 1 << num_vars; // number of evaluations
    log::debug!("Num. of vertices: {}", num_vertices);
    log::debug!("Num. of variables: {}", num_vars);
    log::debug!("Num. of evaluations: {}", num_evals);

    log::info!("Getting evaluations of g over the boolean hypercube");
    let evals = (0..num_evals)
        .map(|i| {
            let x = i / num_vertices.pow(2);
            let y = (i % num_vertices.pow(2)) / num_vertices;
            let z = i % num_vertices;
            let g = adj_matrix[x][y] * adj_matrix[y][z] * adj_matrix[z][x];
            log::debug!(
                "g({:?}, {:?}, {:?}) = {:?}",
                i / num_vertices.pow(2), (i % num_vertices.pow(2)) / num_vertices , i % num_vertices,
                g.representative()
            );

            g
        })
        .collect::<Vec<FE>>();

    // get number of triangles. The sum of g over the hypercube is  divided by the number of permutations (6)
    let num_triangles = evals.iter().fold(FE::zero(), |acc, g| acc + g) / FE::from(6);
    log::info!(
        "The number of triangles in the graph is: {:?}", 
        num_triangles.representative()
    );

    // create a dense multilienar poly from the evaluations
    let poly = DenseMultilinearPolynomial::new(evals);
    assert_eq!(poly.len(), num_evals);
    assert_eq!(poly.num_vars(), num_vars);

    // create sumcheck proof
    let sumcheck = SumCheck::new(poly);
    let proof = sumcheck.prove();

    // verify proof
    proof.verify();

    Ok(())
}