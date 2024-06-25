use lambdaworks_math::field::{element::FieldElement, traits::IsField};

pub struct NTT<F: IsField> {
    /// Twiddle factors w^0, w^1, w^2, ..., w^(n-1) in order,
    /// so that `twiddles[i] = w^i`.
    pub twiddles: Vec<FieldElement<F>>,
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

    pub fn forward(&self, coeffs: &[FieldElement<F>]) -> Vec<FieldElement<F>> {
        assert!(self.twiddles.len() >= coeffs.len(), "too many inputs");
        let n = coeffs.len();
        if n == 1 {
            return coeffs.to_vec();
        }
        assert_eq!(n.count_ones(), 1, "n must be a power of 2");

        let half = n >> 1;
        let (even, odd) = even_odd_split(coeffs.to_vec());
        let (even, odd) = (self.forward(&even), self.forward(&odd));

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
    use super::*;

    #[test]
    fn test_even_odd_split() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let (even, odd) = even_odd_split(arr);

        assert_eq!(even, vec![1, 3, 5, 7]);
        assert_eq!(odd, vec![2, 4, 6, 8]);
    }
}
