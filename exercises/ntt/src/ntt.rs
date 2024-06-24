use lambdaworks_math::field::{element::FieldElement, traits::IsField};

pub struct NTT<F: IsField> {
    /// Twiddle factors w^0, w^1, w^2, ..., w^(n-1) in order,
    /// so that `twiddles[i] = w^i`.
    twiddles: Vec<FieldElement<F>>,
}

impl<F: IsField> NTT<F> {
    pub fn new(w: FieldElement<F>, n: u64) -> Self {
        // must be primitive 2^k-th root of unity
        assert_eq!(n.count_ones(), 1, "n must be a power of 2");
        assert_eq!(w.pow(n), FieldElement::one());
        assert_ne!(w.pow(n - 1), FieldElement::one());

        Self {
            // construct twiddle factors w^0, w^1, w^2, ..., w^(n-1)
            twiddles: (0..n).map(|i| w.pow(i)).collect(),
        }
    }

    pub fn ntt(&self, coeffs: &[FieldElement<F>]) -> Vec<FieldElement<F>> {
        assert!(self.twiddles.len() >= coeffs.len(), "too many inputs");
        let n = coeffs.len();
        if n == 1 {
            return coeffs.to_vec();
        }
        assert_eq!(n.count_ones(), 1, "n must be a power of 2");

        let half = n >> 1;
        let (even, odd) = even_odd_split(coeffs.to_vec());
        let (even, odd) = (self.ntt(&even), self.ntt(&odd));

        let mut res = vec![FieldElement::zero(); n];
        for j in 0..half {
            // A_j = E_j + w^j * O_j
            res[j] = even[j].clone() + self.twiddles[j].clone() * odd[j].clone();

            // A_{j + n/2} = E_j - w^j * O_j
            res[j + half] = even[j].clone() - self.twiddles[j].clone() * odd[j].clone();
        }

        res
    }
}

/// Splits a given array into two arrays, one containing the elements at even indices and the other
/// containing the elements at odd indices.
pub fn even_odd_split<T>(arr: Vec<T>) -> (Vec<T>, Vec<T>) {
    let (even, odd): (Vec<_>, Vec<_>) = arr.into_iter().enumerate().partition(|(i, _)| i & 1 == 0);

    let even = even.into_iter().map(|(_, x)| x).collect::<Vec<_>>();
    let odd = odd.into_iter().map(|(_, x)| x).collect::<Vec<_>>();

    (even, odd)
}

#[cfg(test)]
mod tests {
    use lambdaworks_math::field::test_fields::u64_test_field::U64Field;

    use super::*;

    #[test]
    fn test_even_odd_split() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let (even, odd) = even_odd_split(arr);

        assert_eq!(even, vec![1, 3, 5, 7]);
        assert_eq!(odd, vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_ntt_17() {
        // define a prime field of order 17
        type F = U64Field<17>;
        type FE = FieldElement<F>;

        // 13 is a primitive 4-th root of unity
        // and 4 is the max we can have here because 17 - 1 = 2^4
        let w = FE::from(13u64);

        // see powers of w
        for i in 1..=4u64 {
            println!("w^{} = {}", i, w.pow(i).representative());
        }

        let coeffs = (0..4).map(|i| FE::from(i as u64)).collect::<Vec<_>>();
        let ntt = NTT::new(w, 4);

        let evals = ntt.ntt(&coeffs);

        for (i, e) in evals.iter().enumerate() {
            println!("A_{} = {}", i, e.representative());
        }
    }
}
