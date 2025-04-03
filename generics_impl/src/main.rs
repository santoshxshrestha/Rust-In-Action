struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

struct Stri{
    stri: String,
    
}

// impl of Val
impl Val {
    fn value(&self) -> f64 {
        self.val
    }
}

//inpl of the Stri
impl Stri{
    fn value(&self)-> &String{
       &self.stri
    }

    fn consume_value(self) -> String{
        self.stri
    }
}

// impl of GenVal for a generic type `T`
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn main() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };

    println!("{}, {}", x.value(), y.value());
    println!("{}",x.value());

    let z = Stri{ stri: "santosh".to_string()};
    println!("{}",z.value());

    let a = Stri{ stri: "To be dead".to_owned()};
    println!("{} is consumed",a.consume_value());
    //here we can't use it again because we have consumed it in the implementaion of the method 
    // println!("{} can't be consumed again consumed",a.consume_value());
}
