#![allow(unused)]
fn main() {
    let some_thing:i32 =  1;
    let mut same_thing: i32 = some_thing.to_owned();
    same_thing += 1;
    println!("here is the modified data{}",same_thing+1);
    println!("here is the previous data{}",some_thing);
    {
        let some_string = String::from("Here is the string thing ");
        let mut same_string = some_string.clone();
        let another_strint = ("Ths is another string");
        let last_one = same_string + another_strint;
        println!("{last_one}");
        println!("this is the string that was first created {some_string}");
    }
}
