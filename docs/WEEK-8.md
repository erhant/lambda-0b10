> # Week 8
>
> Plookup & zkVMs.

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

### See Also

- [Plookup](https://eprint.iacr.org/2020/315.pdf)
- [PlonKup](https://eprint.iacr.org/2022/086)

# Zero-Knowledge Virtual Machines (zkVM)

A zero-knowledge virtual machine is a virtual machine that can execute programs such that a zero-knowledge proof can be created about their execution. The idea is to have a virtual machine that can execute programs, but the verifier does not know what the program is doing. The verifier only knows that the program has been executed correctly.

## In the Wild

There are quite a variety of zkVMs with different designs:

- **Cairo**: it was the first VM among all. It is not as fancy as the new ones, but it opened the path for the rest.
- **RISC0**: it is a RISC-V based VM, able to prove Rust code
- **Valida/SP1**: ...
- **Miden**: has its own assembly instruction set.
- **Nexus**: ...
- **Ola**: ...
- **Jolt**: ...

### Architecture

There are two main **architectures** in any kind of VM:

- **Von-Neumann**: The operations and instruction are together in the same piece of memory
- **Harvard**: The operations and instructions are separated

### Proof Systems

The most important thing about a zkVM is the proof system that it uses. We have several options:

- **STARKs** (Cairo, Risc0, Miden)
- **Plonky3** (Valida/SP1)
- **Plonky2** + **STARKY** (Ola)
- **Lasso** (Jolt)
- **Nexus** (Elliptic Curve snarks)

The choice of proof system will affect three main things:

- **Proof Size**: we want small proofs
- **Speed**: we want the VM to be fast
- **Recursion**: we want recursion to be easy

In general, most VMs seem to prefer STARKs because it easier to do recursion & you can choose your field; its also post-quantum secure! The large proof size is a problem there, but you can do a final proving step with a much smaller (constant-size) proof to solve that.

### Instruction Set

Many VMs have a _proof compression_ step that generates one final proof from a multiple of them.

We should also think of the Instruction Set Architecture (ISA) of the VM. The ISA is the set of instructions that the VM can execute. The number of instructions is important, as well as the complexity of the instructions.

- **RISC-V** (RISC0)
- **Cairo** (Cairo)
- **Miden Assembly** (Miden)
- **LLVM** + own ISA (Valida)

### Modularity

A VM can be Modular or Monolithic:

- A modular zkVM is one that can be split into different parts, and each part can be replaced by another one; it is easier to extend such a zkVM with new operations. The communication between the different modules of the VM is often done via lookup arguments.

- A monolithic zkVM however, is one that is a single piece of code.

Some modularity examples:

- In Valida/SP1, we have a specialized Arithmetic Logic Unit (ALU) "copressor" that does specialized arithmetic operations. This is separated from the main CPU of the zkVM, all you have to do is communicate between them.
- In Miden, we have "chiplets".
- In Cairo we have "built-ins".

When we use more modules, our proof size goes up. This is partly due to "communication" proofs.

### Finite Field

Another important aspect is the field used within the zkVM.

- Stark252 (Cairo)
- Mini-Goldilocks (Miden, Ola)
- BabyBear (RISC0, SP1, Lita)
- BN254 base & scalar fields (Nexus)

> [!WARN]
>
> If we are using elliptic curves, we are not as free as we would be in STARK when it comes to picking our field.

Mersenne31 or the binary fields (e.g. as used with Binius) are not used yet, but it would be nice to see them in action sometime.

> Does using a small field incur any security costs? Well, not really because the sampling is done from an extension field, which is much larger even if we are working with a small field.

### Recursion

It is important if the VM has recursion or not.

- If you don't have recursion, the size of the things that you can prove becomes **bounded**.
- If you have recursion, you can prove things of arbitrary size.

For example in StarkNet, instead of proving a whole block, you can prove each transaction separately and then aggregate the proofs.

With the ability to do recursion, we can talk about **continutions**. If you want to prove a very large program, you can run up to a certain point, generate the proof, and then move on to the next part of the program and so on.

- One can reduce a list of proofs in a binary-tree fashion, reducing two-to-one until we get to the root.

- Or we can do "rollup continuations" where we prove each segment along with a proof that the previous segment was correct.

> **Final Proofs**: To make it easy to verify proofs in resource-constrained systems, one can choose to do a final proof using a proof system that is known to have a very efficient verifier. For example, a Groth16 proof to show that a STARK proof is valid.

### Folding Schemes / Proof-Carrying Data

If we are using R1CS (e.g. Nexus), we have some nice folding schemes (Nova, SuperNova, HyperNove) that we can use to "fold" multiple instances into one. Recall that R1CS looked like this:

$$
(Az) \cdot (Bz) = (Cz)
$$

The question is, if I had $z_1$ and $z_2$ can I combine them to produce $z'$ such that if I verify the R1CS with $z'$ it is the same as verifying the R1CS with $z_1$ and $z_2$ separately? The answer is: **yes**, you can. However, you need to do some modifications to R1CS.

Firstly, if the systems are linear, you can do a linear combination to compute $z' = z_1 + r \cdot z_2$. If the systems are not linear, this will fail though. An idea for this was to relax R1CS a bit:

$$
(Az) \cdot (Bz) = u \cdot (Cz) + E
$$

by adding "slack terms" $u$ and $E$ that account for the error. This is the idea behind **Nova** folding scheme. It is based on "Incrementially Verifiable Computation" (IVC). Instead of compressing all proofs into one, here we compress the "executions of the VM". The only proof that is generated is from the end result alone.

This is also the idea behind Mina Protocol, where the blockchain is compressed into a single proof. Their construction in based on Pickles (whic is based on Kimchi) and these make use of the Pasta (Pallas + Vesta) pair of elliptic curves. In such a pair of curves, the base field of one is the scalar field of the other and vice-versa.

### Hash Functions

The choice of hash function is rather important in a VM, as it is used in many places. Some choices:

- Pedersen
- Poseidon
- Monolotih
- Rescue Prime
- BLAKE
- KECCAK

## See also

- Miden docs are nice
- CairoVM paper is nice
