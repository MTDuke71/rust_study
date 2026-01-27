# Fibonacci Sequence

**Category**: Number Theory, Sequences, Recurrence Relations  
**Difficulty**: Fundamental

## Definition

The **Fibonacci sequence** is an infinite sequence of integers where each term is the sum of the two preceding terms.

**Recurrence relation**:
$$F_n = F_{n-1} + F_{n-2}$$

**Initial conditions**: $F_1 = 1$, $F_2 = 1$ (or alternatively $F_0 = 0$, $F_1 = 1$)

**Sequence**: 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, ...

## Key Properties

### 1. Exponential Growth

Fibonacci numbers grow exponentially, approximately following:
$$F_n \approx \frac{\phi^n}{\sqrt{5}}$$

where $\phi = \frac{1+\sqrt{5}}{2} \approx 1.618$ is the **golden ratio**.

**Implication**: Fibonacci numbers grow very fast! $F_{50} \approx 10^{10}$

### 2. Parity Pattern

The parity (even/odd) of Fibonacci numbers follows a repeating pattern:
```
F_1  F_2  F_3  F_4  F_5  F_6  F_7  F_8  F_9  ...
O    O    E    O    O    E    O    O    E    ...
```

**Pattern**: **Odd, Odd, Even** (repeats every 3 terms)

**Proof**: 
- O + O = E
- E + O = O  
- O + E = O
- O + O = E (cycle repeats)

**Consequence**: Every 3rd Fibonacci number is even!

### 3. GCD Property

$$\gcd(F_m, F_n) = F_{\gcd(m, n)}$$

**Example**: $\gcd(F_6, F_9) = \gcd(8, 34) = 2 = F_3$ where $\gcd(6, 9) = 3$

### 4. Closed-Form Formula (Binet's Formula)

$$F_n = \frac{\phi^n - \psi^n}{\sqrt{5}}$$

where:
- $\phi = \frac{1+\sqrt{5}}{2}$ (golden ratio)
- $\psi = \frac{1-\sqrt{5}}{2}$ (conjugate of golden ratio)

**Note**: Despite involving irrational numbers, the result is always an integer!

## Efficient Computation

### Standard Iteration (O(n) time, O(1) space)
```rust
fn fibonacci(n: usize) -> u64 {
    if n <= 2 { return 1; }
    let (mut a, mut b) = (1, 1);
    for _ in 2..n {
        let next = a + b;
        a = b;
        b = next;
    }
    b
}
```

### Matrix Exponentiation (O(log n) time)

Fibonacci can be computed using matrix powers:
$$\begin{bmatrix} F_{n+1} \\ F_n \end{bmatrix} = \begin{bmatrix} 1 & 1 \\ 1 & 0 \end{bmatrix}^n \begin{bmatrix} 1 \\ 0 \end{bmatrix}$$

Using fast exponentiation: compute $F_n$ in **O(log n)** time!

## Even Fibonacci Recurrence

**Special property**: Even Fibonacci numbers satisfy their own recurrence:

If $E_n$ represents the $n$-th even Fibonacci number:
- $E_1 = 2$ (which is $F_3$)
- $E_2 = 8$ (which is $F_6$)
- $E_3 = 34$ (which is $F_9$)

Then:
$$E_n = 4E_{n-1} + E_{n-2}$$

**Derivation**: See [[project-euler-p002]] for complete proof.

**Application**: Summing even Fibonacci terms without generating odd ones!

## Applications

### In Computer Science
- **Dynamic programming**: Classic example of overlapping subproblems
- **Recursion**: Teaching recursion vs memoization vs iteration
- **Complexity analysis**: Worst case for Euclidean algorithm

### In Nature
- **Phyllotaxis**: Arrangement of leaves on a stem
- **Seed patterns**: Sunflower seed spirals
- **Animal populations**: Rabbit breeding problem (original Fibonacci)

### In Mathematics
- **Number theory**: Divisibility properties, primality patterns
- **Combinatorics**: Counting tilings, paths, compositions
- **Golden ratio**: $\lim_{n \to \infty} \frac{F_{n+1}}{F_n} = \phi$

## Generalizations

### Lucas Numbers
Similar recurrence but different initial values:
- $L_n = L_{n-1} + L_{n-2}$
- $L_1 = 1$, $L_2 = 3$
- Sequence: 1, 3, 4, 7, 11, 18, 29, ...

### Tribonacci Numbers
Sum of previous **three** terms:
- $T_n = T_{n-1} + T_{n-2} + T_{n-3}$

### Generalized Fibonacci: F(a, b)
Start with arbitrary $F_1 = a$, $F_2 = b$

## Related Concepts

- **[[recurrence-relations]]** - Fibonacci is the canonical example
- **[[golden-ratio]]** - φ arises naturally from Fibonacci
- **[[dynamic-programming]]** - Fibonacci demonstrates memoization
- **[[modular-arithmetic]]** - Pisano periods (Fibonacci mod m)

## Project Euler Problems

- [[project-euler-p002]] - Sum of even Fibonacci numbers
  - Uses even Fibonacci recurrence: $E_n = 4E_{n-1} + E_{n-2}$
  - Demonstrates O(k) solution where k = even terms only

## Implementation Examples

### Rust (from PE2)
```rust
pub fn sum_even_fib(limit: u64) -> u64 {
    if limit < 2 { return 0; }
    let mut sum = 2u64;
    let mut e_prev = 2u64;
    let mut e_curr = 8u64;
    
    while e_curr <= limit {
        sum += e_curr;
        let next = 4 * e_curr + e_prev;  // Even recurrence
        e_prev = e_curr;
        e_curr = next;
    }
    sum
}
```

See `project_euler/src/problems/p002.rs` for complete code.

## Historical Note

Named after **Leonardo Fibonacci** (c. 1170 – c. 1250), Italian mathematician who introduced the sequence to Western European mathematics in his book *Liber Abaci* (1202).

**Original problem**: "How many pairs of rabbits will there be after n months if:
- You start with one pair
- Each pair produces one new pair each month
- New pairs reproduce starting their second month"

Answer: $F_n$ pairs after n months!

## References

- *Concrete Mathematics* by Graham, Knuth, Patashnik - Chapter 6
- *The Art of Computer Programming Vol. 1* - Knuth (Section 1.2.8)
- [[project-euler-p002]] - Application to even Fibonacci sums
- [[recurrence-relations]] - General theory

---

*Links:*
- **Applications**: [[project-euler-p002]], [[dynamic-programming]]
- **Related Concepts**: [[recurrence-relations]], [[golden-ratio]], [[modular-arithmetic]]
- **Code**: `project_euler/src/problems/p002.rs`
- **Tags**: #fibonacci #sequences #number-theory #recurrence-relations #golden-ratio
