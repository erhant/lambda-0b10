# Shamir's Secret Sharing

Shami's Secret Sharing is a cryptographic technique that allows a secret to be split into multiple parts, called _shares_, in such a way that the secret can only be reconstructed when a sufficient number of shares are combined together. This is a form of threshold cryptography, where the secret is divided into $n$ shares, and the secret can be reconstructed only when $k$ shares are combined together.

Consider a secret $s$ that we want to split into $n$ shares such that the secret can be reconstructed when $k$ shares are combined together. The secret sharing scheme works as follows:

1. The secret $s$ is represented as a polynomial $f(x) = s + a_1x + a_2x^2 + \ldots + a_{k-1}x^{k-1}$ of degree $k-1$.

2. $n$ points $(x_1, f(x_1)), (x_2, f(x_2)), \ldots, (x_n, f(x_n))$ are chosen on the polynomial $f(x)$, where $x_1, x_2, \ldots, x_n$ are distinct.

3. The shares are given to the participants, where each share has the form: $(x_i, f(x_i))$.

4. To reconstruct the secret, a participant must have at least $k$ shares. The secret can be reconstructed by interpolating (via Lagrange Interpolation) the polynomial $f(x)$ using the $k$ shares.

5. The secret $s$ can be reconstructed by evaluating the polynomial $f(x)$ at $x = 0$ to obtain $f(0) = s$.

## Usage

The API is rather straightforward:

```rs
// n shares, with k shares required to reconstruct the secret
let shamir = ShamirSecretShare::<F>::new(n, k);

// create shares from secret
let shares = shamir.create_shares(secret);
assert_eq!(shares.len(), n);

// reconstruct the secret from a subset of shares
let reconstructed_secret = shamir.reconstruct_secret(shares);
assert_eq!(reconstructed_secret, secret);
```
