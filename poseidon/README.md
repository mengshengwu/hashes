# Poseidon Hash

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]

A pure Rust implementation of the [Poseidon hash function][1].

## About

Poseidon is a hash function designed for zero-knowledge proof systems. It was introduced in the paper "Poseidon: A New Hash Function for Zero-Knowledge Proof Systems" by Lorenzo Grassi, Daniel Khovratovich, Christian Rechberger, Arnab Roy, and Markus Schofnegger.

## Usage

```rust
use poseidon::{Poseidon, Digest};

let mut hasher = Poseidon::new();
hasher.update(b"Hello, world!");
let result = hasher.finalize();
```

## Features

- `alloc` (default): Enables allocation features
- `zeroize`: Enables zeroization of sensitive data

## License

Licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/poseidon.svg
[crate-link]: https://crates.io/crates/poseidon
[docs-image]: https://docs.rs/poseidon/badge.svg
[docs-link]: https://docs.rs/poseidon/
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.85+-blue.svg

[//]: # (links)

[1]: https://eprint.iacr.org/2019/458.pdf 