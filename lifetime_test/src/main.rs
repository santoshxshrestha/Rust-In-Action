use std::fmt::Display;
fn lifetime_with_generics<'lifetime1, 'lifetime2, 'lifetime3, A, B, C>(
    first: &'lifetime1 A,
    second: &'lifetime2 B,
    third: &'lifetime3 C,
) -> &'lifetime3 C
where
    A: Display,
    B: Display,
    C: Display,
{
    println!("The first content is {first}");
    println!("The second content is {second}");
    println!("The third content is {third}");
    return third;
}

fn main() {
    let first = 32;
    let second = 43;
    let third = 423;
    println!(
        "Got the refrence to the thrid val {}",
        lifetime_with_generics(&first, &second, &third)
    );
}
