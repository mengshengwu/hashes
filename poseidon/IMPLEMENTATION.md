# Poseidon Hash Implementation

This document describes the implementation of the Poseidon hash function in Rust.

## Overview

Poseidon is a hash function designed specifically for zero-knowledge proof systems. It was introduced in the paper "Poseidon: A New Hash Function for Zero-Knowledge Proof Systems" by Lorenzo Grassi, Daniel Khovratovich, Christian Rechberger, Arnab Roy, and Markus Schofnegger.

## Architecture

### Core Components

1. **FieldElement**: Represents elements in a finite field
2. **State**: Array of field elements representing the hash state
3. **Compression Function**: Processes input blocks and updates state
4. **Permutation**: Applies the Poseidon permutation to the state

### Algorithm Structure

The Poseidon hash function follows the sponge construction:

```
Input → Padding → Absorption → Squeezing → Output
```

#### Permutation Structure

The Poseidon permutation consists of three phases:

1. **Full Rounds** (R_F/2 rounds): Apply S-box to all state elements
2. **Partial Rounds** (R_P rounds): Apply S-box only to first element
3. **Full Rounds** (R_F/2 rounds): Apply S-box to all state elements

Each round includes:
- Adding round constants
- Applying S-box function
- Applying MDS matrix multiplication

## Implementation Details

### Field Arithmetic

The current implementation uses simplified field arithmetic with `u64` values. In a production implementation, this should be replaced with proper finite field arithmetic over the target curve's base field.

```rust
pub struct FieldElement {
    value: u64,
}
```

### S-box Function

The S-box function is defined as `x^5`:

```rust
pub fn sbox(&self) -> Self {
    let mut result = self.value;
    for _ in 0..4 {
        result = result.wrapping_mul(self.value);
    }
    Self { value: result }
}
```

### MDS Matrix

The Maximum Distance Separable (MDS) matrix ensures diffusion:

```rust
pub const BN254_MDS_MATRIX: [[&str; 3]; 3] = [
    ["2", "3", "1"],
    ["1", "2", "3"], 
    ["3", "1", "2"],
];
```

### Parameters

Default parameters for BN254 curve:
- State size (t): 3
- Full rounds (R_F): 8
- Partial rounds (R_P): 56
- Output size: 192 bits (24 bytes)

## Security Considerations

### Current Limitations

1. **Simplified Field Arithmetic**: Uses `u64` instead of proper field arithmetic
2. **Demo Round Constants**: Uses simple sequential constants instead of cryptographically secure ones
3. **Demo MDS Matrix**: Uses a simple matrix instead of a cryptographically secure one

### Production Requirements

For production use in zero-knowledge proof systems:

1. **Proper Field Arithmetic**: Implement arithmetic over the target curve's base field
2. **Secure Round Constants**: Use cryptographically secure round constants
3. **Secure MDS Matrix**: Use a cryptographically secure MDS matrix
4. **Security Audit**: Undergo thorough security analysis
5. **Constant-time Implementation**: Ensure constant-time execution

## Usage Examples

### Basic Hashing

```rust
use poseidon::{Poseidon, Digest};

let mut hasher = Poseidon::new();
hasher.update(b"Hello, world!");
let result = hasher.finalize();
```

### Incremental Hashing

```rust
let mut hasher = Poseidon::new();
hasher.update(b"Part 1");
hasher.update(b"Part 2");
hasher.update(b"Part 3");
let result = hasher.finalize();
```

## Testing

The implementation includes comprehensive tests:

- Empty input hashing
- Basic functionality
- Deterministic output
- Avalanche effect
- Large input handling
- Incremental hashing
- Collision resistance

## Performance

Current performance characteristics:
- Throughput: ~218,000 hashes/second (on test hardware)
- Memory usage: Minimal (in-place operations)
- Block size: 64 bytes

## Future Improvements

1. **Proper Field Arithmetic**: Implement full field arithmetic over BN254/BLS12-381
2. **Optimized Implementation**: Use SIMD instructions for better performance
3. **More Curves**: Support additional elliptic curves
4. **Variable Output Size**: Support different output sizes
5. **Security Audit**: Undergo formal security analysis

## References

- [Poseidon Paper](https://eprint.iacr.org/2019/458.pdf)
- [RustCrypto Hashes](https://github.com/RustCrypto/hashes)
- [Digest Trait](https://docs.rs/digest) 