use std::fmt;

struct MyResult(Result<String, String>);

impl fmt::Display for MyResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Ok(val) => write!(f, "Success: {}", val),
            Err(err) => write!(f, "Error: {}", err),
        }
    }
}

fn result_break(age: i32) -> Result<String, String> {
    if age < 18 {
        Err("hello kiddo".to_string())
    } else if age == 18 {
        Err("hello kid".to_string())
    } else {
        Ok("hello bro".to_string())
    }
}

fn main() {
    let r = MyResult(result_break(21));
    println!("{}", r);
}
