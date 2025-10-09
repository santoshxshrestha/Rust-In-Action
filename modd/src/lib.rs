pub fn add(a: i32, b: i32) -> i32 {
    let sum = a + b;
    return sum;
}

pub fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 { Err("Infinity") } else { Ok(a / b) }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn addition_test() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    pub fn additon_from_subss() {
        assert_eq!(add(-12, -2), -14);
    }

    #[test]
    pub fn dividing_normal() {
        assert_eq!(divide(4, 2), Ok(2));
    }

    #[test]
    pub fn divide_by_zero() {
        assert_eq!(divide(4, 0), Err("Infinity"))
    }

    #[test]
    pub fn divide_to_zero() {
        assert_eq!(divide(0, 1), Ok(0));
    }
}
