use crate::solutions::Solution;
use itertools::Itertools;

pub struct Day03;

impl Solution for Day03 {
    fn part_one(&self, input: &str) -> String {
        self.parse(input)
            .filter(|lengths| self.is_valid_triangle(lengths))
            .count()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day03 {
    fn parse<'a>(&self, input: &'a str) -> impl Iterator<Item = [u16; 3]> + 'a {
        input.lines().map(|line| {
            let (a, b, c) = line
                .split_whitespace()
                .map(|s| s.parse::<u16>().unwrap())
                .collect_tuple()
                .unwrap();

            [a, b, c]
        })
    }

    fn is_valid_triangle(&self, lengths: &[u16; 3]) -> bool {
        let mut l = *lengths;
        l.sort_unstable();

        l[2] < l[0] + l[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_triangle3() {
        assert!(!Day03.is_valid_triangle(&[5, 10, 25]));
        assert!(!Day03.is_valid_triangle(&[5, 10, 15]));
        assert!(Day03.is_valid_triangle(&[5, 10, 14]));
    }
}
