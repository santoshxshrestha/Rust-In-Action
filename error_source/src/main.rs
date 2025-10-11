use thiserror::Error;

#[derive(Error, Debug)]
pub enum AgeError {
    #[error("your age is less then zero")]
    LessThenZero(String),

    #[error("you are just born")]
    EqualToZero(String),

    #[error("this is the custom error that you are getting")]
    CustomError {
        #[source]
        source: Box<dyn std::error::Error + 'static>,
        description: String,
    },
}

pub struct Age {
    age: i32,
}
pub struct Class{
    main: String,
    func: String,

}


fn check_errors(age: Age) -> Result<String, AgeError> {
    if age.age < 0 {
        return Err(AgeError::LessThenZero(
            "your age is in negative".to_string(),
        ));
    } else if age.age == 0 {
        return Err(AgeError::EqualToZero("welcome to the earth".to_string()));
    } else if age.age > 0 {
        return Ok(format!("you got the following error{}", age.age));
    } else {
        Err(AgeError::CustomError {
            source
            description: "the error you just got is called custom error".to_string(),
        })
    }
}

fn main() {}
