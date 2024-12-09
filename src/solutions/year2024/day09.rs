use crate::solutions::Solution;
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub struct Day09;

impl Solution for Day09 {
    fn part_one(&self, input: &str) -> String {
        let mut disk_map = DiskMap::from_str(input).unwrap();

        loop {
            let last_digit_index = disk_map.blocks.iter().rposition(|v| v.is_some()).unwrap();
            let first_empty_index = disk_map.blocks.iter().position(|v| v.is_none()).unwrap();

            if first_empty_index > last_digit_index {
                break;
            }

            disk_map.blocks[first_empty_index] = disk_map.blocks[last_digit_index].take();
        }

        disk_map.checksum().to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from('0')
    }
}

struct DiskMap {
    blocks: Vec<Option<usize>>,
}

impl DiskMap {
    fn checksum(&self) -> usize {
        self.blocks
            .clone()
            .into_iter()
            .flatten()
            .enumerate()
            .fold(0, |acc, (i, id)| acc + i * id)
    }
}

impl FromStr for DiskMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut current_id = 0;

        let test: Vec<Option<usize>> = s
            .trim()
            .chars()
            .enumerate()
            .flat_map(|(i, c)| {
                let times: usize = c
                    .to_string()
                    .parse()
                    .unwrap_or_else(|_| panic!("cannot parse char to usize: '{}'", c));

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

    const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn part_one_example_test() {
        assert_eq!("1928", Day09.part_one(EXAMPLE));
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
