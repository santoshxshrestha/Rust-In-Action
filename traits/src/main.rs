#![allow(unused)]

use std::str::Chars;
struct Class {
    name: &'static str,
    present: bool,
    game: &'static str
}

trait Student {
    fn new(name:&'static str) -> Self;

    fn name(&self) -> &'static str;
    fn game(&self) -> &'static str;

    fn intro(&self){
        println!("Hello my name is {}. And I play {}",self.name(), self.game());
    }
}

impl Class {
    fn is_present(&self) -> bool {
        self.present
    }

    //the is_present is for the following use it has no use in the 
    //main function 
    fn call_parents(&mut self) {
        if self.is_present(){
            println!("{} is already present...",self.name);
        }else {
            println!("Calling the parents of {}.",self.name);
            //assuming that after calling the parents the students will be 
            //there in the class
            self.present = true;
        }
    }
}

impl Student for Class{
    fn new(name: &'static str) -> Class {
        Class { name, present: false, game: "football" } } 

    fn name(&self) -> &'static str {
        self.name
    }

    fn game(&self) -> &'static str {
        "football"
    }

} 

fn main() {
    let mut students = Class::new("Santosh");
    students.present = true;
    students.call_parents();
    students.intro();

}
