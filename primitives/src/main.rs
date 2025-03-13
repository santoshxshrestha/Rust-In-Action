fn main() {
    let boolval0: bool = true;
    let boolval1 = true;
    println!("{}", boolval0);
    println!("{}", boolval1);
    //here both works fine because of the the rust infering the types

    //normal or regular annotation
    let some: i32 = 24;
    println!("{some}");
    //this is caled suffix annotation
    let another = 44i32;
    println!("{another}");
}
