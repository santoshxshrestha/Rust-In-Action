fn delete(box_val: Box<i32>){
    println!("The val {} is deleted.",box_val);
}

fn main() {
    let box_with_i32 = Box::new(32i32);
    let clone_of_vla: &i32 = &box_with_i32.clone();
    delete(box_with_i32);
    println!("{clone_of_vla}");


    let mut string: String = String::new();
    string.push('s');
    println!("some thing here I found look it is : {string}");
}
