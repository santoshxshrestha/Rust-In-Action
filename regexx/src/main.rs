#![allow(unused)]
use regex::Regex;
fn main() {
    let haystack = "The quick brown fox jumps over the lazy dog.";
    let needle = Regex::new(r"\b\w{4}\b").unwrap();
    for mat in needle.find_iter(haystack) {
        println!("Found match: {}", mat.as_str());
    }
}
