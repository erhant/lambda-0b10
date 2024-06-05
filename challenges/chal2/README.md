# [Challenge 2](https://github.com/lambdaclass/lambdaworks/tree/main/exercises/challenge_2)

> # Breaking into the vault of Loki
>
> After years of careful investigation, you have reached the gate to Loki's vault in the icy mountains of Norway, where it is said that many great treasures and powerful weapons are hidden. The gate seems unbreakable, but you spot some ancient machinery with inscriptions in old runes. After some help from ChatGPT, you are able to translate the symbols and the whole message into modern English, and it reads:
>
> If you can prove that the polynomial
>
> $$
> \begin{aligned}
> p(x) &= 69 +78x + 32x^2 + 65x^3 + 82x^4 + 71x^5 + 69x^6 + 78x^7 + 84x^8 + 73x^9 \newline &+78x^{10} + 65x^{11} + 32x^{12} + 78x^{13} + 65x^{14}+ 67x^{15} + 73x^{16} + 32x^{17} \newline
> &+ 84x^{18} + 73x^{19} + 69x^{20} + 82x^{21} + 82x^{22} + 65 x^{23}
> \end{aligned}
> $$
>
> is equal to $3$ at $x = 1$ modulo the BLS12-381 $r$ parameter, then the gate will open.
>
> Below is a long list of bytes representing the SRS that can be used to perform KZG commitments. The machinery, after careful examination, performs the KZG verification using pairings. There is only one open place where you can place a wooden tablet with your answer, comprising 48 bytes. You guess this should be the proof of the KZG scheme, providing the point in compressed form, following the ZCash standard. The other elements contain the commitment to $p(x)$, the desired value $3$, and the point $x=1$. You ask ChatGPT for enlightenment, but it suddenly collapses and only shows the message: fatal error. Is this just an impossible task? Perhaps there is some trick to get by Loki's challenge...

## Solution

It appears that we need to be making a fake proof using KZG, which is possible when you know the toxic waste. This brings the question: can we find something within the SRS that perhaps gives away the toxic waste? Now, toxic waste is just a scalar; the error itself probably has to do with the generator point picked for the curve.

When we look at the points within SRS, it appears that the points start to repeat at some point! In fact, at precisely every 64 elements, the points repeat. This means that for some generator $g$ and secret $s$ we have $g = s^{64}g$. Given the fact that $g$ is not the point at infinity, we can see the following:

$$
g \equiv s^{64}g \pmod{r} \implies s^{64} \equiv 1 \pmod{r}
$$

Our secret is actually a primitive-64th root of unity in the scalar field! It's primitive because otherwise we would see the points repeat earlier. Once we find a candidate value for the primitive-64th root of unity, we can simply check if $g$ times the candidate equals the second point in the SRS. If it does, we have found the toxic waste!

### Finding the Primitive Root

To find the primitive-64th root of unity, we can simply brute force it. Once we have the toxic waste, we can generate a fake proof and open the gate to Loki's vault. To find this value, we follow an approach described at <https://crypto.stackexchange.com/a/63616>.

First, does 64 divide $p-1$ for the BLS12-381's scalar order? The scalar order is equal to $r$ below (see from <https://neuromancer.sk/std/bls/BLS12-381>):

```c
r   = 0x73EDA753299D7D483339D80809A1D80553BDA402FFFE5BFEFFFFFFFF00000001
r-1 = 0x73EDA753299D7D483339D80809A1D80553BDA402FFFE5BFEFFFFFFFF00000000
```

It is pretty obvious that we can divide $r-1$ by 64, since we can right-shift it 6 times without losing any information. The quotient is then:

```c
r-1 / 64 = 0x01CFB69D4CA675F520CCE76020268760154EF6900BFFF96FFBFFFFFFFC000000
```

Following the method in the link above, consider a random $x$ in the field.

$$
(x^{(r-1)/64})^{64} = x^{r-1} = 1
$$

