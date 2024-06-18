use std::marker::PhantomData;

use lambdaworks_math::{
    field::{element::FieldElement, traits::IsField},
    polynomial::Polynomial,
};

pub struct ShamirSecretShare<F: IsField> {
    n: usize,                // num shares to create
    k: usize,                // needed number of shares to reconstruct
    phantom: PhantomData<F>, // to make F part of this struct
}

/// A share is just an evaluation point on the polynomial, i.e. `p(x) = y`.
#[derive(Clone, Debug)]
pub struct Share<F: IsField> {
    pub x: FieldElement<F>,
    pub y: FieldElement<F>,
}

impl<F: IsField> ShamirSecretShare<F> {
    pub fn new(n: usize, k: usize) -> Self {
        assert!(n > k);
        Self {
            n,
            k,
            phantom: PhantomData::default(),
        }
    }

    /// Given a secret, creates a set of shares:
    ///
    /// 1. Create a random polynomial of degree `k-1` with the secret as the constant term.
    /// 2. Evaluate the polynomial at `n` random points to create `n` shares.
    /// 3. Return the shares.
    pub fn create_shares(&self, secret: FieldElement<F>) -> Vec<Share<F>> {
        let xs = (0..=self.n)
            .map(|i| {
                if i == 0 {
                    FieldElement::<F>::zero()
                } else {
                    FieldElement::<F>::from(rand::random::<u64>())
                }
            })
            .collect::<Vec<_>>();

        let mut ys = (0..self.k)
            .map(|i| {
                if i == 0 {
                    secret.clone()
                } else {
                    FieldElement::<F>::from(rand::random::<u64>())
                }
            })
            .collect::<Vec<_>>();

        // interpolate from k points
        let poly = Polynomial::interpolate(&xs.as_slice()[..self.k], &ys).unwrap();

        // create additional shares
        let ys_extra = (self.k..=self.n)
            .map(|i| poly.evaluate(&xs[i]))
            .collect::<Vec<_>>();
        ys.extend(ys_extra);

        // return as Share objects
        xs.into_iter()
            .zip(ys)
            .skip(1) // skip the secret itself
            .map(|(x, y)| Share { x, y })
            .collect()
    }

    /// Given a set of shares, reconstructs the secret.
    ///
    /// 1. Use Lagrange interpolation to reconstruct the polynomial.
    /// 2. Evaluate the polynomial at `0` to get the secret.
    pub fn reconstruct_secret(&self, shares: Vec<Share<F>>) -> FieldElement<F> {
        assert!(shares.len() >= self.k, "not enough shares");
        let xs = shares.iter().map(|s| s.x.clone()).collect::<Vec<_>>();
        let ys = shares.iter().map(|s| s.y.clone()).collect::<Vec<_>>();

        let p = Polynomial::interpolate(&xs, &ys).expect("should interpolate");

        p.evaluate(&FieldElement::<F>::zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambdaworks_math::field::fields::u64_goldilocks_field::Goldilocks64Field;

    type F = Goldilocks64Field;

    fn test_shamir_secret_share(n: usize, k: usize) {
        let shamir = ShamirSecretShare::<F>::new(n, k);

        // create shares from secret
        let secret = FieldElement::<F>::from(rand::random::<u64>());
        let shares = shamir.create_shares(secret.clone());
        assert_eq!(shares.len(), n);

        // reconstruct the secret from a subset of shares
        let subset_shares = shares.into_iter().take(k).collect::<Vec<_>>();
        let reconstructed_secret = shamir.reconstruct_secret(subset_shares);
        assert_eq!(reconstructed_secret, secret);
    }

    #[test]
    fn test_n3_k2() {
        test_shamir_secret_share(3, 2);
    }

    #[test]
    fn test_n5_k3() {
        test_shamir_secret_share(5, 3);
    }

    #[test]
    fn test_n7_k4() {
        test_shamir_secret_share(7, 4);
    }
}
