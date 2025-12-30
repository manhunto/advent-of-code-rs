pub trait MathSlice {
    fn lcm(&self) -> usize;
    #[allow(dead_code)]
    fn gcd(&self) -> usize;
}

pub trait Math {
    fn gcd(&self, other: Self) -> Self;
    fn lcm(&self, other: Self) -> Self;
}

impl Math for usize {
    fn gcd(&self, other: Self) -> Self {
        let mut a = *self;
        let mut b = other;

        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }

        a
    }

    fn lcm(&self, other: Self) -> Self {
        if *self == 0 || other == 0 {
            return 0;
        }

        (self / self.gcd(other)) * other
    }
}

impl MathSlice for [usize] {
    fn lcm(&self) -> usize {
        if self.is_empty() {
            return 1;
        }

        self.iter().fold(self[0], |acc, &x| acc.lcm(x))
    }

    fn gcd(&self) -> usize {
        if self.is_empty() {
            return 0;
        }

        self.iter().fold(self[0], |acc, &x| acc.gcd(x))
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::math::{Math, MathSlice};

    #[test]
    fn lcm_slice_test() {
        assert_eq!([1, 2, 8, 3].lcm(), 24);
        assert_eq!([2, 7, 3, 9, 4].lcm(), 252);
        assert_eq!([2, 3, 4, 5].lcm(), 60);
        assert_eq!([1, 2, 3, 4, 28].lcm(), 84);
        assert_eq!([4, 6, 12, 24, 30, 120].lcm(), 120);
    }

    #[test]
    fn gcd_test() {
        assert_eq!(5, 20.gcd(15));
        assert_eq!(5, 10.gcd(15));
    }

    #[test]
    fn gcd_slice_test() {
        assert_eq!([1, 2, 8, 3].gcd(), 1);
        assert_eq!([2, 7, 3, 9, 4].gcd(), 1);
        assert_eq!([2, 3, 4, 5].gcd(), 1);
        assert_eq!([2, 4].gcd(), 2);
        assert_eq!([3, 6, 9].gcd(), 3);
    }
}
