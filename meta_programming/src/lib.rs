// https://doc.rust-lang.org/book/ch20-05-macros.html
//
#[macro_export]
macro_rules! my_mecro {
    ( $( $expression_name:expr),* ) => {
        {
        let mut string = String::new();
        $(
            string.push_str(&format!("{} ", $expression_name));
        )*
        string
        }
    };
}

#[cfg(test)]
mod tests {
    use super::my_mecro;

    #[test]
    fn test_my_macro() {
        let result = my_mecro!("Hello", "world!", 123, 45.6);
        assert_eq!(result, "Hello world! 123 45.6 ");
    }
}
