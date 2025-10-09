#[test]
fn test_good_one() {
    let good_one: Result<i32, i32> = Ok(13);
    assert!(good_one.is_ok());
}
