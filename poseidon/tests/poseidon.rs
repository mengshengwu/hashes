use digest::Digest;
use poseidon::Poseidon;

#[test]
fn test_poseidon_empty() {
    let hasher = Poseidon::new();
    let result = hasher.finalize();
    
    // Test that empty input produces consistent output
    let hasher2 = Poseidon::new();
    let result2 = hasher2.finalize();
    
    assert_eq!(result, result2);
    assert_eq!(result.len(), 24); // 192 bits = 24 bytes
}

#[test]
fn test_poseidon_basic() {
    let mut hasher = Poseidon::new();
    hasher.update(b"Hello, world!");
    let result = hasher.finalize();
    
    assert_eq!(result.len(), 24);
    
    // Test that same input produces same output
    let mut hasher2 = Poseidon::new();
    hasher2.update(b"Hello, world!");
    let result2 = hasher2.finalize();
    
    assert_eq!(result, result2);
}

#[test]
fn test_poseidon_different_inputs() {
    let mut hasher1 = Poseidon::new();
    hasher1.update(b"Hello");
    let result1 = hasher1.finalize();
    
    let mut hasher2 = Poseidon::new();
    hasher2.update(b"World");
    let result2 = hasher2.finalize();
    
    // Different inputs should produce different outputs
    assert_ne!(result1, result2);
}

#[test]
fn test_poseidon_large_input() {
    let mut hasher = Poseidon::new();
    let large_input = vec![0u8; 1000];
    hasher.update(&large_input);
    let result = hasher.finalize();
    
    assert_eq!(result.len(), 24);
}

#[test]
fn test_poseidon_reset() {
    let mut hasher = Poseidon::new();
    hasher.update(b"Hello");
    let result1 = hasher.finalize();
    
    let mut hasher2 = Poseidon::new();
    hasher2.update(b"Hello");
    let result2 = hasher2.finalize();
    
    // Same input should produce same output
    assert_eq!(result1, result2);
}

#[test]
fn test_poseidon_chunked() {
    let mut hasher = Poseidon::new();
    hasher.update(b"Hello");
    hasher.update(b", ");
    hasher.update(b"world!");
    let result1 = hasher.finalize();
    
    let mut hasher2 = Poseidon::new();
    hasher2.update(b"Hello, world!");
    let result2 = hasher2.finalize();
    
    // Chunked input should produce same output as single input
    assert_eq!(result1, result2);
} 