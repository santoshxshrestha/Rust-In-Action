
use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasher, Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::time::{Duration, Instant};

// A custom data structure that we'll use in our examples
#[derive(Debug, Clone, PartialEq, Eq)]
struct Product {
    id: u32,
    name: String,
    category: String,
    price: u32,  // price in cents
}

// Custom implementation of Hash for our Product
impl Hash for Product {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // We'll only hash by ID for efficiency
        self.id.hash(state);
    }
}

// A simple function to calculate a hash using DefaultHasher
fn calculate_hash<T: Hash>(item: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    item.hash(&mut hasher);
    hasher.finish()
}

// A very simple custom hasher for demonstration
#[derive(Default)]
struct SimpleHasher(u64);

impl Hasher for SimpleHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            // A very basic hash function - don't use this in production!
            self.0 = self.0.wrapping_mul(31).wrapping_add(b as u64);
        }
    }
}

// A builder for our simple hasher
#[derive(Default, Clone)]
struct SimpleHasherBuilder;

impl BuildHasher for SimpleHasherBuilder {
    type Hasher = SimpleHasher;

    fn build_hasher(&self) -> Self::Hasher {
        SimpleHasher::default()
    }
}

// Example of using a custom hashmap with our custom hasher
type SimpleHashMap<K, V> = HashMap<K, V, SimpleHasherBuilder>;

fn main() {
    // Example 1: Basic hash calculation
    let product = Product {
        id: 42,
        name: "Rust Programming Book".to_string(),
        category: "Books".to_string(),
        price: 2999,
    };
    
    let hash_value = calculate_hash(&product);
    println!("Hash of product: {}", hash_value);
    
    // Example 2: Hash consistency with custom implementation
    let product2 = Product {
        id: 42,
        name: "Rust Programming Book (2nd Edition)".to_string(), // Different name
        category: "Programming Books".to_string(),               // Different category
        price: 3499,                                             // Different price
    };
    
    // The hash should be the same since we only hash by ID
    let hash_value2 = calculate_hash(&product2);
    println!("Hash of modified product: {}", hash_value2);
    println!("Hashes are equal: {}", hash_value == hash_value2);
    
    // Example 3: Using products in a HashSet
    let mut product_set = HashSet::new();
    product_set.insert(product.clone());
    
    // Even though some fields are different, this won't be inserted due to same ID
    println!("Product2 already in set: {}", !product_set.insert(product2));
    println!("Set size: {}", product_set.len());
    
    // Example 4: Using our custom hasher with a HashMap
    let mut inventory = SimpleHashMap::default();
    inventory.insert(product.id, product.clone());
    
    println!("Retrieved product: {:?}", inventory.get(&42));
    
    // Example 5: Performance comparison between hashers
    let product_count = 10000;
    let lookup_count = 100000;
    
    // Generate a bunch of products
    let products: Vec<Product> = (0..product_count)
        .map(|i| Product {
            id: i,
            name: format!("Product {}", i),
            category: format!("Category {}", i % 10),
            price: 1000 + (i % 500),
        })
        .collect();
    
    // Standard HashMap with DefaultHasher
    let start = Instant::now();
    let mut standard_map = HashMap::new();
    for product in &products {
        standard_map.insert(product.id, product);
    }
    
    // Perform lookups
    let mut found = 0;
    for _ in 0..lookup_count {
        let id = rand::random::<u32>() % product_count;
        if standard_map.contains_key(&id) {
            found += 1;
        }
    }
    let standard_time = start.elapsed();
    
    // Custom HashMap with our SimpleHasher
    let start = Instant::now();
    let mut simple_map = SimpleHashMap::default();
    for product in &products {
        simple_map.insert(product.id, product);
    }
    
    // Perform lookups
    let mut simple_found = 0;
    for _ in 0..lookup_count {
        let id = rand::random::<u32>() % product_count;
        if simple_map.contains_key(&id) {
            simple_found += 1;
        }
    }
    let simple_time = start.elapsed();
    
    println!("\nPerformance Comparison:");
    println!("DefaultHasher: {} items found in {:?}", found, standard_time);
    println!("SimpleHasher: {} items found in {:?}", simple_found, simple_time);
    println!("SimpleHasher is {}x the speed of DefaultHasher", 
             standard_time.as_nanos() as f64 / simple_time.as_nanos() as f64);
    
    // Example 6: Hash collision detection
    println!("\nHash Collision Analysis:");
    let mut collision_count = 0;
    let mut hash_values = HashMap::new();
    
    for product in &products {
        let hash = calculate_hash(&product.id);
        if hash_values.contains_key(&hash) {
            collision_count += 1;
            println!("Collision detected between {} and {}", 
                     product.id, hash_values[&hash]);
        } else {
            hash_values.insert(hash, product.id);
        }
        
        if collision_count >= 5 {
            println!("... and potentially more collisions.");
            break;
        }
    }
    
    if collision_count == 0 {
        println!("No collisions detected among {} products", products.len());
    }
}
