fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        //here .iter returns an iterator and the type of name is &&str
        //so we need to dereference it twice to get the value of the string
        match *name {
            "Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);

    {
        let names = vec!["Bob", "Frank", "Ferris"];
        //here the value of names is moved to the for loop so we have to clone it to use it later
        //in the println statement
        for name in names.clone().into_iter() {
            match name {
                "Ferris" => println!("There is a rustacean among us!"),
                _ => println!("Hello {}", name),
            }
        }
        println!("names: {:?}", names);
    }
    {
        let mut names = vec!["Bob", "Frank", "Ferris"];
        //here the value of names is moved to the for loop mutably so we can use it later and also
        //modify it
        for name in names.iter_mut() {
            *name = match name {
                &mut "Ferris" => "There is a rustacean among us!",
                _ => "Hello",
            }
        }

        println!("names: {:?}", names);
    }
}
