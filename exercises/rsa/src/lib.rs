use num_bigint::BigUint;
use num_traits::FromPrimitive;

/// A dummy implementation of the RSA cryptosystem.
pub struct RSA {
    e: BigUint,
    d: BigUint,
    n: BigUint,
}

impl RSA {
    pub fn new(p: BigUint, q: BigUint) -> Self {
        let one = BigUint::from_u8(1).unwrap();
        let n = p.clone() * q.clone();
        let phi_n = (p - one.clone()) * (q - one.clone());

        let e = BigUint::from_u32(65537).unwrap();

        // e * d = 1 (mod phi(n))
        let e_inv = e.modinv(&phi_n.clone()).unwrap();
        assert!(
            (e.clone() * e_inv.clone()) % phi_n.clone() == one,
            "not inverse"
        );

        RSA { e, n, d: e_inv }
    }

    pub fn public_key(&self) -> (&BigUint, &BigUint) {
        (&self.e, &self.n)
    }

    pub fn secret_key(&self) -> &BigUint {
        &self.d
    }

    pub fn encrypt(&self, m: BigUint) -> BigUint {
        m.modpow(&self.e, &self.n)
    }

    pub fn decrypt(&self, c: BigUint) -> BigUint {
        c.modpow(&self.d, &self.n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsa() {
        let p = BigUint::from_u32(61).unwrap();
        let q = BigUint::from_u32(53).unwrap();
        let rsa = RSA::new(p, q);

        let m = BigUint::from_u32(42).unwrap();
        assert_eq!(m, rsa.decrypt(rsa.encrypt(m.clone())));
    }
}
