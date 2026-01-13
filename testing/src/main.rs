use std::usize;

fn change_value(x: i32, y: &mut Vec<i32>, index: usize) {
    y[index] = x;
}

fn main() {
    let value = 12;
    let mut vec = vec![1, 2, 3, 4, 5, value];
    change_value(242, &mut vec, 8 as usize);
}
