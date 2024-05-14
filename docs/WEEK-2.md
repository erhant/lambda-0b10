> # Week 2
>
> We talked about Elliptic Curves, mostly on Short Weierstrass Curve and its operations.

# Elliptic Curves: Short Weierstrass

There are several forms of elliptic curve definitions. We will describe the most notable one, that is the Short Weierstrass form. The elliptic curve is defined by the set of pairs $(x, y) \in \mathbb{F}_p \times \mathbb{F}_p$ that satisfy the following curve equation:

$$
y^2 = x^3 + ax + b
$$

where $4a^3 + 27b^2 \ne 0$. Notice that this means we are looking for $x$ where $x^3 + ax + b$ is a square (also denoted as **Quadratic Residue**).

> Diophantine worked on finding rational points on these curves! TODO: add ref

## Point Addition

Elliptic curves form a group under the operation of point addition that is $+:E\times E \to E$. The identity element is the point at infinity $\mathcal{O}$ and we kind of add this as an _extra_ point.

**Addition / Chord Rule**: To add two points $P, Q$ you take a straight line through the two points, which will intersect the curve at a third point $R$. The sum of $P + Q$ is the reflection of $R$ over the x-axis. Given, $P= (x_1, y_1)$ and $Q = (x_2, y_2)$, the sum $P + Q = (x_3, y_3)$ is given by:

$$
\begin{align*}
s &= \frac{y_2 - y_1}{x_2 - x_1} \\
x_3 &= s^2 - x_1 - x_2 \\
y_3 &= s(x_1 - x_3) - y_1 \\
\end{align*}
$$

**Doubling / Tangent Rule**: There is the case where $P = Q$, in this case we take the tangent line at $P$ and find the intersection point $R$. Given, $P = (x_1, y_1)$ the double $2P = (x_3, y_3)$ is given by the same formula, but with a different slope $s$:

$$
\begin{align*}
s &= \frac{3x_1^2 + a}{2y_1} \\
x_3 &= s^2 - 2x_1 \\
y_3 &= s(x_1 - x_3) - y_1 \\
\end{align*}
$$

> Notice that the formula is a bit different when $P = Q$ because the slope is different. **Twisted Edwards** curves have a simpler formula for such a case, both chord and tangent rule are the same!

### Point Inversion

Given a point $P = (x, y)$ the inverse is given by $-P = (x, -y)$, that is the reflection over the x-axis. As such, $P - P = \mathcal{O}$.

### Scalar Multiplication

We can add a point to itself multiple times, this is called scalar multiplication. Given a point $P$ and a scalar $a$, we can compute $aP$ by adding $P$ to itself $a$ times. We use the efficient "double-and-add" algorithm for this. Notice that this is called "square-and-multiply" in multiplicative operations. For example, $5P = 2(2P) + P$ which uses 2 "doubles" and one addition.

## Number of Points

It is quite important to know how many points there are on the curve $r$. The number of points has a bound that is given by the Hasse's theorem:

$$
|p - r| \le \sqrt{p}
$$

where $r$ is the number of points in the curve and $p$ is the field size. It is important that $r$ is large and prime.

> It is generally not easy to find the number of points in the curve. Sometimes you have "families of curves" and there you may have a formula to calculate the number of points. See for example BN254 curve.

### Trace of Frobenius

TODO: explain

> Pasta curves are quite interesting, i.e. the two curves Pallas and Vesta. Both curves are defined over the equation $y^2 = x^3 + 5$.
>
> - Pallas curve is defined over $\mathbb{F}_p$ base field, and has $r$ points.
> - Vesta curve is defined over $\mathbb{F}_{r}$ extension field, and has $p$ points.
>
> Mina Protocol uses these curves for efficient verification! Similarly, Nova folding scheme uses these curves for efficient verification.

## Generator Point

We would like to find a generator element $g \in E$ such that:

$$
\{0g, g, 2g, 3g, \ldots, (r-1)g\} = E
$$

TODO: how many generators are there?

## Point Representations

When we store points in the curve, we usually store them in the **Affine** form. This is the form $(x, y)$ where $x, y \in \mathbb{F}_p$. However, this is not the most efficient way to store points.

