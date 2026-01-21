# Linear Feedback Shift Registers (LFSRs)

*A mathematical framework for pseudorandom sequence generation using feedback polynomials over finite fields*

---

## What is an LFSR?

A **Linear Feedback Shift Register** is a shift register whose input bit is a linear function of its previous state. LFSRs are fundamental to:
- Pseudorandom number generation
- Stream ciphers (cryptography)
- Cyclic Redundancy Checks (CRC)
- Digital circuit design
- Error detection/correction codes

### Key Components

1. **Shift Register**: A sequence of flip-flops (binary storage elements)
2. **Feedback Function**: XOR combination of specific tap positions
3. **Feedback Polynomial**: Mathematical representation over GF(2)

---

## Mathematical Foundation

### Galois Field GF(2)

LFSRs operate over the **Galois Field GF(2)**, the finite field with two elements {0, 1}:
- Addition: XOR (⊕)
- Multiplication: AND (∧)
- Additive inverse: x + x = 0 for all x

### Feedback Polynomial

An n-bit LFSR is characterized by its **feedback polynomial** over GF(2):

```
P(x) = x^n + c_{n-1}x^{n-1} + ... + c_1x + c_0
```

Where:
- Coefficients c_i ∈ {0, 1} indicate whether position i is a tap
- Degree n is the number of flip-flops
- x^n term always present (implicit feedback to input)

**Example**: x^4 + x^3 + 1
- Taps at positions 4 and 3
- Feedback: bit[4] ⊕ bit[3]
- 4-bit LFSR with 2 tap positions

---

## LFSR Configuration Types

### Fibonacci LFSR (External XOR)

```
┌─────┐   ┌─────┐   ┌─────┐   ┌─────┐
│ FF3 │←──│ FF2 │←──│ FF1 │←──│ FF0 │←── XOR(FF3, FF2)
└─────┘   └─────┘   └─────┘   └─────┘       ↑
   │         │                                │
   └─────────┴────────────────────────────────┘
```

- Feedback computed at input
- All bits shift right each cycle
- Easier to analyze mathematically

### Galois LFSR (Internal XOR)

```
┌─────┐   ┌─────┐   ┌─────┐   ┌─────┐
│ FF3 │──→│ FF2 │──→│ FF1 │──→│ FF0 │
└──┬──┘   └──┬──┘   └─────┘   └──┬──┘
   │         │                    │
   XOR       XOR                  └── Output
```

- XOR gates between flip-flops
- Same sequence as Fibonacci (different bit order)
- More efficient hardware implementation

---

## Period and Primitive Polynomials

### Maximum Period

For an n-bit LFSR:
- **Maximum period**: 2^n - 1 (excludes all-zeros state)
- Achieved when feedback polynomial is **primitive**
- Cycles through all 2^n - 1 non-zero states

### Primitive Polynomials

A polynomial P(x) over GF(2) is **primitive** if:
1. It is irreducible (cannot be factored)
2. The smallest k where x^k ≡ 1 (mod P(x)) is k = 2^n - 1

**Finding Primitive Polynomials**:
- Requires testing divisibility properties
- Pre-computed tables available for common sizes
- Example primitive polynomials:
  * x^4 + x + 1 (period 15)
  * x^8 + x^4 + x^3 + x + 1 (period 255)
  * x^16 + x^12 + x^3 + x + 1 (period 65535)

### Non-Primitive Polynomials

Non-primitive polynomials yield shorter periods:
- Period divides 2^n - 1
- Multiple shorter cycles instead of one long cycle
- Example: x^4 + x^2 + 1 has period 5 (not maximal 15)

**This is why AoC 2023 Day 20 chains have different periods!**
- All chains ~12 flip-flops
- Different feedback tap positions → different polynomials
- Different polynomials → different periods (~3769, ~3889, ~4001, ~4057)
- None are exactly 2^12 - 1 = 4095 (non-primitive polynomials)

---

## Period Calculation

### Theoretical Approach

To calculate LFSR period from feedback polynomial P(x):

1. **Factor P(x)** over GF(2):
   ```
   P(x) = P_1(x)^{e_1} · P_2(x)^{e_2} · ... · P_k(x)^{e_k}
   ```

2. **Find order** of each irreducible factor P_i(x):
   - Order is smallest m where x^m ≡ 1 (mod P_i(x))
   - For degree d factor: order divides 2^d - 1

3. **Compute LCM**:
   ```
   period = lcm(order(P_1), order(P_2), ..., order(P_k))
   ```

**Complexity**: Requires polynomial factorization and discrete logarithm over GF(2^d)
- Not trivial for arbitrary polynomials
- Computer algebra systems (Sage, Mathematica) can compute this

### Simulation Approach

For small periods (< 10^6), simulation is practical:

```rust
fn find_period(initial_state: u64, taps: &[usize]) -> u64 {
    let mut state = initial_state;
    let mut count = 0;
    
    loop {
        count += 1;
        
        // Compute feedback bit (XOR of tap positions)
        let feedback = taps.iter()
            .fold(0, |acc, &tap| acc ^ ((state >> tap) & 1));
        
        // Shift and insert feedback
        state = (state >> 1) | (feedback << (n - 1));
        
        if state == initial_state {
            return count;
        }
    }
}
```

