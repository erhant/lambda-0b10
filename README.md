# Sparkling Water Bootcamp 0b10

This repository is an umbrella repository for my notes during [Sparkling Water Bootcamp in Cryptography 01b0](https://github.com/lambdaclass/lambdaworks/blob/bootcamp0b10/bootcamp/sparkling_water_0b10.md).

- [Week 1](./docs/WEEK-1.md): Arithmetic, Abstract Algebra, Polynomials, RSA, FFT.
- [Week 2](./docs/WEEK-2.md): Elliptic Curves, Commitments & Hashing, Pairings.
- [Week 3](./docs/WEEK-3.md): SNARKs, KZG, BabySNARK.
- [Week 4](./docs/WEEK-4.md): STARKs, FRI.
- [Week 5](./docs/WEEK-5.md): PlonK.
- <strike>[Week 6](./docs/WEEK-6.md)</strike> (_skipped due to conference_)
- [Week 7](./docs/WEEK-7.md): Multi-linear Extension, Sum-Check, Binius.
- Week 8

Other things included here:

- [Interview](./exercises/interview/README.md): bootcamp interview questions & answers.
- [RSA](./exercises/rsa/): very basic implementation of RSA cryptosystem.
- [Shamir's Secret Sharing](./exercises/shamir-secret-share/): a basic Shamir's Secret Sharing implementation.
- [Vault of Loki](./exercises/vault-of-loki/README.md): KZG fake proof challenge by LambdaClass.
- [BabySnark](./snarks/babysnark/): small example circuits using BabySnark.
- [Stark101](./snarks/stark101/README.md): the Stark101 prover, based on its blog posts.
- [Sumcheck Protocol](./exercises/sumcheck/): an implementation of Sumcheck protocol for multilinear polynomials.

You can run any of these via the Makefile, just see:

```sh
make interview
make rsa
make shamir
make vault-of-loki
make stark101
make sumcheck
```