Consider addition like $P+Q+R+S+\ldots$, and we compute a slope $s$ in each addition. This slope has a division, so we need to compute the multiplicative inverse of a field element, e.g. using Extended Euclidean Algorithm.

As an alternative, we can use the **Projective** form (Homogeneous projective coordinates) to store points. This is the form $(X, Y, Z)$ where $X, Y, Z \in \mathbb{F}_p$ and $Z \ne 0$. The point $(x, y)$ is represented as $(x, y, 1)$. To go from projective to affine, you can simply do $(X : Y : Z) \to (X/Z, Y/Z)$.

In projective coordinates, you can add points without doing field inversions. The formulas are a bit more complex, but they are more efficient.

There is also the **Jacobian** form, which is a bit more efficient than projective. This is the form $(X, Y, Z)$ where $X, Y, Z \in \mathbb{F}_p$ and $Z \ne 0$. The point $(x, y)$ is represented as $(x, y, 1)$. To go from jacobian to affine, you can simply do $(X : Y : Z) \to (X/Z^2, Y/Z^3)$.

There are many more representations, each with different levels of efficiency.

# Elliptic Curve Cryptography

The public key in Elliptic Curve Cryptography is derived using scalar multiplication. Given a private key $d$ and a base point $G$, the public key is $Q = dG$. This is based on the assumption that the discrete logarithm problem is hard to solve, i.e. given $Q = dG$ and $G$, it is hard to find $d$.

The best algorithms to solve Discrete Logarithm are **Pollard's Rho** and **Baby-Step Giant-Step**. They run in time $\mathcal{O}(\sqrt{r})$ where $r$ is the number of points in the curve. For this reason, the level of security is given by the number of bits in $\sqrt{r}$. For example, $r \approx 2^{256}$ gives a security level of 128 bits.

> BN254 was initially though to have 128 bits of security, but it was later subject to more clever attacks that reduced the security level to ~100 bits. TODO: reference

> In many cases $a = 0$ is picked in the curve, which simplifies the formulas and makes operations a bit more efficient. TODO: give example curves

## Diffie-Hellman Key Exchange

The Diffie-Hellman key exchange is a protocol that allows two parties to agree on a shared secret key over an insecure channel. The protocol is based on the hardness of the discrete logarithm problem.

Alice and Bob would like to agree on a key, but first they have to "exchange" this key securely. They do this by exchanging public keys and then computing the shared secret key.

1. Alice and Bob agree on a curve $E$ over a **base field** $\mathbb{F}_p$ and a generator point $G$. This curve has $r$ points, meaning that its **scalar field** is $\mathbb{F}_r$.

1. Alice picks a private key $a \in \mathbb{F}_r$ and computes the public key $A = aG$. Send this to Bob.

1. Bob picks a private key $b \in \mathbb{F}_r$ and computes the public key $B = bG$. Send this to Alice.

1. Alice computes the shared secret key $S = aB = a(bG)$.

1. Bob computes the shared secret key $S = bA = b(aG)$.

1. Et viola, the shared secret key is the same because $aB = bA = abG$. No one can break this because it is hard to find the discrete log!

1. Now, they can derive the symmetric key they like using a key derivation function (KDF) using the secret $(ab)G$.

This is good an all, but it is not _authenticated_. This means that an attacker could intercept the public keys and replace them with their own. This is called a **Man in the Middle** attack.

## Digital Signatures

ECDSA, Schnorr signatures and BLS signatures all are defined using an elliptic curve.

> Signatures can be stored efficiently as well. For example, a signature is a curve point $(x, y)$, but you can only store $x$ as well, because $y$ can be derived from $x$ by taking the square of curve equation's $x$-side. A single extra bit to indicate the positive / negative solution is enough to store the signature.

## Implementation

LambdaWorks have quite a lot of implementations for elliptic curves. See <https://github.com/lambdaclass/lambdaworks/tree/main/math/src/elliptic_curve>. They use the projective form for efficiency within their operations, and they allow conversion to affine form if needed.
