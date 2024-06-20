> # Week 7
>
> Multivariate polynomials, Multi-linear Extensions & Sumcheck, Binius.

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

where $v$ is a binary decomposition of $v$.

We can think of this like interpolation we had done for univariate polynomials.

Multi-linear extension is **unique**, just like the univariate interpolations!

### Lagrange Basis over Boolean Hypercube

The set $S = \{0, 1\}^n$ is sometimes known as the **Boolean hypercube**.

We can talk about Lagrange basis polynomials in the multivariate case as well. Consider $f(x) = v$. We can create a Lagrange basis that is 1 when $x = v$ and 0 otherwise:

$$
L_r(x) = \prod_{i=0}^{n-1} (x_ir_i + (1 - x_i)(1 - r_i))
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

## See also

- <https://semiotic.ai/articles/sumcheck-tutorial/>
- <https://publish.obsidian.md/matteo/3.+Permanent+notes/Sum-Check+Protocol>
- <https://github.com/0xSage/thaler>
- [Proofs, Arguments, and Zero-Knowledge by Justin Thaler](https://people.cs.georgetown.edu/jthaler/ProofsArgsAndZK.pdf) Chapters 3 & 4

# Binius & Brakedown

> [Binius](https://eprint.iacr.org/2023/1784) is a modified [Brakedown](https://eprint.iacr.org/2021/1043)-type commitment. We will not go into details of **Binius**, but instead we will describe **Brakedown** and **Binary Fields**.

Brakedown is a hash-based commitment scheme, which we will now go over. Lets consider a polynomial $p(x)$ of degree 15, with 16 coefficients $(a_0, a_1, \ldots, a_{15})$. Evaluating this polynomial at $x = z$ translates to:

$$
P(z) = a_0 + a_1z + a_2z^2 + \ldots + a_{15}z^{15}
$$

This actually translates to a "vector \* matrix \* vector" multiplication:

$$
P(z) = v^t \cdot M \cdot w
$$

such that:

$$
v = \begin{bmatrix} 1 \\ z^4 \\ z^8 \\ z^{12} \end{bmatrix},
M = \begin{bmatrix}
a_0 & a_1 & a_2 & a_3 \\
a_4 & a_5 & a_6 & a_7 \\
a_8 & a_9 & a_{10} & a_{11} \\
a_{12} & a_{13} & a_{14} & a_{15}
\end{bmatrix},
w = \begin{bmatrix} 1 \\ z \\ z^2 \\ z^3 \end{bmatrix}
$$

Now, a similar idea applies to multilinear extension as well. Recall that Lagrange basis are given by:

$$
\{L_k^{(r)}\} = \textcircled{x}_{i=0}^n(1-r_i, r_i)
$$

Here the circled $x$ means a tensor product.

$$
\tilde{f}(r) = [ \textcircled{x}_{i=l_1}^n(1-r_i, r_i)]^t \cdot M \cdot  \textcircled{x}_{i=0}^{l_1-1}(1-r_i, r_i)
$$

What we have to do is that we must organize the coefficients within the MLE $\tilde{f}(x_0, x_1, \ldots, x_n)$ into a matrix $M$, as shown below:

$$
M = \begin{bmatrix}
\text{row}_0 \\
\text{row}_1 \\
\ldots \\
\text{row}_{n-l_1} \\
\end{bmatrix}
$$

Create a new matrix $U$ where each row of $U$ is the encoding $\text{Enc}$ (using some linear code) of each row of $M$.

$$
U = \begin{bmatrix}
\text{Enc}(\text{row}_0) \\
\text{Enc}(\text{row}_1) \\
\ldots \\
\text{Enc}(\text{row}_{n-l_1}) \\
\end{bmatrix}
$$

Then, we create a Merkle tree using the columns of $U$ as defined above.

Prover wants to show that $\tilde{f}(r) = v$ (Eval procedure PCS). Prover sends $L$ to the verifier, as defined below:

$$
L = [ \textcircled{x}_{i=l_1}^n(1-r_i, r_i)]^t \cdot M
$$

whic is simply a linear combination of the rows of $M$.

1. The verifier can check the evaluation by completing the rest of the operation using the right hand-side tensor product.

$$
L \cdot \textcircled{x}_{i=0}^{l_1-1}(1-r_i, r_i) = v = \tilde{f}(r)
$$

Doing this requires around $2^{l_1}$ operations, and we usually have $l_1 \approx n/2$.

2. Verifier then checks the encoding of $L$:

$$
\text{Enc}(L) = [ \textcircled{x}_{i=l_1}^n(1-r_i, r_i)]^t \cdot U
$$

Instead of checking the entire encoding, we can do this statistically via columns:

$$
\text{Column}(\text{Enc}(L))_k = [ \textcircled{x}_{i=l_1}^n(1-r_i, r_i)]^t \cdot \text{Column}(U)_k
$$

Prover responds with the required Columns and their authentication paths from the Merkle tree to answer the Verifier's queries during this step.

Verificaiton time & proof size are both $\mathcal{O}(\sqrt{K})$, meaning sub-linear verifer and proof size. Asymptotically, its worse than $\mathcal{O}(\log^2(n))$ but in practice it is still efficient. Prover runs in linear time $\mathcal{O}(n)$.

> When Reed-Solomon encoding is used within **Brakedown**, we call that **Shockwave**.

## Brakedown using Binary Fields

Binius is achieved by using Brakedown over Binary Fields. A binary field is a field with characteristic 2, and is denoted by $\mathbb{F}_{2^m}$. The characteristic is the number of times you have to add 1 to itself to get 0. In binary fields, this is 2.

The simplest binary field is $\mathbb{F}_2$ which is just $\{0, 1\}$ with addition and multiplication that can be represented using XOR and AND respectively. Polynomials over this field have coefficients that are either 1 or 0.

We would like to have irreducible polynomials of a given degree to create extension fields. For example, to create a quadratic extension we need an irreducible polynomial of degree 2. The polynomial $x^2 + x + 1$ is irreducible over $\mathbb{F}_2$. Now, we can consider the polynomials $\mathbb{F}_2[x] / (1 + x + x^2)$ which is a field with 4 elements that are of form $a + bx$ with $a, b \in \mathbb{F}_2$.

### Representation with Bits

Since we are working with "bit" coefficients, it is often useful to represent the polynomials as binary numbers. For example, the polynomial $x^3 + x + 1$ can be represented as 1011 in binary.

- Even in this notation, **addition** can be shown using XOR.
- However, multiplication is not that straightforward because the degree may change. One can build a multiplication table as a precomputed table to do this efficiently.

### Higher Extensions

How can we go to higher extensions? There are two methods:

- **Direct Extension**: Simply find a polynomial of higher degree that is irreducible.
  For example, $\mathbb{F}_2[x] / (1 + x + x^3 + x^4 + x^8)$ is a degree-8 extension used within AES, and coefficients are 0 and 1 only.
- **Towers of Extension**: Find an irreducible polynomial within your extension field, and extend over that, again and again.
  - Start with $\mathbb{F}_2 \to \mathbb{F}_{2^2} = \mathbb{F}_2[x_0] / (1 +x_0 + x_0^2)$ with some degree-2 polynomial in the binary field.
  - Then, extend to $\mathbb{F}_{2^2} \to \mathbb{F}_{2^4} = \mathbb{F}_{2^2}[x_1] / (1 + x_1x_0 + x_1^2)$ with some degree-2 polynomial in the previous extended field.
  - Then, extend to $\mathbb{F}_{2^4} \to \mathbb{F}_{2^8} = \mathbb{F}_{2^4}[x_2] / (1 + x_2x_1 + x_2^2)$ with some degree-2 polynomial in the previous extended field.
  - and so on...

Notice that an element in $\mathbb{F}_{2^8}$ can be represented as:

$$
a_0 + a_1x_2 = (a_{00} + a_{01}x_1) + (a_{10} + a_{11}x_1)x_2
$$

and these look much similar to multilinear extensions we have been talking about.
