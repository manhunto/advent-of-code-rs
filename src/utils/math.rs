pub trait Math {
    /// Greatest common divisor
    fn gcd(&self, other: Self) -> Self;

    /// Least common multiple
    fn lcm(&self, other: Self) -> Self;

    #[allow(dead_code)]
    /// All divisors
    fn divisors(&self) -> Divisors;
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

    fn divisors(&self) -> Divisors {
        Divisors::new(*self)
    }
}

pub trait MathSlice {
    /// Greatest common divisor
    #[allow(dead_code)]
    fn gcd(&self) -> usize;

    /// Least common multiple
    fn lcm(&self) -> usize;
}

impl MathSlice for [usize] {
    fn gcd(&self) -> usize {
        if self.is_empty() {
            return 0;
        }

        self.iter().fold(self[0], |acc, &x| acc.gcd(x))
    }

    fn lcm(&self) -> usize {
        if self.is_empty() {
            return 1;
        }

        self.iter().fold(self[0], |acc, &x| acc.lcm(x))
    }
}

pub struct Divisors {
    target_number: usize,
    current_candidate: usize,
    step: usize,
    pending_large_divisors: Vec<usize>,
}

impl Divisors {
    #[allow(dead_code)]
    pub fn new(target_number: usize) -> Self {
        // Optimization 1: If n is odd, we only check odd numbers (step by 2).
        // If n is even, we must check everything (step by 1).
        let step = if target_number.is_multiple_of(2) {
            1
        } else {
            2
        };

        Divisors {
            target_number,
            current_candidate: 1,
            step,
            pending_large_divisors: Vec::with_capacity(8), // Optimization 2: Pre-allocate a small buffer
        }
    }
}

impl Iterator for Divisors {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // Optimization 3: Use (i <= n / i) instead of (i * i <= n)
        // This prevents integer overflow for very large numbers near usize::MAX
        while self.current_candidate <= self.target_number / self.current_candidate {
            let candidate = self.current_candidate;

            // Optimization 4: Compute quotient and remainder in one go
            // (LLVM usually does this, but explicit code ensures it)
            let paired_large_divisor = self.target_number / candidate;
            let reminder = self.target_number % candidate;

            self.current_candidate += self.step;

            if reminder == 0 {
                if candidate != paired_large_divisor {
                    self.pending_large_divisors.push(paired_large_divisor);
                }

                return Some(candidate);
            }
        }

        self.pending_large_divisors.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod gcd {
        use super::*;

        #[test]
        fn test_basic_two_numbers() {
            assert_eq!(5, 20.gcd(15));
            assert_eq!(5, 10.gcd(15));
            assert_eq!(1, 13.gcd(17)); // Primes
        }

        #[test]
        fn test_zero_handling() {
            assert_eq!(5.gcd(0), 5);
            assert_eq!(0.gcd(5), 5);
            assert_eq!(0.gcd(0), 0);
        }

        #[test]
        fn test_commutativity() {
            let a = 48;
            let b = 18;
            assert_eq!(a.gcd(b), b.gcd(a));
        }

        #[test]
        fn test_slice_basic() {
            assert_eq!([1, 2, 8, 3].gcd(), 1);
            assert_eq!([2, 4].gcd(), 2);
            assert_eq!([3, 6, 9].gcd(), 3);
        }

        #[test]
        fn test_slice_edge_cases() {
            let empty: [usize; 0] = [];
            assert_eq!(empty.gcd(), 0);

            // Single element
            assert_eq!([10].gcd(), 10);

            // Slice containing 0 (gcd of set including 0 is gcd of non-zero elements)
            assert_eq!([10, 20, 0].gcd(), 10);
        }
    }

    mod lcm {
        use super::*;

        #[test]
        fn test_basic_two_numbers() {
            assert_eq!(12, 4.lcm(6));
            assert_eq!(10, 2.lcm(5));
            assert_eq!(0, 0.lcm(5));
        }

        #[test]
        fn test_commutativity() {
            let a = 12;
            let b = 15;
            assert_eq!(a.lcm(b), b.lcm(a));
        }

        #[test]
        fn test_slice_basic() {
            assert_eq!([1, 2, 8, 3].lcm(), 24);
            assert_eq!([2, 3, 4, 5].lcm(), 60);
            assert_eq!([4, 6, 12, 24, 30, 120].lcm(), 120);
        }

        #[test]
        fn test_slice_edge_cases() {
            let empty: [usize; 0] = [];
            assert_eq!(empty.lcm(), 1);

            // Single element
            assert_eq!([7].lcm(), 7);

            // Slice containing 0 => Result should be 0
            assert_eq!([5, 10, 0].lcm(), 0);
        }
    }

    mod divisors {
        use super::*;

        #[test]
        fn test_small_numbers() {
            assert_eq!(1.divisors().collect::<Vec<_>>(), vec![1]);
            assert_eq!(6.divisors().collect::<Vec<_>>(), vec![1, 2, 3, 6]);
        }

        #[test]
        fn test_square_numbers() {
            assert_eq!(16.divisors().collect::<Vec<_>>(), vec![1, 2, 4, 8, 16]);
            assert_eq!(
                36.divisors().collect::<Vec<_>>(),
                vec![1, 2, 3, 4, 6, 9, 12, 18, 36]
            );
        }

        #[test]
        fn test_primes() {
            assert_eq!(101.divisors().collect::<Vec<_>>(), vec![1, 101]);
            assert_eq!(7.divisors().collect::<Vec<_>>(), vec![1, 7]);
        }

        #[test]
        fn test_zero() {
            assert_eq!(0.divisors().count(), 0);
        }

        #[test]
        fn test_large_number_no_overflow() {
            let large_prime = 1_000_000_007;
            let divs: Vec<usize> = large_prime.divisors().collect();
            assert_eq!(divs, vec![1, large_prime]);
        }
    }
}
