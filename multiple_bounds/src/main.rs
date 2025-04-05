use std::fmt::{Debug,
Display};
fn print_info<T: Debug + Display>(x: T) {
    println!("Debug: {:?}",x);
    println!("Display: {}",x);

}  

fn print_debug<T: Debug,U: Debug>(x:T,y:U){
    println!("Debug: {:?}",x);
    println!("Debug: {:?}",y);

}
fn main() {

    let name = "Sansosh".to_owned();
    let number = 34i32;
    let array = [3,52,32,523,2,352,24];
    let vector = vec![54,52,42,52,4,52,4,324];

    print_info(name);
    print_info(number);
    // print_info(array);
    // here we cant  pass array because hte array won't be there who follows the 
    // display trait 
    print_debug(array, vector);
}
