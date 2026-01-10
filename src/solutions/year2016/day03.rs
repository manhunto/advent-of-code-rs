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

    fn part_two(&self, input: &str) -> String {
        self.parse_vertically(input)
            .iter()
            .filter(|lengths| self.is_valid_triangle(lengths))
            .count()
            .to_string()
    }
}

impl Day03 {
    fn parse<'a>(&'a self, input: &'a str) -> impl Iterator<Item = [u16; 3]> + 'a {
        input.lines().map(|line| {
            let (a, b, c) = self.parse_line(line).unwrap();

            [a, b, c]
        })
    }

    fn parse_vertically(&self, input: &str) -> Vec<[u16; 3]> {
        let mut cols = [Vec::new(), Vec::new(), Vec::new()];

        for line in input.lines() {
            let (a, b, c) = self.parse_line(line).unwrap();

            cols[0].push(a);
            cols[1].push(b);
            cols[2].push(c);
        }

        let flat: Vec<u16> = cols.iter().flat_map(|col| col.iter().copied()).collect();

        flat.chunks_exact(3)
            .map(|chunk| [chunk[0], chunk[1], chunk[2]])
            .collect()
    }

    fn parse_line(&self, line: &str) -> Option<(u16, u16, u16)> {
        line.split_whitespace()
            .map(|s| s.parse::<u16>().unwrap())
            .collect_tuple()
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
