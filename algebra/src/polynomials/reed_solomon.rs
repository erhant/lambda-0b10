use lambdaworks_math::{
    field::{element::FieldElement, traits::IsField},
    polynomial::Polynomial,
};

pub struct ReedSolomon {
    /// Codeword length
    n: usize,
    /// Message length
    k: usize,
}

// TODO: !!!

impl ReedSolomon {
    pub fn new(n: usize, k: usize) -> Self {
        assert!(n > k);
        ReedSolomon { n, k }
    }

    /// Distance between codewords
    pub fn distance(&self) -> usize {
        self.n - (self.k - 1)
    }

    /// Encodes the message into a codeword. There are several methods here:
    ///
    /// - Encode the message as evaluations of a polynomial of degree k-1, and evaluate the polynomial at n points.
    /// - Treat the message as coefficients of a polynomial of degree k-1, and evaluate the polynomial at n points.
    pub fn encode<F: IsField>(&self, message: Vec<FieldElement<F>>) -> Vec<FieldElement<F>> {
        assert!(message.len() == self.k);
        // treat the messages as evaluations of a polynomial
        let mut xs = Vec::with_capacity(self.k);
        let mut ys = Vec::with_capacity(self.k);
        for (i, m) in message.iter().enumerate().take(self.k) {
            xs.push(FieldElement::<F>::from(i as u64));
            ys.push(m.clone())
        }

        // interpolates the polynomial from the given message
        let p = Polynomial::interpolate(&xs, &ys).expect("should interpolate");

        // evaluate the polynomial at the n-k extra points, to obtain n evaluations in total
        for i in self.k..self.n {
            let i = FieldElement::<F>::from(i as u64);
            ys.push(p.evaluate(&i));
        }

        ys
    }

    pub fn decode() -> ! {
        unimplemented!("Quite hard to implement a decoder")
    }
}