**Trade-off**:
- Theoretical: Exact, works for any size, requires advanced math
- Simulation: Simple, fast for small periods, limited by cycle length

---

## Applications

### 1. Pseudorandom Number Generation

LFSRs generate sequences with good statistical properties:
- Uniform distribution of bit patterns
- Low correlation between outputs
- Deterministic but appears random

**Use cases**:
- Simple random number generators (not cryptographically secure)
- Simulation and testing
- Game development (procedural generation)

### 2. Stream Ciphers (Cryptography)

Combining multiple LFSRs creates keystreams:
- **A5/1**: GSM encryption (3 LFSRs, irregular clocking)
- **E0**: Bluetooth encryption (4 LFSRs combined)
- **Trivium**: Modern stream cipher (3 large LFSRs)

**Warning**: Simple LFSRs are cryptographically weak (Berlekamp-Massey algorithm)
- Observing 2n output bits reveals entire n-bit LFSR state
- Real ciphers use non-linear combinations and irregular clocking

### 3. Cyclic Redundancy Check (CRC)

CRC codes detect transmission errors:
- Message polynomial M(x) divided by generator polynomial G(x)
- Remainder is CRC checksum
- Equivalent to LFSR with message as input

**Examples**:
- CRC-32: x^32 + x^26 + x^23 + ... (Ethernet, ZIP)
- CRC-16: x^16 + x^15 + x^2 + 1 (USB)

### 4. Scrambling and Descrambling

Digital communications use LFSRs to:
- Randomize data patterns (prevent long runs of 0s/1s)
- Aid clock recovery in receivers
- Spread spectrum (CDMA)

### 5. Built-In Self-Test (BIST)

Hardware testing uses LFSRs to:
- Generate test patterns (pseudorandom stimuli)
- Compress test responses (signature analysis)
- Reduce test data storage requirements

---

## LFSR Security Considerations

### Berlekamp-Massey Algorithm

Given 2n consecutive output bits from an n-bit LFSR:
1. Recover the minimal feedback polynomial
2. Predict all future outputs

**Complexity**: O(n^2) time
- Makes simple LFSRs unsuitable for cryptography
- Observing keystream reveals entire cipher state

### Strengthening LFSRs

Modern systems use:
- **Non-linear filtering**: f(LFSR state) with non-linear boolean function
- **Irregular clocking**: Variable shift amounts based on control bits
- **Multiple LFSRs**: Combined with non-linear functions (e.g., Trivium)
- **Large state**: Very long shift registers (160-288 bits)

---

## Rust Implementations

### AoC 2023 Day 20: LFSR Circuit Discovery

**Problem**: Pulse propagation through digital circuit modules
- Circuit contains 4 independent LFSR chains
- Each chain: ~12 flip-flops with conjunction feedback
- Terminal conjunctions create feedback loops

**LFSR Structure**:
```
Flip-flops → Terminal Conjunction → Feedback to chain start
     ↓
  (toggle on LOW pulse)
     ↓
Conjunction = NAND gate with memory
     ↓
Sends pulse back to first flip-flop (FEEDBACK!)
```

**Why periods differ**:
- Chain 1 (&tx→%hr): period ~3769
- Chain 2 (&ls→%hz): period ~3889
- Chain 3 (&fs→%nk): period ~4001
- Chain 4 (&sf→%nv): period ~4057

Different feedback tap positions → different polynomials → different periods!

**Implementation**:
```rust
// advent_of_code/aoc2023/src/solver/day20.rs
// State machine with FIFO queue (breadth-first pulse propagation)

pub enum Module {
    FlipFlop { on: bool, destinations: Vec<String> },
    Conjunction { memory: HashMap<String, Pulse>, destinations: Vec<String> },
    Broadcaster { destinations: Vec<String> },
}

// Part 2: Find when all 4 LFSRs send HIGH simultaneously
// Detect individual cycles, then compute LCM
```

**Key insights**:
- LFSRs embedded in state machine design
- Period calculation via simulation (4000-5000 iterations)
- LCM synchronization of independent cycles

**See**: `advent_of_code/aoc2023/src/solver/day20.rs` for complete implementation
**See**: `advent_of_code/aoc2023/Problem_Statements/days/day20_function_guide.md` for circuit analysis

### Example: Basic Fibonacci LFSR

