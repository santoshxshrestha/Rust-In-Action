// Example 1: Automatic derivation (what the compiler does for you)
#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn example_automatic() {
    // Two points with the same coordinates are equal
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    assert_eq!(p1, p2);
    
    // Using in a HashSet
    use std::collections::HashSet;
    let mut points = HashSet::new();
    points.insert(p1);
    // p2 won't be inserted because it's equal to p1
    assert!(points.contains(&p2));
}

// Example 2: Manual implementation for custom equality
struct Person {
    id: u32,
    name: String,
    email: String,
}

// Implement PartialEq to compare only by ID
impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

// Implement Eq since our equality is reflexive, symmetric, and transitive
impl Eq for Person {}

// Implement Hash to be consistent with equality (crucial!)
impl std::hash::Hash for Person {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Only hash the fields used in equality comparison
        self.id.hash(state);
    }
}

fn example_manual() {
    // Two persons with same ID but different names are equal
    let p1 = Person { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string() };
    let p2 = Person { id: 1, name: "Alice Smith".to_string(), email: "alice.smith@example.com".to_string() };
    assert_eq!(p1, p2); // Equal because they have the same ID
    
    // Using in a HashMap
    use std::collections::HashMap;
    let mut people = HashMap::new();
    people.insert(p1, "Employee");
    // Updating information for someone with the same ID
    people.insert(p2, "Manager");
    assert_eq!(people.len(), 1); // Only one entry because equality is based on ID
}

// Example 3: Implementing for a type with floating point numbers
struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

impl PartialEq for Vector3D {
    fn eq(&self, other: &Self) -> bool {
        // Careful comparison for floating point values
        (self.x - other.x).abs() < 1e-10 &&
        (self.y - other.y).abs() < 1e-10 &&
        (self.z - other.z).abs() < 1e-10
    }
}

// Note: We can implement Eq if we're sure our comparison will always say
// a value equals itself (which our implementation above does)
impl Eq for Vector3D {}

impl std::hash::Hash for Vector3D {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // For floating point, we need to be careful
        // One approach: convert to bits or quantize
        let x_bits = self.x.to_bits();
        let y_bits = self.y.to_bits();
        let z_bits = self.z.to_bits();
        
        x_bits.hash(state);
        y_bits.hash(state);
        z_bits.hash(state);
    }
}

// Example 4: Case-insensitive string
#[derive(Debug, Clone)]
struct CaseInsensitiveString(String);

impl PartialEq for CaseInsensitiveString {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}

impl Eq for CaseInsensitiveString {}

impl std::hash::Hash for CaseInsensitiveString {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash the lowercase version to match our equality definition
        self.0.to_lowercase().hash(state);
    }
}

fn example_case_insensitive() {
    use std::collections::HashSet;
    
    let s1 = CaseInsensitiveString("Hello".to_string());
    let s2 = CaseInsensitiveString("HELLO".to_string());
    let s3 = CaseInsensitiveString("hello".to_string());
    
    assert_eq!(s1, s2);
    assert_eq!(s2, s3);
    
    let mut set = HashSet::new();
    set.insert(s1);
    assert!(set.contains(&s2));
    assert!(set.contains(&s3));
    assert_eq!(set.len(), 1); // Only one unique value
}

// Example 5: Custom equality for a complex type
struct Book {
    isbn: String,
    title: String,
    author: String,
    publication_year: u32,
    price: f64,
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        // Books are equal if they have the same ISBN
        self.isbn == other.isbn
    }
}

impl Eq for Book {}

impl std::hash::Hash for Book {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Only hash the ISBN
        self.isbn.hash(state);
    }
}

fn main() {
    example_automatic();
    example_manual();
    example_case_insensitive();
    
    // Example with Book
    let book1 = Book {
        isbn: "978-1449373320".to_string(),
        title: "Programming Rust".to_string(),
        author: "Jim Blandy".to_string(),
        publication_year: 2017,
        price: 45.99,
    };
    
    let book2 = Book {
        isbn: "978-1449373320".to_string(),
        title: "Programming Rust, 2nd Edition".to_string(), // Different title
        author: "Jim Blandy and Jason Orendorff".to_string(), // Different author list
        publication_year: 2021, // Different year
        price: 49.99, // Different price
    };
    
    // Books are equal because they have the same ISBN
    assert_eq!(book1, book2);
    
    // Using in HashMap
    use std::collections::HashMap;
    let mut books = HashMap::new();
    books.insert(book1, "First floor");
    books.insert(book2, "Second floor"); // This overwrites the previous entry
    
    assert_eq!(books.len(), 1); // Only one book (by ISBN)
}

