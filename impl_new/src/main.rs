#![allow(dead_code)]
struct Laptops<'a>{
    name: &'a str,
    price: &'a i64
}

impl<'a> Laptops<'a>{
    fn new(name: &'a str, price: &'a i64)->Self{
        Self{name, price}
    }
}
impl<'a> std::fmt::Display for Laptops<'a>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} and price is ${}",self.name, self.price)
    }
}



fn main() {
    let acer_v_15 = Laptops::new("acer_v_15", &95000);
    println!("The new laptop available here is {}",acer_v_15);
}
