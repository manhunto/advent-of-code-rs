use std::collections::HashMap;
use regex::Regex;
use crate::chain_pattern_finder::Chain;
use crate::infinite_iterator::InfiniteIterator;
use crate::math::lcm;
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

        let currents: Vec<&str> = instructions
            .keys()
            .map(|c| *c)
            .filter(|c| c.ends_with('A'))
            .collect();

        let mut chains: Vec<Chain> = currents
            .iter()
            .map(|c| Chain::new(vec![c.to_string()]))
            .collect();

        let mut processed: HashMap<usize, usize> = HashMap::new();

        let mut watched: Vec<usize> = vec![];

        loop {
            let direction = navigation.next();

            for (i, current) in chains.iter_mut().enumerate() {
                let (left, right) = instructions.get(current.last().as_str()).unwrap();

                let new = match direction {
                    'R' => right,
                    'L' => left,
                    _ => panic!("WTF"),
                };

                if !processed.contains_key(&i) {
                    if new.ends_with('Z') && !watched.contains(&i){
                        current.push_and_start_watch(new.to_string());
                        watched.push(i);
                    } else {
                        let result = current.push(new.to_string());
                        if result.is_some() {
                            processed.insert(i, result.unwrap().1);
                        }
                    }
                }
            }

            if processed.len() == chains.len() {
                println!("{:?}", processed);

                let mut t:  Vec<u64> = vec![];
                for p in processed.values() {
                    t.push(*p as u64);
                }

                return lcm(t).to_string();
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
