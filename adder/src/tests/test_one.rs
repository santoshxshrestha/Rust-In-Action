mod common;
use adder::add;

#[test]
fn test_add_positive_number() {
    common::setup();
    assert_eq!( add(2,3), 5);
}
