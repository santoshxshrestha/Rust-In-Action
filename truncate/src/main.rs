//this will only work for vector not for array
fn main() {
    let mut vector = vec![0; 1024];
    vector.truncate(2);
    println!("The content of the vector is : {:?}", vector);
}
