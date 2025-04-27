#![allow(unused)]

use std::fmt::write;
#[derive(Debug)]
struct Meters (f64);

struct Centimeters (u64);

struct Feet (f64);

struct Height {
    name:String,
    height: Centimeters
}
impl Centimeters {
    fn to_meters(&self) -> Meters {
        Meters(
            self.0 as f64 / 100.0
        )

    }

    fn to_feet(&self) -> Feet {
        Feet(
            self.0 as f64 / 30.48 
        )
    }
    
}

impl std::fmt::Display for Meters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:.3}m",self.0)
    }
}

impl std::fmt::Display for Centimeters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.3}cm",self.0)
    }
}

impl std::fmt::Display for Feet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.3}feet",self.0)
    }
}




fn main() {
    let a = Height{
        name: "Santosh".to_owned(),
        height: Centimeters(185),
    };
    println!("Height of {} is {}",a.name, a.height);

    println!();
    println!();
    let b = Height{
        name: "Larry".to_owned(),
        height: Centimeters(112),
    };
    println!("Height of {} is {}",b.name, b.height.to_meters());

    println!();
    println!();

    let c = Height {
        name: "Strike".to_owned(),
        height: Centimeters(198),
    };
    println!("Height of {} is {} which is {} and also {}",c.name,c.height, c.height.to_meters(), c.height.to_feet());

}
