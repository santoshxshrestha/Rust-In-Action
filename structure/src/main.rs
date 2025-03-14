#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}
struct Square {
    point: Point,
    length: f32,
}

fn rect_area(rectangle: Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = rectangle;
    let calculation = { ((x2 - x1) * (y2 - y1)).abs() };
    calculation
}

fn square(square1: Square) -> (Rectangle, f32, f32) {
    let Square {
        point: Point { x, y },
        length,
    } = square1;
    let rectangle = Rectangle {
        top_left: Point {
            x: x - length,
            y: y - length,
        },
        bottom_right: Point { x, y },
    };    (rectangle, length, length)
}
fn main() {
    let rectangle = Rectangle {
        top_left: Point { x: 5.2, y: 0.4 },
        bottom_right: Point { x: 10.3, y: 0.2 },
    };

    println!(
        "The area of the given rectange is :{} sq units",
        rect_area(rectangle)
    );
    let square0= Square {
        point: Point { x: 32.3, y: 32.42 },
        length: 32.25,
    };

    let (rect, length, height) = square(square0);
    println!(
        "The rectangle formed form the above point and length is: {:#?}.\n The length of the rectangle is: {}\n.and height of the rectangle is: {}",
        rect, length, height 
    );
}
