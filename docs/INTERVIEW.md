# Lambda0b10

> Using lambdaworks, compute the public key associated with the secret key `0x6C616D6264617370` with the BLS12-381 elliptic curve. Provide link to repo.

It is the point `(0x67f9ffc5eaf6c19292112eadf50c11e7460e7568493141676f6ba1374badd9f6ab1f2f5e155b0e3d2f4c1de79554f80, 0x18509d22f2107b667a8f75de737a4fb967f6c3e745a7c2361868515402318f006bd360b8a8763d7844381c6e510799cc)`. See <https://github.com/erhant/lambda-0b10/blob/main/examples/chal_bls12_381.rs>

> What is a lookup argument and what are the protocols used?

A lookup argument is a set-membership argument, i.e. instead of computing f(x) = y, one can lookup a table where the domain and range of f is given as columns, and see if a row has x and y in that table. This helps with the cases where computing f is costly, (i.e. a SHA256 hash in a circuit) but looking up the input and output is much more efficient. Plookup (A. Gabizon & Z. Williamson) was a milestone in the lookup tables scene. TinyRAM (Bootle et al.) is another earlier work that made use of lookups.

> What are the differences between SHA-2 and SHA-3?

SHA-2 uses Merkle-Damgard construction, while SHA-3 uses Sponge construction. MD constructions are open to length extension attacks, Sponge are not. SHA-2 is more performant than SHA-3, although SHA-3 is more amenable to parallelization.

> Explain Reed-Solomon codes in a concise way

Reed-Solomon code is an error-correcting code, where a message of length K is treated as a univariate polynomial of degree K-1, and the codeword is the evaluation of this polynomial at N publicly known points. The distance of this code is n - k + 1, which is actually the most optimal one; i.e. Reed-Solomon code is an optimal linear code.

> Give the proof systems included in lambdaworks

Based on the table under the README of Lambdaworks, it currently supports Groth16 and STARK.

> Give the multiplicative inverse of 2 modulo $2^{64} - 2^{32} + 1$ (the so-called mini-Goldilocks prime)

It is `9223372034707292161`. See <https://github.com/erhant/lambda-0b10/blob/main/examples/goldilocks.rs>.

> Explain generics in Rust.

Generics are template arguments that allow an implementation or definition to respect multiple types. When the logic of some code is applicable to multiple types, we go for generics instead of writing that code for each type separately. Generics can also help define objects (structs) that accept multiple types as well.

Rust's generics are quite powerful, in the sense that we can not only specify types but also specify traits: we can have template arguments that accept any type that implements a certain function, or a trait.

> Why are we launching this today? What makes this day so special?

Perhaps its because today is the day before the Dencun upgrade on Ethereum, and we owe the tech behind it to a lot of cryptography. Not sure though, every day is special if it is the start of a cryptography bootcamp! :)
