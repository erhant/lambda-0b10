use lambdaworks_math::polynomial::Polynomial;

use crate::field::Stark101PrimeFieldElement as FE;

pub fn interpolate(xs: &[FE], ys: &[FE]) -> Polynomial<FE> {
    // TODO: use FFT
    Polynomial::interpolate(xs, ys).expect("should interpolate ")
}
