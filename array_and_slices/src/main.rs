use std::mem;
fn slice_fn(array: &[i32]) {
    println!(
        "The second element of the slice of array in fn is: {}",
        array[1]
    );
}

fn main() {
    // All elements can be initializ
    let ys: [i32; 500] = [0; 500];

    println!("The count of the elements in the array is: {} ", ys.len());
    // println!("{:?}", ys);
    let array: [i32; 5] = [0, 1, 51, 51, 5];
    println!("The val in the array of the 4th index is: {}", array[3]);

    println!(
        "The count of the elements in the array is: {} ",
        array.len()
    );

    println!("The array occupies {} bytes", mem::size_of_val(&array));

    let slice = &array[0..5];
    println!("The slice is {:?}", slice);

    {
        let empty_array: [u32; 0] = [];
        println!(
            "Here is the empty_array that you want to print: {:?}",
            &empty_array
        );
        assert_eq!(&empty_array, &[]);
        assert_eq!(&empty_array, &[][..]);
    }

    //slice of array for funtion
    let array: [i32; 10] = [1, 41, 41, 14, 151, 231, 2512, 31, 52, 31];
    slice_fn(&array[1..6]);

    {
        let array = ["Anil", "Kumar", "Rohit", "Diwas", "Hair", "Santosh"];
        match array {
            [other @ .., "Santosh"] => {
                println!(
                    "Here is the rustacean: {} and other are {:?}",
                    array[(array.len()) - 1],
                    other,
                )
            }
            _ => println!("No rustacean found"),
        }
    }
}
