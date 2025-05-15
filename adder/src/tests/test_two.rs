mod common;

use my_project::add;

#[test]
fn test_add_negative_numbers() {
    common::setup(); // shared setup
    assert_eq!(add(-2, -3), -5);
}

