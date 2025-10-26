use core::f64;

fn main() {
    let x: u64 = 2402783029;
    let mega_bytes = x as f64 / 1000000 as f64;
    println!("{} MB", mega_bytes);
}
