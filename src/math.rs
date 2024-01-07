pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

pub fn lcm(vec: Vec<usize>) -> usize {
    vec.iter().fold(*vec.first().unwrap(), |ans, val| {
        (val * ans) / (gcd(*val, ans))
    })
}

#[cfg(test)]
mod tests {
    use crate::math::{gcd, lcm};

    #[test]
    fn lcm_test() {
        assert_eq!(24, lcm(vec![1, 2, 8, 3]));
        assert_eq!(252, lcm(vec![2, 7, 3, 9, 4]));
        assert_eq!(60, lcm(vec![2, 3, 4, 5]));
        assert_eq!(84, lcm(vec![1, 2, 3, 4, 28]));
        assert_eq!(120, lcm(vec![4, 6, 12, 24, 30, 120]));
    }

    #[test]
    fn gcd_test() {
        assert_eq!(5, gcd(20, 15));
        assert_eq!(5, gcd(10, 15));
    }
}
