use lambdaworks_math::field::{element::FieldElement, traits::IsField};

/// Returns the trace of a program for `FibonacciSq`.
pub fn fibonacci_square<F: IsField>(
    a_0: FieldElement<F>,
    a_1: FieldElement<F>,
    n: usize,
) -> Vec<FieldElement<F>> {
    let mut trace = Vec::with_capacity(n);
    trace.push(a_0);
    trace.push(a_1);

    for i in 2..n {
        let a_i = trace[i - 1].square() + trace[i - 2].square();
        trace.push(a_i);
    }

    trace
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::Stark101PrimeFieldElement as FE;

    #[test]
    fn test_trace() {
        let n = 1023;
        let a_0 = FE::from(1u64);
        let a_1 = FE::from(3141592u64);
        let a = fibonacci_square(a_0, a_1, n);

        assert_eq!(a.len(), n);
        assert_eq!(*a.last().unwrap(), FE::from(2338775057u64));
    }
}
