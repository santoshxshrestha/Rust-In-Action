#![allow(unused)]
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// `<'a: 'b, 'b>` reads as lifetime `'a` is at least as long as `'b`.
// Here, we take in an `&'a i32` and return a `&'b i32` as a result of coercion.
// so here 'a is treated like hte 'b so we are able to use &'b i32 to the val that is
// is going to be returned here
fn choose_first<'a: 'b, 'b>(first: &'a i32, second: &'b i32) -> &'b i32 {
    first
}

//this is the one which we would use 
fn choose_first_another<'a: 'b, 'b>(first: &'a i32, second: &'b i32) -> &'a i32 {
    first
}

fn choose_second<'a: 'b, 'b>(first: &'a i32, second: &'b i32) -> &'b i32 {
    second
}

fn main() {
    let first = 2; // Longer lifetime
    
    {
        let second = 3; // Shorter lifetime
        
        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
        println!("{} is the second", choose_second(&first, &second));
    };
}
