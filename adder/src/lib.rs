pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    //this depends on the pretty_assertions crate is added to the Cargo.toml file
    //which makes the output of the assert_eq! macro more readable

    #[test]
    fn test_add(){
        assert_eq!(add(1, 3), 5);
    }
}
