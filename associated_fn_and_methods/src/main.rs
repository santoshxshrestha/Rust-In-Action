struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }
}

impl Person {
    fn demon(self) {
        println!("Consuming {} whose age is {}.", self.name, self.age)
    }
}

fn main() {
    let person = Person::new("John".to_string(), 32);
    person.demon()
}
