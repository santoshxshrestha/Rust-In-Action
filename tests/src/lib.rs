//we can test the private function too
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn subtract(left: i32, right: i32) -> i32 {
    left - right
}

pub fn bad_sub(left: i32, right: i32) -> i32 {
    left + right
}

pub fn divide_non_zero_result(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("Divide-by-zero-error");
    }else if a<b {
        panic!("Divide result is zero");
        
    }else {
        a/b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_bad_sub() {
        assert_eq!(bad_sub(2, 1),1);
    }

    #[test]
    fn good_add() {
        assert_eq!(add(2, 1),3);
    }

    #[test]
    fn good_subtract() {
        assert_eq!(subtract(4,2),2);
    }


    #[test]
    fn test_divide() {
        assert_eq!(divide_non_zero_result(4,2),2);
    }

    #[test]
    #[should_panic]
    fn test_any_panic() {
        divide_non_zero_result(1, 3);
    }

    #[test]
    #[should_panic = "Divide result is zero"]
    fn test_specific_panic() {
        divide_non_zero_result(1, 3);
    }

    //this will failed because here we are selected some thing different
    #[test]
    #[ignore]
    #[should_panic(expected = "Divide-by-zero-error")]
    fn test_specific_panic_wrong() {
        divide_non_zero_result(1, 3);
    }



}
