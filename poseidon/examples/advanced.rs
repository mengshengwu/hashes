use poseidon::{Poseidon, Digest};
use std::collections::HashMap;

// Simple hex encoding function
fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn main() {
    println!("=== Poseidon Hash Function Demo ===\n");

    // Basic hashing
    println!("1. Basic Hashing:");
    let mut hasher = Poseidon::new();
    hasher.update(b"Hello, Poseidon!");
    let hash = hasher.finalize();
    println!("   Input: 'Hello, Poseidon!'");
    println!("   Hash: {:?}", hash);
    println!("   Hash (hex): {}", to_hex(&hash));
    println!();

    // Deterministic hashing
    println!("2. Deterministic Hashing:");
    let mut hasher1 = Poseidon::new();
    hasher1.update(b"Same input");
    let hash1 = hasher1.finalize();
    
    let mut hasher2 = Poseidon::new();
    hasher2.update(b"Same input");
    let hash2 = hasher2.finalize();
    
    println!("   Same input produces same hash: {}", hash1 == hash2);
    println!();

    // Avalanche effect demonstration
    println!("3. Avalanche Effect (small input changes):");
    let inputs = [
        b"Hello",
        b"Hellp", // One character different
        b"Hallo", // Two characters different
    ];
    
    for input in &inputs {
        let mut hasher = Poseidon::new();
        hasher.update(*input);
        let hash = hasher.finalize();
        println!("   Input: '{}' -> Hash: {:?}", String::from_utf8_lossy(*input), hash);
    }
    println!();

    // Large data hashing
    println!("4. Large Data Hashing:");
    let large_data = vec![0u8; 10000];
    let mut hasher = Poseidon::new();
    hasher.update(&large_data);
    let hash = hasher.finalize();
    println!("   10KB of zeros -> Hash: {:?}", hash);
    println!();

    // Incremental hashing
    println!("5. Incremental Hashing:");
    let mut hasher = Poseidon::new();
    hasher.update(b"Part 1");
    hasher.update(b"Part 2");
    hasher.update(b"Part 3");
    let incremental_hash = hasher.finalize();
    
    let mut hasher2 = Poseidon::new();
    hasher2.update(b"Part 1Part 2Part 3");
    let single_hash = hasher2.finalize();
    
    println!("   Incremental: {:?}", incremental_hash);
    println!("   Single:      {:?}", incremental_hash);
    println!("   Equal: {}", incremental_hash == single_hash);
    println!();

    // Hash collision resistance test
    println!("6. Collision Resistance Test:");
    let mut hash_map = HashMap::new();
    let mut collision_found = false;
    
    for i in 0..1000 {
        let input = format!("Input {}", i);
        let mut hasher = Poseidon::new();
        hasher.update(input.as_bytes());
        let hash = hasher.finalize();
        
        if let Some(existing_input) = hash_map.get(&hash) {
            println!("   Collision found!");
            println!("   Input 1: '{}'", existing_input);
            println!("   Input 2: '{}'", input);
            collision_found = true;
            break;
        } else {
            hash_map.insert(hash, input);
        }
    }
    
    if !collision_found {
        println!("   No collisions found in 1000 random inputs");
    }
    println!();

    // Performance test
    println!("7. Performance Test:");
    let test_data = vec![0u8; 1000];
    let iterations = 1000;
    
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let mut hasher = Poseidon::new();
        hasher.update(&test_data);
        let _hash = hasher.finalize();
    }
    let duration = start.elapsed();
    
    let throughput = (iterations as f64) / duration.as_secs_f64();
    println!("   {} hashes in {:?}", iterations, duration);
    println!("   Throughput: {:.2} hashes/second", throughput);
    println!();

    println!("=== Demo Complete ===");
} 