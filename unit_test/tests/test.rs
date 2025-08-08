use unit_test::adder;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_addition() {
        assert_eq!(adder(3, 5), 10);
    }
}