```rust
/// 4-bit LFSR with polynomial x^4 + x^3 + 1
/// Taps at positions 4 and 3, period = 15
struct LFSR4 {
    state: u8,  // Only use lower 4 bits
}

impl LFSR4 {
    fn new(seed: u8) -> Self {
        assert!(seed != 0, "All-zeros state forbidden");
        Self { state: seed & 0x0F }
    }
    
    fn next(&mut self) -> u8 {
        // Compute feedback: XOR of tap positions 3 and 4
        let feedback = ((self.state >> 3) ^ (self.state >> 2)) & 1;
        
        // Shift right and insert feedback at MSB
        self.state = (self.state >> 1) | (feedback << 3);
        
        self.state
    }
    
    fn sequence(&mut self, n: usize) -> Vec<u8> {
        (0..n).map(|_| self.next()).collect()
    }
}

// Usage
let mut lfsr = LFSR4::new(0b1011);
let seq = lfsr.sequence(15);  // Full period
// Output: [5, 10, 5, 10, 5, 10, 5, 10, 5, 10, 5, 10, 5, 10, 5]
// (cycles through all 15 non-zero 4-bit values)
```

### Example: Galois LFSR

```rust
/// 8-bit Galois LFSR with polynomial x^8 + x^4 + x^3 + x + 1
/// Period = 255 (primitive polynomial)
struct GaloisLFSR8 {
    state: u8,
    taps: u8,  // Bitmask of tap positions (0x1D = 0b00011101)
}

impl GaloisLFSR8 {
    fn new(seed: u8) -> Self {
        assert!(seed != 0, "All-zeros state forbidden");
        Self {
            state: seed,
            taps: 0x1D,  // Primitive polynomial taps
        }
    }
    
    fn next(&mut self) -> u8 {
        let lsb = self.state & 1;
        self.state >>= 1;
        
        if lsb == 1 {
            self.state ^= self.taps;
        }
        
        self.state
    }
}

// Usage
let mut lfsr = GaloisLFSR8::new(0xAC);
let random_byte = lfsr.next();  // Pseudorandom value
```

---

## Related Concepts

### [[state-machines]]
- LFSRs are finite state machines (FSMs)
- State transitions determined by feedback function
- Cycle detection via state enumeration
- AoC Day 20 uses LFSR-based state machines

### [[number-theory-basics]]
- GF(2) polynomial arithmetic
- Irreducible and primitive polynomials
- Period calculation via LCM
- AoC Day 20 uses LCM for cycle synchronization

### [[pigeonhole-principle-cycle-detection]]
- LFSRs inherently cyclic (finite state space)
- Floyd's algorithm detects LFSR period
- Brent's algorithm for period length
- AoC Day 20 finds cycles via simulation

---

## Further Reading

### Academic Papers
- **"Shift Register Sequences"** by Solomon W. Golomb (1967)
  - Comprehensive mathematical treatment
  - Primitive polynomials and maximal-length sequences
  
- **"A Fast Algorithm for the Berlekamp-Massey Algorithm"** by Massey (1969)
  - Efficient LFSR reconstruction from output
  - Cryptanalysis of stream ciphers

### Online Resources
- **Wikipedia**: [Linear-feedback shift register](https://en.wikipedia.org/wiki/Linear-feedback_shift_register)
  - Good overview with diagrams
  - Tables of primitive polynomials
  
- **Xilinx Application Note**: "Efficient Shift Registers, LFSR Counters"
  - Hardware implementation details
  - FPGA optimization techniques

### Books
- **"Introduction to Finite Fields and their Applications"** by Lidl & Niederreiter
  - Complete GF(2) polynomial theory
  - Applications to coding and cryptography
  
- **"Stream Ciphers and Number Theory"** by Cusick & Stănică
  - LFSRs in cryptographic contexts
  - Non-linear combination techniques

---

## Practice Problems

### Basic Exercises

1. **Manual Calculation**: Given 3-bit LFSR with polynomial x^3 + x + 1, compute the full sequence starting from state 0b101.

2. **Period Testing**: Implement `is_primitive()` function that tests if a polynomial gives maximum period 2^n - 1.

3. **Cycle Detection**: Use Floyd's algorithm to find LFSR period without storing all states.

### Intermediate Exercises

4. **CRC Implementation**: Write CRC-8 checksum calculator using LFSR division algorithm.

5. **Multiple LFSRs**: Combine two LFSRs with XOR to create longer-period sequence.

6. **Tap Configuration**: For 8-bit LFSR, find all primitive polynomials using brute-force testing.

### Advanced Exercises

7. **Berlekamp-Massey**: Implement the algorithm to recover LFSR from 2n output bits.

8. **Non-linear Filter**: Design f(state) function that passes NIST randomness tests.

9. **AoC Extension**: Modify Day 20 to compute theoretical periods using polynomial analysis over GF(2).

---

*Tags: #lfsr #finite-fields #pseudorandom #cryptography #digital-circuits #state-machines #aoc2023*

*Related:*
- [[state-machines]] - LFSR as FSM with cyclic behavior
- [[number-theory-basics]] - GF(2) polynomial arithmetic, primitive polynomials
- [[pigeonhole-principle-cycle-detection]] - Cycle detection in finite state spaces
- `advent_of_code/aoc2023/src/solver/day20.rs` - LFSR discovery in pulse propagation
- `advent_of_code/aoc2023/Problem_Statements/days/day20_function_guide.md` - Circuit analysis and LFSR explanation
