# [Stark101](https://github.com/starkware-industries/stark101)

Our objective is to prove that we know $x$ such that $a_{1022} = 2338775057$ in a finite field where $a_n$ is given by the Fibonacci sequence $a_n = a_{n-1} + a_{n-2}$ with $a_0 = 1$ and $a_1 = x$. The order of field is $3 \times 2^{30} - 1 = 3221225473$.

- [Notebook 1](https://github.com/starkware-industries/stark101/blob/master/tutorial/Stark101-part1.ipynb) - [Video 1](https://www.youtube.com/watch?v=Y0uJz9VL3Fo)
- [Notebook 2](https://github.com/starkware-industries/stark101/blob/master/tutorial/Stark101-part2.ipynb) - [Video 2](https://www.youtube.com/watch?v=fg3mFPXEYQY)
- [Notebook 3](https://github.com/starkware-industries/stark101/blob/master/tutorial/Stark101-part3.ipynb) - [Video 3](https://www.youtube.com/watch?v=gd1NbKUOJwA)
- [Notebook 4](https://github.com/starkware-industries/stark101/blob/master/tutorial/Stark101-part4.ipynb) - [Video 4](https://www.youtube.com/watch?v=CxP28qM4tAc)
- [Notebook 5](https://github.com/starkware-industries/stark101/blob/master/tutorial/Stark101-part5.ipynb) - [Video 5](https://www.youtube.com/watch?v=iuNbrTkH2ik)

The implementation is found within the [`main.rs`](./src/main.rs) file. We make use of LambdaWorks's following tools together with our custom field:

- MerkleTree using `Sha2_256Backend` for Merkle commitments
- Transcript using `DefaultTranscript`, for the Fiat-Shamir transform
- Polynomial library for polynomial operations
- An additional Proof struct has been written, so that at the end the proof is serialized & saved on disk.

> [!TIP]
>
> We stick to the naming conventions used in the tutorial, so it should be easy to follow the code along with the notebooks.

## Usage

Run the prover via:

```sh
cargo run --release --bin stark101
```

> [!TIP]
>
> `debug` mode is rather slow especially during the most compute-intensive interpolation part, so we use `release` mode instead.
