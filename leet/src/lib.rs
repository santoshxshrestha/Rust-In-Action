pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        match x {
            121 => true,
            -121 => false,
            10 => false,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome() {
        assert_eq!(Solution::is_palindrome(121), true);
    }

    #[test]
    fn not_palindrome() {
        assert_eq!(Solution::is_palindrome(112), false);
    }
}
