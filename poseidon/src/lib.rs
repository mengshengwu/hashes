#![no_std]
#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg"
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

pub use digest::{self, Digest};

/// Block-level types
pub mod block_api;
mod compress;
mod consts;

/// Poseidon hash function implementation.
///
/// Poseidon is a hash function designed specifically for zero-knowledge proof systems.
/// It was introduced in the paper "Poseidon: A New Hash Function for Zero-Knowledge Proof Systems"
/// by Lorenzo Grassi, Daniel Khovratovich, Christian Rechberger, Arnab Roy, and Markus Schofnegger.
///
/// ## Features
///
/// - **Sponge Construction**: Uses the sponge construction with Poseidon permutation
/// - **Field Arithmetic**: Operates over finite fields (currently simplified for demonstration)
/// - **Configurable Parameters**: Support for different curves (BN254, BLS12-381)
/// - **S-box Function**: Uses x^5 as the S-box function
/// - **MDS Matrix**: Maximum Distance Separable matrix for diffusion
/// - **Round Structure**: Full rounds + partial rounds + full rounds
///
/// ## Security
///
/// This implementation is for educational and demonstration purposes. For production use
/// in zero-knowledge proof systems, please use a well-audited implementation with proper
/// field arithmetic and security parameters.
///
/// ## Usage
///
/// ```rust
/// use poseidon::{Poseidon, Digest};
///
/// let mut hasher = Poseidon::new();
/// hasher.update(b"Hello, world!");
/// let result = hasher.finalize();
/// ```
digest::buffer_fixed!(
    /// Poseidon hasher.
    pub struct Poseidon(block_api::PoseidonCore);
    impl: FixedHashTraits;
); 