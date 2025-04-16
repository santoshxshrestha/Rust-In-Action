#![allow(dead_code,unused_variables)]
fn eat_val(boxed_val:Box<i32>){
    println!("This value is eaten that contained {}",boxed_val);

}

fn borrow_val(boxed_val:&Box<i32>){
    println!("Thsi val is {}",boxed_val);
}

fn borrow_int(boxed_val:&i32){
    println!("Thsi val is {}",boxed_val);
}

    
fn main() {
    let boxed = Box::new(42i32);
    let stacked = 23i32;
    borrow_val(&boxed);
    borrow_int(&stacked);
    println!("The val after the refrence {}",boxed);

    {
        let borrowed_val: &i32 = &boxed.clone();
        println!("The borrowed value is {borrowed_val}");
        eat_val(boxed);
        //val is eaten here but    I have used the clone trait in the 
        //borrowing place so I can use it here 
        println!("The borrowed value is {borrowed_val}");
    }
}
