use std::collections::hash_set;
fn main() {
    let mut hash_set = hash_set::HashSet::new();
    hash_set.insert("hello");
    hash_set.insert("hello");
    hash_set.insert("hello");
    hash_set.insert("hello");
    hash_set.insert("hi");
    println!("{:?}", hash_set);
}
