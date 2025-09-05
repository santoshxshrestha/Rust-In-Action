pub struct Nums {
    a: i32,
    b: i32,
}

pub struct AnotherName {
    a: i32,
    b: i32,
}

pub trait Add {
    fn add(self) -> i32;
    fn message() -> String {
        String::from("This is the default message from the Add trait.")
    }
}
impl Add for Nums {
    fn add(self) -> i32 {
        self.a + self.b
    }
}
impl Add for AnotherName {
    fn add(self) -> i32 {
        self.a + self.b
    }
}

fn main() {
    let nums = Nums { a: 1, b: 2 };
    let another = AnotherName { a: 3, b: 4 };
    println!("Nums add: {}", nums.add());
    println!("AnotherName add: {}", another.add());
    println!("{}", Nums::message());
}
