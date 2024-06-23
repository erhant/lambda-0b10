# Sumcheck Protocol

Consider a $n$-variate polynomial $g(x_1, x_2, \ldots, x_n)$ of degree $d$ over $\mathbb{F}$, and a set $S = \{0, 1\}^n$ also denoted as a Boolean hypercube. We are interested in the following sum:

$$
\sum_{(x_1, x_2, \ldots, x_n)\in\{0, 1\}^n}g(x_1, x_2, \ldots, x_n) = H
$$

A naive proof would be to send the polynomial to the verifier and have him evaluate the polynomial at all $2^n$ points and have it sum them up. However, this is not efficient. Instead, we can use the sumcheck protocol to prove the sum in a more efficient manner.

### First Round

The prover computes the sum $H$ and it sends a value $C_1$ that is claimed to equal $H$. Along with that, the prover sends a univariate polynomial $g_1(X_1)$ that is claimed to equal:

$$
\sum_{(x_2, x_3, \ldots, x_n)\in\{0, 1\}^{n-1}}g(X_1, x_2, \ldots, x_n)
$$

The verifier checks that $g_1(0) + g_1(1) = C_1$ and that $g_1$ is a univariate polynomial of degree at most $\deg_1(g)$ (the degree of the term $X_1$ in the $n$-variate $g$ polynomial). If the check passes, the verifier sends a random challenge $r_1$ to the prover.

### Middle Rounds

In each intermediate round $j$, the prover sends a univariate polynomial $g_j(X_j)$ which it claims to equal:

$$
\sum_{(x_{j+1}, \ldots, x_n)\in\{0, 1\}^{n-j}}g(r_1, r_2, \ldots, r_{j-1}, X_j, x_{j+1}, \ldots, x_n)
$$

The verifier checks that $g_j(0) + g_j(1) = g_{j-1}(r_{j-1})$ and that $g_1$ is a univariate polynomial of degree at most $\deg_j(g)$ (the degree of the term $X_j$ in the $n$-variate $g$ polynomial). If the check passes, the verifier sends a random challenge $r_j$ to the prover.

> The first round can be seen as a special case of the middle round, where instead of a univariate polynomial we have a constant polynomial for $g_0$.

### Final Round

With the end of last "middle" round the prover had sent the polynomial $g_n(X_n)$ that is claimed to equal $g(r_1, r_2, \ldots, X_n)$, and the verifier checked it as described above. Then, we ended up with $n$ random values $(r_1, r_2, \ldots, r_n)$. Finally, the verifier makes an oracle query to the polynomoial $g$ itself to compute:

$$
g(r_1, r_2, \ldots, r_n)
$$

The verifier checks that:

$$
g_n(r_n) = g(r_1, r_2, \ldots, r_n)
$$

If all checks pass, the verifier accepts the proof.

## Usage

Sumcheck takes in a `DenseMultilinearPolynomial` created from evaluations of a function, meaning that a multi-linear extension (MLE) takes place from those evaluations.

The struct simple has a `prove` function, which returns a `SumCheckProof` struct that has a `verify` function. Verification panics if the proof is invalid.

```rs
// assuming `evals` exist
let poly = DenseMultilinearPolynomial::new(evals);

// create proof
let sumcheck = SumCheck::new(poly);
let proof = sumcheck.prove();

// verify proof
proof.verify();
```

## Implementation

The `prove` function is rather straightforward, we begin with the first interpolation and the computation for $C_1$, then we proceed with the middle rounds and finally the final round.

Within the loop, we check the results of previous round (i.e. $g_{j-1}(r_{j-1}) = g_j(0) + g_j(1)$) and the degree of the polynomial. Then, a random variable is sampled and we proceed to the next round.

If the round is not final, a new polynomial is interpolated with the given random variables. At the final round, the last check $g_n(r_n) = g(r_1, r_2, \ldots, r_n)$ is performed.

Prover does all the verifier checks during proof creation for sanity, and the proof simply contains the interpolated polynomials. The random variables are obtained from the transcript, so they are not stored in the proof.

> [!NOTE]
>
> This implementation is rather naive, especially within the univariate interpolations. For example, suppose we have $g(x_1, x_2, x_3)$ and we would like to interpolate $g_1(X_1) = g(X_1, 0, 0) + g(X_1, 0, 1) + g(X_1, 1, 0) + g(X_1, 1, 1)$. This is done via 4 different Lagrange interpolations, one for each term, and for each interpolation we evaluate it at 2 points (because we are ok with a degree 1 polynomial). It is possible to reuse some of these interpolations within the recursive steps, but I did not bother with that here.
>
> As a result, if you try with a high number of variables, this may take a while to compute.
