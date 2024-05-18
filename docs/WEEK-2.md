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
p + 1 + t
$$

where $|t| \le 2\sqrt{p}$ (Hasse's bound), $r$ is the number of points in the curve and $p$ is the field size. It is important that $r$ is large and prime.

> It is generally not easy to find the number of points in the curve. Sometimes you have "families of curves" and there you may have a formula to calculate the number of points. See for example BN254 curve.

In the best case, we would like to number of points on the curve to some large prime number. However, we are still okay with large numbers with some large prime factor.

### Trace of Frobenius

TODO: explain

> Pasta curves are quite interesting, i.e. the two curves Pallas and Vesta. Both curves are defined over the equation $y^2 = x^3 + 5$.
>
> - Pallas curve is defined over $\mathbb{F}_p$ base field, and has $r$ points.
> - Vesta curve is defined over $\mathbb{F}_{r}$ extension field, and has $p$ points.
>
> Mina Protocol uses these curves for efficient verification! Similarly, Nova folding scheme uses these curves for efficient verification.

## Generator Point

In a prime order group, we would like to find a generator element $g \in E$ such that:

$$
\{0g, g, 2g, 3g, \ldots, (r-1)g\} = E
$$

TODO: how many generators are there?

In groups with non-prime order but with a large prime factor, we instead go for a generator point $g$ that generates the large prime order subgroup, not the entire group!

So, to make sure we have a safe generator point, we need to make sure that:

- The generator is within the curve
- The generator generates the large prime order subgroup, meaning that its order is equal to the large prime factor!

### Pohlig-Hellman Attack

What happens if we pick a generator $g'$ that generates the entire curve instead? Meaning that its order is $n = r \times h$ where $r$ is some prime (not necessarily the largest) and $h$ is a cofactor.

$g'$ has order $n$, and $h \times g'$ has order $r$ (i.e. cofactor clearing). With that, you can find the modulo of a secret key within that small subgroup (which is much easier) and then reveal parts of the secret key.

Using the small subgroups, you can find the secret key $d$ modulo $r$ for many factors of $n=r_1, r_2, \ldots, r_k$ and then use the **Chinese Remainder Theorem** to find the secret key $d$ modulo $n$.

> This attack was used in several Capture-the-Flag events, such as ZKHACK or Lambda-Ingonyama ZK-CTF. In these challenges, there was either a faulty generator thats in the wrong subgroup, or something that leaked information about the discrete log, enabling the Chinese Remainder Theorem to take place in the attack.

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

# Commitments

Commitments are a way to **commit** to a value without revealing it; think of it like having a piece of data and putting the data inside an envelope. This is useful in many cryptographic protocols. A cryptographic commitment scheme has two important properties:

- **Hiding**: The commitment should hide the value $m$, one cannot know what is committed just by looking at the commitment.
- **Binding**: The commitment should bind the value $m$ to the commitment $C$.

## Hash Functions

A cryptographic hash function is a **one-way function**, they are hard to invert! This means that given $h \gets H(m)$ it is hard to find $m$ just by looking at $h$, so $m \gets H^{-1}(h)$ is infeasible.

> SHA-2 is based on Merkle-Damgard construction, which uses a Compression function. Merkle-Damgard construction has a length-extension attack.

> SHA-3 is based on Sponge construction, which has an "absorb" step and a "squeeze" step. It begins by absorbing the input, and then squeezing the sponge results in bits of the hash.

A hash function can be used within a commitment scheme.

## Merkle Trees

A Merkle Tree is a method of comitting to a vector of values. Consider $\vec{a} = (a_0, a_1, \ldots, a_{n-1})$ where $n=2^m$, we can commit to this vector by creating a tree of hashes. The leaves of the tree are the values $a_i$, and the internal nodes are the hashes of their children.

We can use any cryptographic hash function within our Merkle Tree, but most people use SHA-2, SHA-3, Blake2, or Blake3; there are mostly based on bitwise operations. Within the zero-knowledge space, people use more "circuit-friendly" hashes such as Poseidon, Monolith, and Rescue; these are mostly based on Algebraic operations.

When we create a binary tree of hashes, we can commit to a value by revealing the root of the tree. This is a commitment to the entire vector of values, also denoted as the **Merkle Root**.

In particular, we will use the Merkle Trees as a way of committing to polynomials! Consider a polynomial with coefficients $(a_0, a_1, \ldots, a_{n-1})$, we can commit to this polynomial by creating a Merkle Tree from this list of values, treated as a vector. Using this, we will actually be able to build a **polynomial commitment scheme**. In particular, we would like to prove evaluations of a committed polynomial.

## Using Elliptic Curves for Commitments

Now, we look at a commitment scheme known as KZG (Kate-Zaverucha-Goldberg) commitment scheme. Consider an elliptic curve $E$ with prime order.

One way of committing to a polynomial $P$ would be to evaluate the polynomial at a point $s$ to obtain $P(s)$, and then commit to the evaluation using a generator $g$ by doing $P(s)g$. The resulting commitment is just a point in the curve.

If you were the one who received the commitment, you would have to solve discrete-log to find out the polynomial, but that is hard. This is a **hiding** commitment scheme. However, this is not binding, you could simply pick the constant polynomial $Q(x) = P(s)$.

> TODO: Check Pedersen hashes & inner-product arguments (IPA)

Is there are a way to commit without knowing $s$? Yes! Imagine a set of points like:

$$
\{s^0g, s^1g, s^2g, s^3g, \ldots, s^{n-1}g\}
$$

This is basically a set of points:

$$
\{P_0, P_1, \ldot,s P_{n-1}\}
$$

We refer to this as a **Structured Reference String** (SRS). Now, you can do the following:

$$
P(S)g = \sum a_iP_i = a_0P_0 + a_1P_1 + a_2P_2 + \ldots + a_{n-1}P_{n-1}
$$

So, no need to know what $s$ is to evaluate the polynomial at that point! Notice that given any $P_i = s^ig$, you cant find $s$ thanks to discrete-log. This operation is called **Multi-Scalar Multiplication** (MSM) and is the main bottleneck within the zk-SNARKs. One of the most efficient algorithms on this is called the **Pippenger's Algorithm**.

> TODO: check "Security in the algebraic model".

> In one CTF, the trick was to look at the SRS and see that the points were repeating from some point on! There, $s$ belonged to a small order subgroup.

> TODO: MOV Attack and Cheon's Attack in Pairings

Thanks to this new method, we now have a commitment scheme that is both hiding and binding. We have computed $P_s = P(s)g$ without knowing $s$ and we can't change the polynomial without knowing $s$ to break the binding property.

## Pairings

So imagine I have a commitment $\text{commit}(P) = P_s = P(s)g$ (computed with MSM). Now, I want to show you $P(z) = v$ for some $z$. What I will do is make use of the property:

$$
P(x) - v = (x-z)Q(x)
$$

What I will do is to send you $z$, the value $v$ along with an evaluation proof. For that, I will give you a commitment $\text{commit}(Q) = Q_s$. We will need **Pairing** for this part!

Pairing is a bilinear map $e : G_1 \times G_2 \to G_T$ that takes two inputs and returns a new element. The pairing that we will use (a type-3 pairing) has the property:

$$
e(ag_1, bg_2) = e(g_1, g_2)^{ab}
$$

This is a bilinear map, meaning that it is linear in both arguments. This is a very useful property for zero-knowledge proofs. A bilinear pairing has the following properties:

- $e(g_1, g_2) \ne 1$ (non-degenerate)
- $e(g_1 + g_3, g_2) = e(g_1, g_2) e(g_3, g_2)$

Now, notice that:

$$
e(P_s, g_2) = e(P(s)g_1, g_2) = e(g_1, g_2)^{P(s)}
$$

Notice that non-degenerancy is helpful here because $e(g_1, g_2) \ne 1$, so we have some non-identity element that we are raising to some power.

What we then do is:

$$
e(Q_s, sg_2 - zg_2) = e(Q(s)g_1, (s-z)g_2) = e(g_1, g_2)^{Q(s)(s-z)}
$$

$$
e(P_s - vg_1, g_2) = e((P(s) - v)g_1, g_2) = e(g_1, g_2)^{P(x) - v}
$$

We can compare these two pairings, and check if they are equal. This works over a random point thanks to the Schwartz-Zippel Lemma.
