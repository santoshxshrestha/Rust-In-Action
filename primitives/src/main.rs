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
    {
        //creating array
        let my_array: [i32; 5] = [1, 2, 4, 5, 65];
        //creating tuple
        let my_tupul = (5u32, 5i32, 5i32, 6u32, 65i32, 53.62f32);
        println!("my array is {:#?}", my_array);
        println!("my tuple is {:#?}", my_tupul);
    }
    {
        //enotation
        let val1 = 2e-20;

        let val2 = 2e20;
        let val3 = 2E20;
        println!("the val1 is {val1}");
        println!("the val2 is {val2}");
        println!("the val3 is {val3}");
    }
}
