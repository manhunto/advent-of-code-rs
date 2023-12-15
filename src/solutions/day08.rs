use std::collections::HashMap;
use regex::Regex;
use crate::infinite_iterator::InfiniteIterator;
use crate::solutions::Solution;

pub struct Day08;

impl Solution for Day08 {
    fn part_one(&self, input: &str) -> String {
        let mut navigation = self.parse_navigation(input);
        let instructions = self.parse_instructions(input);

        let mut current: &str = "AAA";
        let mut move_count: u32 = 0;

        loop {
            let (left, right) = instructions.get(current).unwrap();
            let direction = navigation.next();

            move_count += 1;
            current = match direction {
                'R' => right,
                'L' => left,
                _ => panic!("WTF"),
            };

            if current == "ZZZ" {
                return move_count.to_string();
            }
        }
    }

    fn part_two(&self, input: &str) -> String {
        let mut navigation = self.parse_navigation(input);
        let instructions = self.parse_instructions(input);

        let mut currents: Vec<&str> = instructions
            .keys()
            .map(|c| *c)
            .filter(|c| c.ends_with('A'))
            .collect();

        let mut move_count = 0;

        loop {
            let direction = navigation.next();

            for current in currents.iter_mut() {
                let (left, right) = instructions.get(current).unwrap();

                *current = match direction {
                    'R' => right,
                    'L' => left,
                    _ => panic!("WTF"),
                };
            }

            move_count += 1;

            let ends: Vec<&str> = currents
                .iter()
                .map(|c| *c)
                .filter(|c| c.ends_with('Z'))
                .collect();

            if ends.len() == currents.len() {
                return move_count.to_string();
            }
        }
    }
}

impl Day08 {
    fn parse_navigation(&self, input: &str) -> InfiniteIterator<char> {
        let navigation = input.lines().next().unwrap();

        InfiniteIterator::new(navigation.chars().collect())
    }
    fn parse_instructions<'a>(&'a self, input: &'a str) -> HashMap<&str, (&str, &str)> {
        let lines = input.lines().skip(2);
        let re = Regex::new(r"^([0-9A-Z]{3}) = \(([0-9A-Z]{3}), ([0-9A-Z]{3})\)$").unwrap();

        let mut instructions: HashMap<&str, (&str, &str)> = HashMap::new();

        for line in lines {
            let captures = re.captures(line).unwrap();
            let node = captures.get(1).unwrap().as_str();
            let left = captures.get(2).unwrap().as_str();
            let right = captures.get(3).unwrap().as_str();

            instructions.insert(node, (left, right));
        }

        instructions
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::file_system::read_example;
    use crate::infinite_iterator::InfiniteIterator;
    use crate::solutions::day08::{Day08};
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("08");

        assert_eq!("2", Day08.part_one(&input.as_str()));
    }

    #[test]
    fn part_one_example_test2() {
        let input = read_example("08_2");

        assert_eq!("6", Day08.part_one(&input.as_str()));
    }

    #[test]
    fn part_two_example_test() {
        let input = read_example("08_3");

        assert_eq!("6", Day08.part_two(&input.as_str()));
    }

    #[test]
    fn parse_instructions_test() {
        let input = read_example("08");

        let expected: HashMap<&str, (&str, &str)> = HashMap::from([
            ("AAA", ("BBB", "CCC")),
            ("BBB", ("DDD", "EEE")),
            ("CCC", ("ZZZ", "GGG")),
            ("DDD", ("DDD", "DDD")),
            ("EEE", ("EEE", "EEE")),
            ("GGG", ("GGG", "GGG")),
            ("ZZZ", ("ZZZ", "ZZZ")),
        ]);

        assert_eq!(expected, Day08.parse_instructions(&input.as_str()));
    }

    #[test]
    fn parse_navigation_test() {
        let input = read_example("08");

        let expected: InfiniteIterator<char> = InfiniteIterator::new(vec!['R', 'L']);

        assert_eq!(expected, Day08.parse_navigation(&input));
    }
}
