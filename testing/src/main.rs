fn add<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    return a + b;
}
fn main() {
    let a = 3;
    let b = 23;

    println!(" the sum of  two number a and b is: {} ", add(a, b));
}
