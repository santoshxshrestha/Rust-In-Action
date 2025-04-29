#![allow(unused)]

use std::fmt::format;
#[derive(PartialEq,Clone, Copy)]
enum Liquids {
    Water,
    Beer,
    Cocacola,
    Fanta,
    Vodka,
}

impl std::fmt::Display for Liquids {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Liquids::Water => write!(f,"Water"),
            Liquids::Beer => write!(f,"Beer"),
            Liquids::Cocacola => write!(f,"Cocacola"),
            Liquids::Fanta => write!(f,"Fanta"),
            Liquids::Vodka => write!(f,"Vodka"),
        }
    }
    
}

struct Customer<'a> {
    name: &'a str,
    age: u8,
    order: Liquids,
}

impl<'a> Customer<'a> {
    fn new(name: &'a str, age: u8, order: Liquids) -> Self {
        Customer { name, age, order}
    }
    fn serve(&self) {
        if self.age < 18 && (self.order == Liquids::Vodka || self.order == Liquids::Beer) {
            panic!("sorry {}, you're too young for {}.", self.name, self.order);
        }else{
            println!("Your order of {} is ready {}.",self.order, self.name);
        }

    }


}



fn main() {
    let kiran = Customer{
        name: "Kiran",
        age: 21,
        order: Liquids::Beer,
    };

    kiran.serve();

    let kamal = Customer{
        name: "Kamal",
        age: 42,
        order: Liquids::Water,
    };
    kamal.serve();

    let samip = Customer{
        name: "Samip",
        age: 9,
        order: Liquids::Beer,
    };
    samip.serve();
    //after the thread gets panicked the code will stop running 

    let santosh = Customer{
        name: "Santosh",
        age: 20,
        order: Liquids::Fanta,
    };
    santosh.serve();
}
