fn main() {
    let refrence = &89;
    if refrence == &89 {
        println!("The value of refrence is: {}", *refrence);
        println!("The value of refrence is: {}", refrence);
    }
    //similarly with the match statement

    match refrence {
        &val => println!("The value of refrence is: {}", val),
    }

    //derefrenceing before matching
    match *refrence {
        val => println!("The value of refrence is: {}", val),
    }

    let for_refrence = 89;
    //here copy is used because the value is a i32 which is a primitive type
    match for_refrence {
        val => println!("The value of refrence is: {}", val),
    }
    println!(
        "The value of the for_refrence after match is: {}",
        for_refrence
    );

    //again testing the same thing with the non primitive type like String
    {
        let string = String::from("Hello !");
        match string {
            ref val => println!("The value of string is: {}", val),
        }
        //here we can't use the string after the match because the ownership of the string is moved
        //to the match statement
        //but after using the ref keyword we can use the string after the match statement
        println!("The value of the string after match is: {}", string);
    }

    //now taking the mutable refrence
    {
        let vals = 24532;
        match vals {
            mut val => {
                println!("The value of vals is: {}", val);
                val = 0;
                println!("The value of vals after mutation is: {}", val);
            }
        }
    }

    {
        //doing the same thing with the string lets see the result
        let string = String::from("Hello Santosh!");
        match string {
            //here we can make it mutable because we are taking the ownership of the string
            mut val => {
                println!("The value of string is : {}", val);
                val.push_str(" How are you ?");
                println!("The value of string after mutation is: {}", val);
            }
        }
        /*
        since we are taking the ownership of the string we can't use the string after the match
        statement
        println!("The value of string after match is: {}", string);
        */
    }
    {
        /*
        doing the same thing with the string lets see the result
        but this time we are using the ref keyword
        */
        let mut string = String::from("Hello Santosh!");
        match string {
            //here we are using the ref keyword so that we can use the string after the match
            //so we have to use the mut in the main string
            ref mut val => {
                println!("The value of string is : {}", val);
                val.push_str(" How are you ?");
                println!("The value of string after mutation is: {}", val);
            }
        }
        //since we are taking the ownership of the string we can't use the string after the match
        //statement
        // println!("The value of string after match is: {}", string);
    }
}
