use crate::solutions::Solution;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub struct Day11;

impl Solution for Day11 {
    fn part_one(&self, input: &str) -> String {
        let mut password = Password::<8>::from_str(input.trim())
            .unwrap()
            .increment()
            .unwrap();

        while !password.is_valid() {
            password = password.increment().unwrap()
        }

        password.to_string()
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
                //fixme: if current letter is ambiguous we can skip to the next (optimization)
                chars[i] += 1;
                break;
            }
        }

        Ok(Password { chars })
    }

    fn contains_increasing_letters(&self, length: usize) -> bool {
        for i in 0..N - length + 1 {
            if self.chars[i] + 1 == self.chars[i + 1] && self.chars[i + 1] + 1 == self.chars[i + 2]
            {
                return true;
            }
        }

        false
    }

    fn not_contains_ambiguous_letters(&self) -> bool {
        for i in 0..N {
            if self.chars[i] == b'i' || self.chars[i] == b'o' || self.chars[i] == b'l' {
                return false;
            }
        }

        true
    }

    fn contains_nonoverlapping_two_pairs(&self) -> bool {
        let mut i = 0;
        let mut pairs = 0;

        while i < N - 1 {
            if self.chars[i] == self.chars[i + 1] {
                pairs += 1;
                i += 2;
            } else {
                i += 1;
            }
        }

        pairs >= 2
    }

    fn is_valid(&self) -> bool {
        self.not_contains_ambiguous_letters()
            && self.contains_increasing_letters(3)
            && self.contains_nonoverlapping_two_pairs()
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

    #[test]
    fn password_contains_increasing_letters() {
        assert!(Password::<3>::from_str("xyz")
            .unwrap()
            .contains_increasing_letters(3));

        assert!(Password::<4>::from_str("axyz")
            .unwrap()
            .contains_increasing_letters(3));

        assert!(!Password::<3>::from_str("abd")
            .unwrap()
            .contains_increasing_letters(3));
    }

    #[test]
    fn password_not_contains_ambiguous_letters() {
        assert!(Password::<2>::from_str("zz")
            .unwrap()
            .not_contains_ambiguous_letters());
        assert!(!Password::<2>::from_str("zi")
            .unwrap()
            .not_contains_ambiguous_letters());
        assert!(!Password::<2>::from_str("oa")
            .unwrap()
            .not_contains_ambiguous_letters());
        assert!(!Password::<2>::from_str("ul")
            .unwrap()
            .not_contains_ambiguous_letters());
    }

    #[test]
    fn password_contains_nonoverlapping_two_pairs() {
        assert!(Password::<4>::from_str("aabb")
            .unwrap()
            .contains_nonoverlapping_two_pairs());
        assert!(!Password::<3>::from_str("aaa")
            .unwrap()
            .contains_nonoverlapping_two_pairs());
        assert!(Password::<4>::from_str("aaaa")
            .unwrap()
            .contains_nonoverlapping_two_pairs());
        assert!(!Password::<5>::from_str("baaab")
            .unwrap()
            .contains_nonoverlapping_two_pairs());
    }

    #[test]
    fn password_is_valid() {
        assert!(!Password::<8>::from_str("hijklmmn").unwrap().is_valid());
        assert!(!Password::<8>::from_str("abbceffg").unwrap().is_valid());
        assert!(!Password::<8>::from_str("abbcegjk").unwrap().is_valid());
        assert!(Password::<8>::from_str("abcdffaa").unwrap().is_valid());
        assert!(Password::<8>::from_str("ghjaabcc").unwrap().is_valid());
    }

    #[test]
    fn part_one_example() {
        // assert_eq!("abcdffaa", Day11.part_one("abcdefgh"));
        assert_eq!("ghjaabcc", Day11.part_one("ghijklmn"));
    }
}
