pub fn result_break(age: i32) -> std::result::Result<String, String> {
    if age < 18 {
        Err(format!("hello kiddo"))
    } else if age == 18 {
        Err(format!("hello kid"))
    } else {
        Ok(format!("hello bro"))
    }
}

fn main() {
    match result_break(32) {
        Ok(content) => println!("{content}"),
        Err(e) => eprintln!("{e}"),
    }
}
