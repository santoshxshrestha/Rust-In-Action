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
    fn serve_v1(&self) {

        if self.age < 18 && (self.order == Liquids::Vodka || self.order == Liquids::Beer) {
            panic!("sorry {}, you're too young for {}.", self.name, self.order);
        }else{
            println!("Your order of {} is ready {}.",self.order, self.name);
        }
    }

    fn serve_v2(&self) {
        if self.age < 18 {
            match self.order {
                Liquids::Beer => println!("Are you crazy boy you are just {} and ordering Beer.",self.age),
                Liquids::Vodka => println!("Hell nah! I am not giving Vodka to you , you are just a kid of {}.",self.age),
                _ =>  println!("Your order of {} is ready {}.",self.order, self.name),
            }
        }
        else{
            println!("Your order of {} is ready {}.",self.order, self.name);
        }
    }

    fn serve_v3(&self) {
        if self.age<18 {
            
        }
    }


}



fn main() {
    let kiran = Customer{
        name: "Kiran",
        age: 21,
        order: Liquids::Beer,
    };
    kiran.serve_v1();
    println!();

    let kamal = Customer{
        name: "Kamal",
        age: 42,
        order: Liquids::Water,
    };
    kamal.serve_v1();
    println!();

    let samip = Customer{
        name: "Samip",
        age: 9,
        order: Liquids::Beer,
    };
    samip.serve_v1();
    println!();
    //after the thread gets panicked the code will stop running if panic is runned

    let santosh = Customer{
        name: "Santosh",
        age: 20,
        order: Liquids::Fanta,
    };
    santosh.serve_v1();
    println!();

    let nahida = Customer {
        name: "Nahida",
        age: 8,
        order: Liquids::Cocacola,
    };
    nahida.serve_v1();
    println!();
}
