use crate::solutions::Solution;
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub struct Day09;

impl Solution for Day09 {
    fn part_one(&self, _input: &str) -> String {
        String::from('0')
    }

    fn part_two(&self, _input: &str) -> String {
        String::from('0')
    }
}

struct DiskMap {
    blocks: Vec<Option<usize>>,
}

impl FromStr for DiskMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut current_id = 0;

        let test: Vec<Option<usize>> = s
            .chars()
            .enumerate()
            .flat_map(|(i, c)| {
                let times: usize = c.to_string().parse().unwrap();

                let value: Option<usize> = match i % 2 == 0 {
                    true => {
                        let id = Some(current_id);

                        current_id += 1;

                        id
                    }
                    false => None,
                };

                vec![value; times]
            })
            .collect();

        Ok(Self { blocks: test })
    }
}

impl Display for DiskMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = self
            .blocks
            .iter()
            .map(|v| match v {
                None => '.',
                Some(v) => (v % 10).to_string().chars().next().unwrap(),
            })
            .join("");

        write!(f, "{}", value)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day09::{Day09, DiskMap};
    use crate::solutions::Solution;
    use std::str::FromStr;

    const EXAMPLE: &str = r#""#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("0", Day09.part_one(EXAMPLE));
    }

    #[test]
    fn parse_test() {
        let result = DiskMap::from_str("12345").unwrap();
        assert_eq!("0..111....22222", result.to_string());

        let result = DiskMap::from_str("2333133121414131402").unwrap();
        assert_eq!(
            "00...111...2...333.44.5555.6666.777.888899",
            result.to_string()
        );
    }
}
