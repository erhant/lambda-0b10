> # Week 7
>
> Multivariate polynomials.

# Multivariate Polynomials

A multivariate polynomial is a polynomial with more than one variable. For example, the polynomial $x^2 + 2xy + y^2$ is a multivariate polynomial in the variables $x$ and $y$.

So far within the bootcamp we have worked with univariate polynomials, where we did interpolation with FFT and we made use of quotient polynomials by dividing some polynomial with a vanishing polynomial over some domain.

The degree of a multivariate polynomial is the highest sum of the exponents of the variables in any term of the polynomial. For example, the degree of the polynomial $x^2 + 2xy + y^2$ is 2.

Schwartz-Zippel lemma that we have made use of so far works here as well.

$$
\Pr[P(r) = 0 : r \gets \mathbb{F}] \leq \frac{d \cdot n}{|\mathbb{F}|}
$$

## Multi-linear Extension (MLE)

For a given function $f : S \to \mathbb{F}_p$ we can obtain a multilinear extension $\tilde{f}$ such that for the given $(x_0, x_1, \ldots, x_{n-1})$ we have:

$$
f(x_0, x_1, \ldots, x_{n-1}) = \tilde{f}(x_0, x_1, \ldots, x_{n-1})
$$

We can think of this like interpolation we had done for univariate polynomials.

Multi-linear extension is **unique**, just like the univariate interpolations!

### Lagrange Basis over Boolean Hypercube

The set $S = \{0, 1\}^n$ is sometimes known as the **Boolean hypercube**.

We can talk about Lagrange basis polynomials in the multivariate case as well. Consider $f(x) = v$. We can create a Lagrange basis that is 1 when $x = v$ and 0 otherwise:

$$
L_r(x) = \prod_{i=0}^{n-1} (x_iv_i + (1 - x_i)(1 - v_i))
$$

where $f(x) = v$ and $v_i$ are the bits from the binary decomposition of $v$.

With many such basis functions, we can construct the MLE:

$$
\tilde{f}(x) = \sum_{v}^{2^n-1} f(v) \cdot L_v(x)
$$

### Evaluating at a Random Point

In the multivariate case, we do not make use of FFT; we instead use a clever algorithm around the Lagrange basis construction.

> See the "Proofs, Arguments, and Zero-Knowledge" book by Justin Thaler to see the efficient method, chapters 3 & 4.

We can evaluate the multilinear extension at a random point $r = (r_0, r_1, \ldots, r_n)$ efficiently to get the value of the function at that point:

$$
\tilde{f}(r_0, r_1, \ldots, r_n) = \sum_{v} f(v) \cdot L_v(r_0, r_1, \ldots, r_n)
$$

# Sum-Check Protocol

Sum-check protocol is an important protocol in the context of MLE-based proof systems. It is efficient, and has low communication costs. Consider a $\nu$-variate polynomial $g(x_1, x_2, \ldots, x_\nu)$ of degree $d$ over $\mathbb{F}$, and a set $S = \{0, 1\}^\nu$; the Sum-check proves that $H$ is the result of the sum below:

$$
\sum_{x_1\in\{0, 1\}}\sum_{x_2\in\{0, 1\}}\ldots\sum_{x_\nu\in\{0, 1\}}g(x_1, x_2, \ldots, x_\nu) = H
$$

We want to reduce the amount of work that the verifier has to do to check this result. In the naive version, the verifier would have to check $2^\nu$ values. The sum-check protocol reduces this to $O(\nu)$, along with a cost to evaluate $g$ at a random point.

## Protocol

> We will describe the interactive one, but one can use Fiat-Shamir to make this non-interactive.

1. The prover sends the verifier the value $c_1$ which is claimed to be equal to $H$.

2. The prover sends $g_1(x_1)$ (a univariate polynomial of degree less than $d$) which is claimed to equal to:

$$
\sum_{y_2\in\{0, 1\}}\sum_{y_3\in\{0, 1\}}\ldots\sum_{y_\nu\in\{0, 1\}}g(x_1, y_2, y_3, \ldots, y_\nu)
$$

> Basically, the sum is taken over all values except the first one.

The verifier checks that $g_1(0) + g_1(1) = c_1$, which essentially translates to the original summation above, and also makes sure that $g_1$ is a polynomial of degree less than $d$.

> Verifier can check the degree by checking the number of coefficients, since the polynomial is sent in clear.

3. Verifier chooses random $r_1$ and sends it to the prover. We must now make sure that $g_1(r_1)$ is the correct value.

$$
\sum_{y_2\in\{0, 1\}}\sum_{y_3\in\{0, 1\}}\ldots\sum_{y_\nu\in\{0, 1\}}g(r_1, y_2, y_3, \ldots, y_\nu)
$$

Well, the expression above is just like the sum-check protocol, but instead of $\nu$ variables, we have $\nu - 1$ variables with $x_1 = r_1$ fixed. We can repeat the process above to check prove this result.

4. The prover sends $g_2(x_2)$ which is claimed to equal:

$$
g_2(x) = \sum_{y_3\in\{0, 1\}}\sum_{y_4\in\{0, 1\}}\ldots\sum_{y_\nu\in\{0, 1\}}g(r_1, x_2, y_3, \ldots, y_\nu)
$$

The verifier checks that $g_2(0) + g_1(1) = g_1(r_1)$, and confirms the degree.

5. Verifier chooses random $r_2$ and sends it to the prover. We must now make sure that $g_2(r_2)$ is the correct value.

**and so on...**

Towards the end, the prover sends $g_\nu(x_\nu)$, and verifier checks that $g_\nu(0) + g_\nu(1) = g_{\nu-1}(r_{\nu-1})$ along with the degree.

At the final step, verifier picks random $r_\nu$ and verifies that:

$$
g_\nu(r_\nu) = g(r_1, r_2, \ldots, r_\nu)
$$

and if this is true, it **accepts**. This final evaluation over $g$ is the random point evaluation we talked about earlier.

> There is also something called the Zero-Check Protocol.

## Proof Systems & PCS based on MLE & Sum-Check

- **HyperPlonk** adapts the Plonk protocol to multivariate polynomials.

- **Spartan** is a proof system based on R1CS.

- **Brakedown** is a polynomial commitment scheme based on multivariate polynomials.

- **Binius** is a really efficient proof system.
