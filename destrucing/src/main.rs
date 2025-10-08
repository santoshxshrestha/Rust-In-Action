pub enum MiddleName {
    None,
    Some(String),
}

impl MiddleName {
    pub fn some(name: &str) -> Self {
        Self::Some(name.to_string())
    }

    pub fn none() -> Self {
        Self::None
    }
}

pub struct Name {
    first_name: String,
    middle_name: MiddleName,
    last_name: String,
}

impl Name {
    fn new(first_name: &str, middle_name: MiddleName, last_name: &str) -> Self {
        Self {
            first_name: first_name.to_string(),
            middle_name: middle_name,
            last_name: last_name.to_string(),
        }
    }

    fn display(self) {
        match self.middle_name {
            MiddleName::Some(middle_name) => {
                println!("{} {} {}", self.first_name, middle_name, self.last_name)
            }
            MiddleName::None => println!("{} {}", self.first_name, self.last_name),
        }
    }
}

fn main() {
    let santosh = Name::new("Santosh", MiddleName::None, "Shrestha");
    santosh.display();

    let hari = Name::new("Hari", MiddleName::Some("Parsad".to_string()), "Rimal");
    hari.display();

    // here we are not doing the to string
    let samip = Name::new("Samip", MiddleName::some("Bahadur"), "Shrestha");
    samip.display();
}
