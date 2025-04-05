//this way is better with many different use cases out there
//you can't manually returen the &str with out putting the life time 
//parameter there 
//you can't manually returen the &str with out putting the life time 
//parameter there 
fn get_color() -> &'static str {
    "red"
}

fn calling_get_color() {
    println!("{}",get_color());
}

fn another_way_of_doing_things(){
    println!("red");
}

//some usages of the trait 
trait Color {
    fn name(&self) -> &'static str;
}

struct Apple;

struct Sky;

impl Color for Apple {
    fn name(&self) -> &'static str {
        "Red"
    }
}

impl Color for Sky {
    fn name(&self) -> &'static str {
        "Blue"
    }
}

fn main() {
    let color = get_color();
    println!("{}",color);

    calling_get_color();

    println!("{}",get_color());
    another_way_of_doing_things();

    let sky = Sky;
    println!("Color of the sky is {}",Sky::name(&sky));

    println!("Color of the apple is {}",Apple::name(&Apple));
    {
        trait Color {
            fn name(&self) -> &'static str; 
        }

        struct Apple;    
        struct Sky;
        struct Grass;

        impl Color for Apple {
            fn name(&self)-> &'static str {
                "Red"
            }
        }

        impl Color for Sky {
            fn name(&self)-> &'static str {
                "Blue"
            }
        }
        impl Color for Grass {
            fn name(&self)-> &'static str {
                "Green"
            }
        }
        //here the print_color function reduces the reduntency of the code
        fn print_color<T: Color>(item: &T, name: &str){
            println!("The color of {} is {}",name,item.name());
        }

        //content inside the main fn so no need of the fn main() again here
        let apple = Apple;
        let sky = Sky;
        let grass = Grass;

        print_color(&apple, "apple");
        print_color(&sky, "sky");
        print_color(&grass, "grass");

    }


}
