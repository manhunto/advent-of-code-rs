use crate::utils::chain_pattern_finder::Chain;
use crate::utils::math::lcm;
use crate::solutions::Solution;
use regex::Regex;
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;
use std::iter::Cycle;
use std::str::Chars;

pub struct Day08;

impl Solution for Day08 {
    fn part_one(&self, input: &str) -> String {
        let mut navigation = self.parse_navigation(input);
        let instructions = self.parse_instructions(input);

        let mut current: &str = "AAA";
        let mut move_count: u32 = 0;

        loop {
            let (left, right) = instructions.get(current).unwrap();
            let direction = navigation.next().unwrap();

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

        let currents: Vec<&str> = instructions
            .keys()
            .copied()
            .filter(|c| c.ends_with('A'))
            .collect();

        let mut chains: Vec<Chain> = currents
            .iter()
            .map(|c| Chain::new(vec![c.to_string()]))
            .collect();

        let mut processed: HashMap<usize, (usize, usize)> = HashMap::new();

        let mut watched: Vec<usize> = vec![];

        loop {
            let direction = navigation.next().unwrap();

            for (i, current) in chains.iter_mut().enumerate() {
                let (left, right) = instructions.get(current.last().as_str()).unwrap();

                let new = match direction {
                    'R' => right,
                    'L' => left,
                    _ => panic!("WTF"),
                };

                if let Vacant(entry) = processed.entry(i) {
                    if new.ends_with('Z') && !watched.contains(&i) {
                        current.push_and_start_watch(new.to_string());
                        watched.push(i);
                    } else if let Some(r) = current.push(new.to_string()) {
                        entry.insert(r);
                    }
                }
            }

            if processed.len() == chains.len() {
                let ranges: Vec<usize> = processed.values().map(|(a, b)| *b - *a + 1).collect();

                return lcm(ranges).to_string();
            }
        }
    }
}

impl Day08 {
    fn parse_navigation<'a>(&'a self, input: &'a str) -> Cycle<Chars<'_>> {
        input.lines().next().unwrap().chars().cycle()
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
    use crate::solutions::year2023::day08::Day08;
    use crate::solutions::year2023::read_2023_example;
    use crate::solutions::Solution;
    use std::collections::HashMap;

    #[test]
    fn part_one_example_test() {
        let input = read_2023_example("08");

        assert_eq!("2", Day08.part_one(input.as_str()));
    }

    #[test]
    fn part_one_example_test2() {
        let input = read_2023_example("08_2");

        assert_eq!("6", Day08.part_one(input.as_str()));
    }

    #[test]
    fn part_two_example_test() {
        let input = read_2023_example("08_3");

        assert_eq!("6", Day08.part_two(input.as_str()));
    }

    #[test]
    fn parse_instructions_test() {
        let input = read_2023_example("08");

        let expected: HashMap<&str, (&str, &str)> = HashMap::from([
            ("AAA", ("BBB", "CCC")),
            ("BBB", ("DDD", "EEE")),
            ("CCC", ("ZZZ", "GGG")),
            ("DDD", ("DDD", "DDD")),
            ("EEE", ("EEE", "EEE")),
            ("GGG", ("GGG", "GGG")),
            ("ZZZ", ("ZZZ", "ZZZ")),
        ]);

        assert_eq!(expected, Day08.parse_instructions(input.as_str()));
    }
}
