fn main() {
    #![allow(dead_code)]
    enum Temperature {
        Celsius(i32),
        Fahrenheit(i32),
        Kelvin(i32),
    }

    let temp = Temperature::Celsius(32);
    match temp {
        Temperature::Celsius(c) if c < 12 => {
            println!("Too cold. The temperature is less then 12 , it is {}C", c)
        }
        Temperature::Celsius(c) if c < 32 => {
            println!(
                "It's a nice day. The temperature is less then 32, it is  {}C",
                c
            )
        }

        Temperature::Celsius(c) if c == 32 => {
            println!("Too hot. The  is more then 32, it is {}C", c)
        }
        Temperature::Celsius(c) if c > 32 => {
            println!("Too hot. The temperature is more then 32, it is {}C", c)
        }
        Temperature::Celsius(c) if c > 50 => {
            println!("Too hot. The temperature is more then 50, it is {}C", c)
        }
        _ => println!("Ohh the thermometer is broken"),
    }
}
