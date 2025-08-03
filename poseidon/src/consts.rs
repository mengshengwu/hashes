//! Poseidon constants and parameters

/// Poseidon parameters for BN254 curve
pub const BN254_T: usize = 3; // t parameter (state size)
pub const BN254_FULL_ROUNDS: usize = 8; // R_F parameter
pub const BN254_PARTIAL_ROUNDS: usize = 56; // R_P parameter

/// Poseidon parameters for BLS12-381 curve  
pub const BLS12_381_T: usize = 3;
pub const BLS12_381_FULL_ROUNDS: usize = 8;
pub const BLS12_381_PARTIAL_ROUNDS: usize = 56;

/// Default parameters (using BN254)
pub const DEFAULT_T: usize = BN254_T;
pub const DEFAULT_FULL_ROUNDS: usize = BN254_FULL_ROUNDS;
pub const DEFAULT_PARTIAL_ROUNDS: usize = BN254_PARTIAL_ROUNDS;

/// Field modulus for BN254 (p = 21888242871839275222246405745257275088548364400416034343698204186575808495617)
pub const BN254_MODULUS: &str = "21888242871839275222246405745257275088548364400416034343698204186575808495617";

/// Field modulus for BLS12-381
pub const BLS12_381_MODULUS: &str = "4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787";

/// Default field modulus
pub const DEFAULT_MODULUS: &str = BN254_MODULUS;

/// Round constants for BN254 (first few rounds as example)
pub const BN254_ROUND_CONSTANTS: [&str; 8] = [
    "0",
    "1", 
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
];

/// MDS matrix for BN254 (simplified for t=3)
pub const BN254_MDS_MATRIX: [[&str; 3]; 3] = [
    ["2", "3", "1"],
    ["1", "2", "3"], 
    ["3", "1", "2"],
]; 