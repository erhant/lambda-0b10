# Number Theoretic Transform (NTT)

More specifically, here we implement radix-2 fast Number Theoretic Transform (NTT) and its inverse (INTT). The NTT of a vector of $n$ polynomial coefficients $a_0, a_1, \ldots, a_{n-1}$ is defined as $n$ new coefficients $A_0, A_1, \ldots, A_{n-1}$ such that:

$$
A_j = \sum_{i=0}^{n-1}  a_j \cdot \omega^{ij}
$$

where $n$ is a power of 2, $\omega$ is a primitive $n$-th root of unity in the finite field $\mathbb{F}_p$, and all operations are defined over the field $\pmod p$. Notice that $A_j$ is equal to the polynomial evaluated at $\omega^j$.

> [!NOTE]
>
> A primitive $n$-th root of unity is an element $\omega$ such that $\omega^n = 1$ and $\omega^j \ne 1$ for all $0 < j < n$ that divides $n$.

In the more explicit form:

$$
A_j = a_0 \cdot \omega^{0j} + a_1 \cdot \omega^{1j} + \ldots + a_{n-1} \cdot \omega^{(n-1)j}
$$

The trick is to split the polynomial into two parts: the even coefficients and the odd coefficients. This is done by using the following relations:

$$
\begin{align*}
A_j = &a_0 \cdot \omega^{0j} + a_2 \cdot \omega^{2j} + \ldots + a_{n-2} \cdot \omega^{(n-2)j} &\text{(even terms)}\\
    + &a_1 \cdot \omega^{1j} + a_3 \cdot \omega^{3j} + \ldots + a_{n-1} \cdot \omega^{(n-1)j} &\text{(odd terms)}
\end{align*}
$$

Now notice that odd terms have a common factor of $\omega^j$ below:

$$
\begin{align*}
A_j = &a_0 \cdot \omega^{0j} + a_2 \cdot \omega^{2j} + \ldots + a_{n-2} \cdot \omega^{(n-2)j}\\
    + w^j (&a_1 \cdot \omega^{0j} + a_3 \cdot \omega^{2j} + \ldots + a_{n-1} \cdot \omega^{(n-2)j})
\end{align*}
$$

Now let us substitue $\gamma = \omega^2$ and re-write the above equation:

$$
\begin{align*}
A_j = &a_0 \cdot \gamma^{0j} + a_2 \cdot \gamma^{1j} + \ldots + a_{n-2} \cdot \gamma^{(n/2-1)j}\\
    + w^j(&a_1 \cdot \gamma^{0j} + a_3 \cdot \gamma^{1j} + \ldots + a_{n-1} \cdot \gamma^{(n/2-1)j})
\end{align*}
$$

What we ended up with is two smaller NTTs of size $n/2$ each! This is the basis of the (radix-2) Cooley-Tukey algorithm for fast Fourier transform. The algorithm is recursive and has a time complexity of $O(n \log n)$.

> [!NOTE]
>
> If $\omega$ is a primitive $n$-th root of unity, then $\omega^2$ is a primitive $n/2$-th root of unity. So this substitution does not break that property.

Primitive $n$-th roots of unity have two properties that are useful for the NTT:

- **Periodicity**: $\omega^{j + n} = \omega^j$ for all $j$. It is quite evident why this is:

$$
\omega^{j + n} = \omega^j \cdot \omega^n = \omega^j \cdot 1 = \omega^j
$$

- **Symmetricity**: $\omega^{j + n/2} = -\omega^j$ for all $j$. (TODO: explain why)

We end up with the following "butterfly" operation:

$$
\begin{align*}
A_j       &= E_j + \omega^j O_j \\
A_{j+n/2} &= E_j - \omega^j O_j
\end{align*}
$$

where $E_j$ is the even part and $O_j$ is the odd part of $A_j$, as depicted above. Here, $\omega^j$ is also called the "twiddle factor".
