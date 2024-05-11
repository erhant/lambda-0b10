> # Week 1
>
> We talked about prime numbers, divisibility. Then we talked about groups, rings, and fields. We also talked about Montgomery form for field elements, and using limbs for that for better performance. In particular, we talked about Mersenne primes and MiniGoldilocks prime, and show how we can have efficient "modular reductions" there. The overall idea is that division & remainder is expensive and we would like to do as few as possible.

# Arithmetic

## Primes

Prime numbers are defined as positive integers greater than 1 that have no positive divisors other than 1 and themselves. In other words, a prime number is a number that cannot be formed by multiplying two smaller positive integers. Number 1 is not a prime, although sometimes it may be useful to consider it as a prime; not in our case anyways.

The [fundamental theorem of arithmetic](<(https://en.wikipedia.org/wiki/Fundamental_theorem_of_arithmetic)>), also called the unique factorization theorem and prime factorization theorem, states that every integer greater than 1 can be represented uniquely as a product of prime numbers, up to the order of the factors.

$$
n = p_1^{a_1} \cdot p_2^{a_2} \cdot \ldots \cdot p_k^{a_k}
$$

## Division & GCD

If a number $a$ divides another number $b$, then $a$ is called a divisor of $b$, and $b$ is called a multiple of $a$. The division of two integers $a$ and $b$ can be expressed as $a = b \cdot q + r$, where $q$ is the quotient and $r$ is the remainder. The remainder $r$ is always less than the divisor $b$.

The **greatest common divisor (GCD)** of two integers $a$ and $b$, denoted as $\text{gcd}(a, b)$, is the largest positive integer that divides both $a$ and $b$ without leaving a remainder. The GCD can be computed using the **Euclidean algorithm**, which is an efficient way to find the GCD of two numbers. The algorithm works by iteratively replacing the larger number with the remainder of the division of the two numbers until the remainder is zero. The last non-zero remainder is the GCD of the two numbers.

# Abstract Algebra

## Groups

A group is a set equipped with a binary operation that combines any two elements to form a third element, satisfying certain properties. A group is shown as $(G, \cdot)$ and consists of the following components:

- A set of elements $G$.
- A binary operation (denoted as $\cdot$) that takes two elements and produces a third element.

The operation must satisfy the following properties:

- **Closure**: For any two elements $a, b \in G$, the result of the operation is also in the group: $a \cdot b \in G$. It is said that the group $G$ is _closed_ under its binary operation.

- **Identity**: There exists an element $e \in G$, called the identity element, such that for any element $a \in G$, the operation $a \cdot e = e \cdot a = a$.

- **Inverse**: For every element $a \in G$, there exists an element $b \in G$, called the inverse of a, such that $a \cdot b = b \cdot a = e$. The inverse of $a$ is denoted as $a^{-1}$.

- **Associativity**: For any three elements $a, b, c \in G$, the operation is associative, meaning $(a \cdot b) \cdot c = a \cdot (b \cdot c)$. This property ensures that the order of operations does not matter.

There is an additional property as well, called the **commutative property** or **abelian property**. A group is said to be _Abelian_ if the binary operation is commutative, meaning $a \cdot b = b \cdot a$ for all elements $a, b \in G$.

If the group has a finite number of elements, it is called a **finite group**.

### Operation Notation

For the binary operation, we can use the additive notation or multiplicative notation.

- **Additive**: $a \cdot b = a + b$
- **Multiplicative**: $a \cdot b = ab$

### Examples

- The integers under addition $(\mathbb{Z}, +)$.
- The integers modulo $n$ under addition $(\mathbb{Z}_n, +)$.

## Rings

A ring is a set equipped with two binary operations, addition and multiplication, that satisfy certain axioms. A ring $(R, +, \times)$ consists of the following components:

- A set of elements $R$.
- An addition operation (denoted as $+$) that takes two elements and produces a third element.
- A multiplication operation (denoted as $\times$) that takes two elements and produces a third element.

The operations must satisfy the following properties:

- **Additive + Multiplicative Closure**: For any two elements $a, b \in R$, the result of the addition is also in the ring: $a + b \in R$ and the result of the multiplication is also in the ring: $a \times b \in R$. The ring $R$ is _closed_ under both addition and multiplication.

- **Additive + Multiplicative Associativity**: For any three elements $a, b, c \in R$, the addition and multiplication operations are associative, meaning $(a + b) + c = a + (b + c)$ and $(a \times b) \times c = a \times (b \times c)$. This property ensures that the order of operations does not matter.

- **Additive Identity**: There exists an element $0 \in R$, called the additive identity, such that for any element $a \in R$, the addition $a + 0 = 0 + a = a$. Nothing is said about multiplication yet.

- **Additive Inverse**: For every element $a \in R$, there exists an element $-a \in R$, called the additive inverse of $a$, such that $a + (-a) = (-a) + a = 0$. The inverse of $a$ is denoted as $-a$.

- **Addition Commutativity**: The addition operation is commutative, meaning $a + b = b + a$ for all elements $a, b \in R$.

- **Distributivity**: For any three elements $a, b, c \in R$, the ring satisfies the distributive property, meaning $a \times (b + c) = (a \times b) + (a \times c)$ and $(b + c) \times a = (b \times a) + (c \times a)$.

If the ring has a **multiplicative identity**, i.e., an element $e \in R$ such that $a \times e = e \times a = a$ for all $a \in R$, then the ring is called a _ring with unity_ and that element $e$ is called a **unity**.

If the multiplication is **commutative**, then the ring is called a **commutative ring**.

If the ring has a finite number of elements, it is called a **finite ring**.

### Examples

- The set $\mathbb{Z}$ of all integers, and is a commutative ring with unity.
- The set $\mathbb{Q}$ of all rational numbers.
- The set $\mathbb{R}$ of all real numbers.
- The set $\mathbb{C}$ of all complex numbers.

## Fields

A field is a ring $(F, +, \times)$ with the following properties:

- $F$ is a commutative ring.
- There is a non-zero unity $e \in F$.
- Every non-zero element $a \in F$ have a multiplicative inverse $a^{-1} \in F$ such that $a \times a^{-1} = a^{-1} \times a = e$.

If the field has a finite number of elements, it is called a **finite field**. The ring of integers modulo $p$, denoted as $\mathbb{Z}_p$, where $p$ is a prime number, is a finite field. This one is particularly important in cryptography!

> We can also say that a field is an "integral domain" where every non-zero element has a multiplicative inverse.

### Implementation

Doing division & modulo in a finite field is expensive, so we want to avoid it as much as possible. One way to do this is to use the **Montgomery form** for field elements. This form allows us to perform modular reductions more efficiently. See [Montgomery Arithmetic](https://eprint.iacr.org/2017/1057.pdf) for more details. Lambdaworks has a struct called `MontgomeryBackendPrimeField` for this, see <https://github.com/lambdaclass/lambdaworks/blob/main/math/src/field/fields/montgomery_backed_prime_fields.rs>.

# Polynomials

A polynomial is an expression consisting of variables (also called indeterminates) and coefficients, that involves only the operations of addition, subtraction, multiplication, and non-negative integer exponents. The general form of a polynomial is:

$$
f(x) = a_n x^n + a_{n-1} x^{n-1} + \ldots + a_1 x + a_0
$$

where $a_n, a_{n-1}, \ldots, a_1, a_0$ are the coefficients, $x$ is the variable, and $n$ is the degree of the polynomial. Polynomials are usually defined over rings, and the coefficients are elements of the ring. We are in particular interested in polynomials over finite fields of prime order.

### Implementation

The most basic implementation of polynomials is by storing the list of coefficients, see for example <https://github.com/lambdaclass/lambdaworks/blob/bootcamp0b10/math/src/polynomial/mod.rs>.

To evaluate the polynomial at a point, we can use the **Horner's method**, which is an efficient way to evaluate polynomials. The method is based on the observation that a polynomial can be rewritten as a nested form:

$$
f(x) = a_n x^n + a_{n-1} x^{n-1} + \ldots + a_1 x + a_0 = ((a_n x + a_{n-1}) x + \ldots + a_1) x + a_0
$$

## Evaluations

A degree $n$ polynomial can be represented by $n+1$ points. The polynomial interpolation problem is to find a polynomial of degree at most $n$ that passes through $n+1$ given points. **Lagrange interpolation** formula is a widely used method to find the polynomial that passes through the given points. In fact, there is a unique degree $n$ polynomial that passes through $n+1$ distinct points. This polynomial can be found using the Lagrange interpolation formula, which is based on the Lagrange basis polynomials.

## Shamir's Secret Sharing

Shamir's Secret Sharing is a cryptographic algorithm that allows a secret to be shared among a group of participants, such that only a subset of the participants can reconstruct the secret. The algorithm is based on polynomial interpolation, noting the fact that a degree $n-1$ polynomial can be uniquely determined by $n$ points.

The algorithm works as follows:

1. The secret is represented as a constant term of a polynomial of degree $t-1$.
2. A random polynomial of degree $t-1$ is generated, where the constant term is the secret.
3. The polynomial is evaluated at $n$ distinct points such that $n > t$, and the evaluations are shared among the participants.
4. To reconstruct the secret, at least $t$ participants combine their shares and interpolate the polynomial to find the secret.

The security of Shamir's Secret Sharing is based on the fact that any subset of less than $t$ participants does not have enough information to reconstruct the polynomial and hence the secret. Note that this security is not a computational security, but rather an information-theoretic security; meaning that there is absolutely no way to recover the secret without the required number of shares.

LambdaWorks has a struct called `ShamirSecretSharing` for this, see <https://github.com/lambdaclass/lambdaworks/tree/main/examples/shamir_secret_sharing>.

## Reed-Solomon Codes

Reed-Solomon codes are a type of error-correcting code that is widely used in digital communication and storage systems. Consider a message $m=(m_1, m_2, \ldots, m_k)$ of length $k$ that needs to be transmitted over a noisy channel. Reed-Solomon codes encode the message into a longer codeword $c=(c_1, c_2, \ldots, c_n)$ of length $n > k$ such that the original message can be recovered even if some of the codeword symbols are corrupted.

The encoding process involves generating a polynomial of degree $k-1$ from the message symbols and evaluating the polynomial at $n$ distinct points to obtain the codeword symbols. The decoding process involves interpolating the polynomial from the received codeword symbols to recover the original message.

> One can think of Reed-Solomon as a Shamir's Secret Sharing scheme where the secret is the message to be transmitted, and the shares are the codeword symbols. However, all the shares are sent over the network so that if some of them are lost or corrupted, the original message can still be recovered.

### Distance

The **minimum distance** of a code is the minimum number of positions in which any two codewords differ. It is a measure of the error-correcting capability of the code.

Error correcting codes have a measure of **distance** that is usually computed as the Hamming distance, which is the number of positions at which the corresponding symbols of two codewords are different. For instance:

```py
#   ✓  x  x  ✓  ✓  ✓
m1: A  B  C  D  E  F
m2: A  A  D  D  E  F
```

These two messages have a Hamming distance of 2, because the second and third symbols are different. Reed-Solomon codes have a minimum distance of $n-k+1$, which means they can correct up to $(n-k)/2$ errors. The idea is that if the errors are less than or equal to $(n-k)/2$, then the decoder can correct them by finding the closest codeword to the received word.

# RSA

RSA (Rivest-Shamir-Adleman) is a public-key cryptosystem that is widely used for secure data transmission. It is based on the difficulty of factoring large integers. The RSA algorithm involves the following steps:

- Choose two large prime numbers $p$ and $q$, and then compute $n = p \times q$.

- Compute the **Euler totient function** $\phi(n) = (p-1) \times (q-1)$. The Euler totient function $\phi(n)$ counts the number of positive integers less than $n$ that are coprime to $n$. It has a nice property that when $n$ is a prime number, $\phi(n) = n-1$. Furthermore, it is a multiplicative function, meaning that $\phi(ab) = \phi(a) \times \phi(b)$ if $a$ and $b$ are coprime. Thats how we get $(p-1)\times(q-1)$ quite easily.

> One can also use the **Carmichael totient function** $\lambda(n)$, which is the smallest positive integer such that $a^{\lambda(n)} \equiv 1 \mod n$ for all $a$ coprime to $n$. The Carmichael function is always less than or equal to the Euler totient function, and is often used in RSA for more efficient implementations.

- Choose an integer $e$ such that $1 < e < \phi(n)$ and $e$ is coprime to $\phi(n)$. The number $e$ is the public exponent. Some choices for $e$ are $3, 17$ and $65537$. Note that these are equal to $2^1+1, 2^4+1$ and $2^{16}+1$ respectively.

> Choosing a small $e$ makes encryption faster, but is also susceptible to attacks. Most often we choose $e=65537$. For example, when $e=3$ it may be the case that $m^3$ is not so large, and is still within the modulus, so one can simply find the cube-root of the ciphertext to recover the plaintext.

- Compute the private exponent $d$ such that $d \equiv e^{-1} \mod \phi(n)$. The number $d$ is the private exponent.

- The **public key** is $(n, e)$, and the **private key** is $d$.

To encrypt a message $m \in \mathbb{Z}_n$, the sender uses the public key $(n, e)$ to compute the ciphertext $c = m^e \mod n$. To decrypt the ciphertext, the receiver uses the private key $d$ to compute the plaintext $m = c^d \mod n$.

This works because $m^{e \times d} \equiv m \mod n$ by Euler's theorem. The security of RSA is based on the difficulty of factoring the product $n = p \times q$.

## Generating Primes

When the prime is not large, one can simply check if it is prime by trial division. However, when the prime is large, this method is not efficient. One way to generate large prime numbers is to use the **Miller-Rabin primality test**, which is a probabilistic algorithm that can determine whether a number is prime with high probability. The algorithm works by repeatedly testing the primality of a number using a set of random bases.

## Implementation

See <https://blog.lambdaclass.com/how-to-create-your-own-crappy-rsa-as-a-software-developer/>.
