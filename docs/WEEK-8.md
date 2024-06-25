> # Week 8
>
> [Plookup](https://eprint.iacr.org/2020/315.pdf) & zkVMs.

# Lookup Argument

Remember the gate constraint:

$$
q_La + q_Rb + q_Mab + q_Oc + q_C + PI = 0
$$

From this, we had interpolated polynomials like $q_L(x), q_R(x), q_M(x), q_O(x), q_C(x)$ as well as $a(x), b(x)$ and $c(x)$. We also had the ability to capture more complex computations with **custom gates**.

Consider the XOR example over `u16` (16-bit unsigned) integers, i.e. $a \oplus b = c$. So the constraints here is that $a, b$ must be in range, and $c$ should be result of the XOR operation (which is kinda complex in finite field).

For this, we can instead compute everything beforehand and create a **lookup table**. For example, consider a table $T$ with three columns such that $T_0$ has 16-bit values for $a$, $T_1$ has 16-bit values for $b$ and $T_2$ has 16-bit values for $c$. What we want is to show that at some point the trace $a(x), b(x), c(x)$ have a corresponding "row" in this table.

We do that using a **lookup gate**. We will describe a selector for this lookup gate $q_{LU}(x)$. Let's dive in.

## Compressing the Columns

Sample random $\zeta$ from verifier, and compress the columns of the table via a random linear combination:

$$
t_i = T_{0i} + \zeta T_{1i} + \zeta^2 T_{2i}
$$

We can also define a linear combination for the trace columns:

$$
f_i = a_i + \zeta b_i + \zeta^2 c_i
$$

The problem reduces to showing that $\{f_i\} \subset \{t_i\}$, i.e. the values in $f_i$ shall appear in values of $t_i$. It is okay if $f_i$ has some duplicate values, or that it has some values missing from $t_i$.

Here is the idea, if for a set of values $\{a_i\}, \{b_i\}$ the following holds:

$$
\prod(x + a_i) = \prod(x + b_i)
$$

then $\{a_i\}$ is a permutation of $\{b_i\}$. This is because the polynomial on the left has roots at $a_i$, and the polynomial on the right has roots at $b_i$. If the polynomials are equal, then the roots must be equal. There is a problem with **multiplicities** though, what if the same values has been used many times?

> [!NOTE]
>
> There is an [alternative method](https://eprint.iacr.org/2022/1530.pdf) using the derivate of this as well:
>
> $$
> \sum \frac{1}{x + a_i} = \sum \frac{m_j}{x + b_j}
> $$

We will now work with randomized differences, let $s = (f, t)$, sorted by $t$. Then, compute:

- $\Delta s = s_i + \delta s_{i+1}$
- $\Delta t = t_i + \delta t_{i+1}$

## Protocol

1. Compress columns of $T$ to obtain $t_i$ as:

$$
t_i = T_{0i} + \zeta T_{1i} + \zeta^2 T_{2i}
$$

> Here, $\zeta$ is provided by the verifier.

2. Construct the "query wire", which are just the values $f_i$. There are two cases though, one when we are doing a lookup and one when we are not.

$$
f_i =
\begin{cases}
a_i + \zeta b_i + \zeta^2 c_i & \text{if lookup} \\
T_{0,n} + \zeta T_{1,n} + \zeta^2 T_{2,n} & \text{if not lookup}
\end{cases}
$$

> The "not lookup" case is simply to fill some dummy values that are included within the table anyways. In the example above, we simply picked the last ($n$-th) row of the table.

3. Create sorted vector $s$ by combining the vectors $f, t$.

- $f = (f_0, f_1, \ldots, f_n)$
- $t = (t_0, t_1, \ldots, t_n)$
- $s = (f, t)$ concatenated & sorted with respect to $t$

> An example: $t = (1, 5, 7, 2, 4, 8)$ and $f = (1, 4, 4, 1, 8, 7)$.
>
> When we build $s$ just by concatenating these two vectors, we get $s = (1, 4, 4, 1, 8, 7, 1, 5, 7, 2, 4, 8)$.
>
> After sorting with respect to $t$, we get $s = (1, 1, 1, 5, 7, 7, 2, 4, 4, 4, 8, 8)$.
>
> This helps when we are computing the differences. When a difference occurs in the sorted $s$, and we compute $\Delta s$, we will get some factors like $(1 + \delta)s_i$.

4. Compute deltas:

$$
\Delta t_i = \begin{cases}
t_i + \delta t_{i+1} & \text{if } i \in \{0, 1, \ldots, n-1\} \\
t_i + \delta t_0 & \text{if } i = n \text{ to wrap around}
\end{cases}
$$

$$
\Delta s_i = \begin{cases}
s_i + \delta s_{i+1} & \text{if } i \in \{0, 1, \ldots, n-1\} \\
s_i + \delta s_0 & \text{if } i = n \text{ to wrap around}
\end{cases}
$$

Note that $s_i$ is twice the length of $t_i$. For that reason, we will split $\Delta s_i$ into two parts and compute $\Delta s_i$ for both of them:

- $h_1 = (s_0, s_1, \ldots, s_n)$
- $h_2 = (s_{n+1}, s_{n+2}, \ldots, s_{2n+1})$

Now, let $\omega$ be a primitive root of unity of order $n$ (as we used to interpolate the trace polynomial), we will create the following polynomial:

$$
Z(\omega x) = Z(x) \cdot \frac{
  (1+\delta)(\epsilon + f(x))(\epsilon(1 + \delta) + t(x) + \delta t(\omega x))
}{
 (\epsilon(1 + \delta) + h_1(x) + \delta h_1(\omega x))(\epsilon(1 + \delta) + h_2(x) + \delta h_2(\omega x))
}
$$

This is similar to the permutation argument we had seen in PlonK. What it proves is that $(f, t)$ is a permutation of the sorted $s$.

$$
Z(\omega x) = Z(x) \cdot \frac{
  (1+\delta)(\epsilon + f(x))(\epsilon(1 + \delta) + \Delta t(x))
}{
 (\epsilon(1 + \delta) + \Delta h_1 (x))(\epsilon(1 + \delta) + \Delta h_2(x))
}
$$

> Another way to view this polynomial is to see that:
>
> $$
> Z_{i+1} = Z_i \cdot \frac{
>   (1+\delta)(\epsilon + f_i)(\epsilon(1 + \delta) + \Delta t_i)
> }{
>  (\epsilon(1 + \delta) + \Delta h_{1,i})(\epsilon(1 + \delta) + \Delta h_{2,i})
> }
> $$

We can split $s$ into $h_1, h_2$ in an even-odd order, and rewrite the equation above as below, changing one instance of $h_1$ with $h_2$:

$$
Z(\omega x) = Z(x) \cdot \frac{
  (1+\delta)(\epsilon + f(x))(\epsilon(1 + \delta) + t(x) + \delta t(\omega x))
}{
 (\epsilon(1 + \delta) + h_1(x) + \delta h_2(\omega x))(\epsilon(1 + \delta) + h_1(x) + \delta h_2(\omega x))
}
$$

Let's revisit our polynomials then:

- **Selector Polynomials**: The lookup selector $q_{LU}(x)$ is designed such that $q_{KU}(\omega^k)$ is 1 if the $k$-th gate is a lookup gate, or 0 if it is not.

- **Permutation Polynomials**: $S_{\sigma_0}(x), S_{\sigma_1}(x), S_{\sigma_2}(x)$ along with $S_{ID_0}(x) = x, S_{ID_2}(x) = k_1x$, polys x, k_1, and k_2x (out of domain ks)

- **Table Polynomials**: $T_0(x), T_1(x), T_2(x)$ will simply interpolate the columns of the table.

Finally, show that:

1. Gate constraints hold.
2. Lookup constraints hold: $q_{LU, i} \cdot (a_i + \zeta b_i + \zeta^2 c_i) = 0$
3. Copy constraints hold
4. Values of $f_i$ are contained in $t_i$

## Revisiting the Protocol

- **Round 1**: Compute the blinded $a(x), b(x), c(x)$ is the same as before, and obtain $\boxed{a}, \boxed{b}, \boxed{c}$.
- **Round 2**: Compute the compressed table & do the sorting using $f_i, t_i$. The table is precomputed, but the compression requires a public input from the verifier (i.e. the transcript). Once the sorted set is computed, we end up with $\boxed{f}, \boxed{h_1}, \boxed{h_2}$. Here these polynomials have blinding factors as well.
- **Round 3**: Compute the permutation arguments, with some additional randomness & blindings. Now we will capture both the copy constraints for $a, b, c$ and the permutations for the lookups $f, t$. We end up with $\boxed{Z_1}, \boxed{Z_2}$, one for each respectively.
- **Round 4**: Compute the quotient polynomials, showing that the constraints hold and all.
- **Round 5**: Comptute evaluations on the random sampled point, for each polynomial.
- **Round 6**: Create the opening proof for the evaluations.

> At this point there were too many equations to write down, so we end here...

## See Also

- [Plookup](https://eprint.iacr.org/2020/315.pdf)
- [PlonKup](https://eprint.iacr.org/2022/086)
