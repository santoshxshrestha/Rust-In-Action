#![allow(unused_variables)]
#![allow(unused)]

use std::fmt::Pointer;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let x = "s";
    //they both does the same thing here
    let ref y = x;
    let z = &x;

    //here were are comparing the &&str with &&str
    println!("There both the value are equal: {}", y == z);

    //here were are comparing the &str with &str
    println!("There both the value are equal: {}", *y == *z);
    //here were are comparing the str with str
    println!("There both the value are equal: {}", **y == **z);

    let origin: Point = Point { x: 0, y: 0 };

    let x_axis = {
        let Point {
            x: ref val_in_x,
            y: _,
        } = origin;
        *val_in_x
    };

    println!("The val in the x_axis is: {}", x_axis);

    let mut point = origin;
    let y_axis: i32 = {
        let Point {
            x: _,
            y: ref mut val_in_y,
        } = point;
        *val_in_y = 20;
        *val_in_y
    };
    println!("The val in the y_axis is: {}", y_axis);

    println!("so this is the original points {:?}", origin);
}
