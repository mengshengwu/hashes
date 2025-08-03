# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-08-03

### Added
- Initial implementation of Poseidon hash function
- Support for BN254 and BLS12-381 curve parameters
- Field element arithmetic operations
- S-box function (x^5) implementation
- MDS matrix multiplication
- Full and partial round implementations
- 192-bit (24-byte) output size
- Comprehensive test suite
- Basic usage examples

### Features
- `alloc` (default): Enables allocation features
- `zeroize`: Enables zeroization of sensitive data

### Technical Details
- Based on the Poseidon paper: "Poseidon: A New Hash Function for Zero-Knowledge Proof Systems"
- Implements the sponge construction with Poseidon permutation
- Uses simplified field arithmetic for demonstration purposes
- Configurable round constants and MDS matrices
- Compatible with the `digest` crate interface 