So, $x^{(r-1)/64}$ is a 64th root of unity. When we are interested in a primitive $n$-th root $g$, it must be that $g^j \ne 1$ for all $0 < j < n$ that divides $n$. Since $n=64$ in our case, we must ensure that $g^{32}$ is not a root of unity as well. It suffices to check for 32 and not for 16, 8 etc. as any further divisions will surely result in an 32th root anyways.

Here is the snippet:

```rs
fn find_primitive_root() -> FrElement {
    loop {
        // random element within the scalar field of order r
        let g = FrElement::from(random::<u64>());

        // (r - 1) / 64
        let cofactor: UnsignedInteger<6> = UnsignedInteger::from_hex_unchecked(
            "0x01CFB69D4CA675F520CCE76020268760154EF6900BFFF96FFBFFFFFFFC000000",
        );

        // obtain root of unity via cofactor clearing
        let root = g.pow(cofactor);
        debug_assert_eq!(root.pow(64u64), FrElement::one());

        // check that its indeed primitive
        if root.pow(32u64) != FrElement::one() {
            return root;
        }
    }
}
```

### Finding the Secret

When it comes to primitive roots, there is no "the" primitive root; there are many, and none is more primitive than the other. We can simply pick one and check if it works. To do this, we look at the first two elements in SRS for the group, which is $\{g, sg\}$. Once we find a candidate $s'$, we can check if $sg = s'g$ and then recover the "toxic waste".

Here is the snippet:

```rs
fn find_toxic_waste(g1: &G1Point, sg1: &G1Point, g2: &G2Point, sg2: &G2Point) -> FrElement {
    // infinite loop, but we are SURE about this
    loop {
        // find a primitive root of unity
        let s = find_primitive_root();

        // see if it matches the secret
        if g1.operate_with_self(s.representative()) == *sg1
            && g2.operate_with_self(s.representative()) == *sg2
        {
            return s;
        }
    }
}
```

### Faking the Proof

Now that we know the toxic waste, we can generate the fake proof. Recall the thing that a verifier checks in KZG: consider a polynomial $P(x)$ that is evaluated at point $z$ and results in $v$. This implies that:

$$
P(x) - v = (x-z)Q(x)
$$

for some polynomial $Q(x)$. By using the commitments at a secret points $s$ here we prove:

$$
P(s) - v = (s-z)Q(s)
$$

Normally, the $s$ on the right side comes from the $sg_2$ within the SRS, and the quotient polynomial & its commitment is computed explicitly. However, ours is a fake proof and we can't have a valid quotient here. Instead, we will do:

$$
(P(s) - v)(s-z)^{-1} = Q(s)
$$

By knowing the secret $s$ we can make it seem as if we have committed to a valid quotient polynomial! This is perfectly fine since its just a scalar multiplication anyways. Our fake proof is to show that $P(1) = 3$, which means:

$$
(P(s) - 3)(s-1)^{-1} = Q(s)
$$

Well, we can compute that $Q(s)$ without having $Q$ at all, the left hand-side of the equation is enough. Once we compute $Q(s)$ that way, all that remains is to compute the commitment $Q(s)g_1$ (as if we have done an MSM). Again, recall that this was only possible because we perfectly knew the secret $s$.

```rs
let (v, z) = (FrElement::from(3), FrElement::from(1));

// compute q(s) via the fake proof method = (P(s) - v) / (s - z)
let q_s = (p.evaluate(&s) - v.clone()) * (s - z.clone()).inv().expect("should invert");

// find the commitment as g * q(s)
// normally we would do MSM for this using SRS, but we know the toxic waste :)
let q_commitment = g1.operate_with_self(q_s.representative());

let fake_proof = q_commitment;
println!("Fake proof for submission:");
println!("{:?}", &fake_proof.to_affine().x().to_string());
println!("{:?}", &fake_proof.to_affine().y().to_string());
assert!(kzg.verify(&z, &v, &p_commitment, &fake_proof));
println!("Faked succesfully!");
```
