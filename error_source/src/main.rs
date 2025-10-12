use core::fmt;
use std::{error::Error, fs};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AgeError {
    #[error("you age is :{0} which is less then zero \n it is not possible")]
    LessThenZero(String),

    #[error("there is no such file out there")]
    NoFile(NoSuchFile),
}

#[derive(Debug, Error)]
pub struct NoSuchFile {
    #[source]
    source: std::io::Error,
}

impl fmt::Display for NoSuchFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No such file {}", self.source)
    }
}

pub fn read_content(name: &str) -> Result<String, AgeError> {
    match fs::read_to_string(name) {
        Ok(content) => Ok(format!("the age is {}", content)),
        Err(e) => Err(AgeError::NoFile(NoSuchFile { source: e })),
    }
}

fn main() {
    let name = "santosh.txt";
    if let Err(e) = read_content(name) {
        eprint!("you got error :{:?}", e.source())
    }
}
