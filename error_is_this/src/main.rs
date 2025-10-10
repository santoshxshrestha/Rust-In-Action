use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("You have to put there some name babe")]
    NoName,

    #[error("Who the hell are you {0}")]
    DontKnowYou(String),

    #[error("what the fwak why are you here {0}")]
    WhyYouHere(String),
}

fn greet(name: &str) -> Result<String, Errors> {
    match name.trim() {
        "Anish" => Err(Errors::DontKnowYou(name.to_string())),
        "Kamal" => Err(Errors::WhyYouHere(name.to_string())),
        "" => Err(Errors::NoName),
        _ => Ok(format!("Hello there {}", name)),
    }
}

fn main() {
    let anish = "Anish";
    match greet(&anish) {
        Ok(message) => println!("message: {}", message),
        Err(e) => eprintln!("message: {}", e),
    }

    let kamal = "Kamal";
    match greet(&kamal) {
        Ok(message) => println!("message: {}", message),
        Err(e) => eprintln!("message: {}", e),
    }

    let hair = " ";
    match greet(&hair) {
        Ok(message) => println!("message: {}", message),
        Err(e) => eprintln!("message: {}", e),
    }
}
