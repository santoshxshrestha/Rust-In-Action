#![allow(unused)]

#[derive(Debug)]
pub struct Testing {
    pub name :String,
    pub class: i32,

}

impl Testing {
    pub fn hello() {
        println!("this is hello func of the testing struct");
    }
}

fn main() {
    let test = Testing{
        name: "hello".to_string(),
        class: 32i32,
    };
    println!("this is the content of the struct{:?}",test);

    // here we cant call it by the use of . because it doesn't takes self, &self and &mut self and
    // mut self 
    Testing::hello();

}
