use anyhow::{Result, anyhow, ensure};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MeroErrors {
    #[error("less than 10:{0}")]
    LessThen10(String),

    #[error("sub by 5 error: {0}")]
    SubBy5(String),
}
pub fn count_again(num: i32) -> Result<String> {
    ensure!(
        num > 10,
        MeroErrors::LessThen10("your number is less then 10".to_string())
    );
    let sub_by_5 = num - 5;
    ensure!(
        sub_by_5 > 10,
        MeroErrors::SubBy5("after sub by 5 the number is less then 10".to_string())
    );
    Ok(format!(
        "correct: your number is 🥢{sub_by_5} ⭕ greater then 10"
    ))
}

pub fn count(num: i32) -> Result<String> {
    if num < 10 {
        return Err(anyhow!(
            "your number is less then 10 so you got this error:{}",
            num
        ));
    };
    Ok(format!("your number is greater then 10: {}", num))
}

fn main() {
    // match count(10) {
    //     Ok(message) => println!("pass: {}", message),
    //     Err(e) => eprintln!("error:{}", e),
    // }
    match count_again(20) {
        Ok(message) => println!("passed:{:?}", message),
        Err(e) => eprintln!("error: {}", e),
    }

    match count_again(12) {
        Ok(message) => println!("passed:{:?}", message),
        Err(e) => eprintln!("error: {}", e),
    }
    match count_again(5) {
        Ok(message) => println!("passed:{:?}", message),
        Err(e) => eprintln!("error: {}", e),
    }
}
