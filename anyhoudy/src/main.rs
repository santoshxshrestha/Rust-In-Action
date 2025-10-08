pub trait Defaulting {
    fn name(&self) -> String;

    fn default_message(&self) -> String {
        format!("This is the default message from {}", self.name())
    }
}

pub struct Animal {
    pub name: String,
}
impl Defaulting for Animal {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Animal {
    pub fn new(name: &str) -> Self {
        Animal {
            name: name.to_string(),
        }
    }
}

pub trait Name {
    fn printing(&self) -> String;
}
pub struct Person {
    pub name: String,
}

impl Name for Person {
    fn printing(&self) -> String {
        self.name.clone()
    }
}

impl Person {
    pub fn new(name: &str) -> Self {
        Person {
            name: name.to_string(),
        }
    }
}

pub fn greet<T: Name>(entity: T) -> String {
    format!("Hello, {}!", entity.printing())
}

pub fn curse(entity: &dyn Name) -> String {
    format!("Go away, {}!", entity.printing())
}

fn main() {
    let karan = Person::new("Karan");
    println!("{}", greet(karan));

    let dog = Animal::new("Dog");
    println!("{}", dog.default_message());
}
