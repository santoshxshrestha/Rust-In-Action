struct Counter<'a> {
    counter: &'a mut i32,
}

impl Counter<'_> {
    fn increment(&mut self) {
        *self.counter += 1;
    }
}

fn main() {
    let mut num = 0;

    let mut counter = Counter { counter: &mut num };
    counter.increment();

    println!("{num}"); // prints 1
}
