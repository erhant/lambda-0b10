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

The **greatest common divisor (GCD)** of two integers $a$ and $b$, denoted as $\text{gcd}(a, b)$, is the largest positive integer that divides both $a$ and $b$ without leaving a remainder. The GCD can be computed using the Euclidean algorithm, which is an efficient way to find the GCD of two numbers. The algorithm works by iteratively replacing the larger number with the remainder of the division of the two numbers until the remainder is zero. The last non-zero remainder is the GCD of the two numbers.

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

- Additive: $a \cdot b = a + b$
- Multiplicative: $a \cdot b = ab$

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

## Implementation

The most basic implementation of polynomials is by storing the list of coefficients, see for example <https://github.com/lambdaclass/lambdaworks/blob/bootcamp0b10/math/src/polynomial/mod.rs>.

To evaluate the polynomial at a point, we can use the Horner's method, which is an efficient way to evaluate polynomials. The method is based on the observation that a polynomial can be rewritten as a nested form:

$$
f(x) = a_n x^n + a_{n-1} x^{n-1} + \ldots + a_1 x + a_0 = ((a_n x + a_{n-1}) x + \ldots + a_1) x + a_0
$$

# RSA

RSA (Rivest-Shamir-Adleman) is a public-key cryptosystem that is widely used for secure data transmission. It is based on the difficulty of factoring large integers. The RSA algorithm involves the following steps:

pk e, sk d. n = p.q.

## Implementation

See <https://blog.lambdaclass.com/how-to-create-your-own-crappy-rsa-as-a-software-developer/>.

## Generating Primes

Primality Testing, Miller-Rabin, Carmichael Totient
