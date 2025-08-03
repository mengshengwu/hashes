use poseidon::Poseidon;
use digest::Digest;

fn main() {
    // Create a new Poseidon hasher
    let mut hasher = Poseidon::new();
    
    // Update with some data
    hasher.update(b"Hello, Poseidon!");
    
    // Get the final hash
    let result = hasher.finalize();
    
    println!("Poseidon hash: {:?}", result);
    println!("Hash length: {} bytes", result.len());
    
    // Demonstrate incremental hashing
    let mut hasher2 = Poseidon::new();
    hasher2.update(b"Hello, ");
    hasher2.update(b"Poseidon!");
    let result2 = hasher2.finalize();
    
    println!("Incremental hash: {:?}", result2);
    println!("Hashes are equal: {}", result == result2);
    
    // Demonstrate different inputs produce different outputs
    let mut hasher3 = Poseidon::new();
    hasher3.update(b"Different input");
    let result3 = hasher3.finalize();
    
    println!("Different input hash: {:?}", result3);
    println!("Different from original: {}", result != result3);
} 