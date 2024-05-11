use lambdaworks_math::field::{element::FieldElement, traits::IsField};

pub struct RSA<F: IsField> {
    e: FieldElement<F>,
}

impl<F: IsField> RSA<F> {
    pub fn new(e: FieldElement<F>) -> Self {
        RSA { e }
    }

    pub fn encrypt(&self, message: FieldElement<F>) -> FieldElement<F> {
        unimplemented!()
    }

    pub fn decrypt(&self, ciphertext: FieldElement<F>) -> FieldElement<F> {
        unimplemented!()
    }
}
