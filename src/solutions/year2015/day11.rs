use crate::solutions::Solution;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub struct Day11;

impl Solution for Day11 {
    fn part_one(&self, input: &str) -> String {
        Password::<8>::from_str(input.trim())
            .unwrap()
            .increment()
            .unwrap()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

#[derive(Debug, PartialEq)]
struct Password<const N: usize> {
    chars: [u8; N],
}

impl<const N: usize> FromStr for Password<N> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != N {
            return Err(format!("Expected {} bytes, got {}", s.len(), N));
        }

        let mut chars = [0u8; N];
        chars.copy_from_slice(s.as_bytes());

        Ok(Self { chars })
    }
}

impl<const N: usize> Password<N> {
    fn increment(&self) -> Result<Self, String> {
        let mut chars = self.chars;
        let mut i = N - 1;

        loop {
            if chars[i] + 1 > b'z' {
                chars[i] = b'a';

                if i == 0 {
                    return Err("Password length overflow".into());
                }

                i -= 1;
                continue;
            } else {
                chars[i] += 1;
                break;
            }
        }

        Ok(Password { chars })
    }
}

impl<const N: usize> Display for Password<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.chars))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_increment() {
        assert_eq!(
            Password::<2>::from_str("xy"),
            Password::from_str("xx").unwrap().increment()
        );
        assert_eq!(
            Password::<2>::from_str("xz"),
            Password::from_str("xy").unwrap().increment()
        );
        assert_eq!(
            Password::<2>::from_str("ya"),
            Password::from_str("xz").unwrap().increment()
        );
        assert_eq!(
            Password::<2>::from_str("yb"),
            Password::from_str("ya").unwrap().increment()
        );
    }

    #[test]
    fn password_increment_overflow() {
        assert!(Password::<2>::from_str("zz").unwrap().increment().is_err());
    }

    #[test]
    fn password_display() {
        assert_eq!("zz", Password::<2>::from_str("zz").unwrap().to_string());
    }
}
