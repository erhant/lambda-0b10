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
