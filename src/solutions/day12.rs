use std::fmt::{Display, Formatter};
use itertools::{Itertools, repeat_n};
use crate::solutions::Solution;

pub struct Day12;

impl Solution for Day12 {
    fn part_one(&self, input: &str) -> String {
        let records = Self::parse_input_part_one(input);

        records
            .iter()
            .map(|c| c.possible_arrangements())
            .sum::<i32>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let records = Self::parse_input_part_two(input);

        for record in records {
            println!("{}", record);
        }

        String::from("0")
    }
}

impl Day12 {
    fn parse_input_part_one(input: &str) -> Vec<ConditionRecord> {
        input
            .lines()
            .map(ConditionRecord::from)
            .collect()
    }

    fn parse_input_part_two(input: &str) -> Vec<ConditionRecord> {
        input
            .lines()
            .map(Self::unfold)
            .map(ConditionRecord::from)
            .collect()
    }

    fn unfold(input: &str) -> String {
        let mut groups = input.split_whitespace();

        let pattern= groups.next().unwrap();
        let pattern = (0..5).map(|_| pattern).join("?");

        let order = groups.next().unwrap();
        let order = (0..5).map(|_| order).join(",");

        format!("{} {}", pattern, order)
    }
}

#[derive(Debug)]
struct ConditionRecord {
    pattern: Vec<Spring>,
    order: Vec<i32>,
}

impl From<&str> for ConditionRecord {
    fn from(value: &str) -> Self {
        let mut groups = value.split_whitespace();

        let pattern: Vec<Spring> = groups.next().unwrap().chars().map(Spring::from).collect();
        let order: Vec<i32> = groups.next().unwrap().split(",").filter_map(|c| c.to_string().parse().ok()).collect();

        ConditionRecord {
            pattern,
            order,
        }
    }
}

impl From<String> for ConditionRecord {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl ConditionRecord {
    fn is_valid(&self) -> bool {
        let pattern_to_order: Vec<i32> = self.pattern
            .split(|s| s != &Spring::Damaged)
            .filter_map(|g| if g.is_empty() { None } else { Some(g.len() as i32) })
            .collect();

        pattern_to_order == self.order
    }

    fn possible_arrangements(&self) -> i32 {
        let unknown = self.pattern.clone().into_iter().filter(|s| s == &Spring::Unknown).collect_vec().len();

        let permutations: Vec<Vec<char>> = repeat_n(['.', '#'], unknown)
            .multi_cartesian_product()
            .into_iter()
            .collect();

        permutations
            .iter()
            .map(|per| {
                let mut iter = per.iter();

                let new: Vec<Spring> = self.pattern.clone()
                    .iter()
                    .map(|s| match s {
                        Spring::Unknown => Spring::from(*iter.next().unwrap()),
                        value => value.clone()
                    })
                    .collect();

                self.with_pattern(new)
            })
            .filter(|c| c.is_valid())
            .collect::<Vec<Self>>()
            .len() as i32
    }

    fn with_pattern(&self, pattern: Vec<Spring>) -> Self {
        Self {
            pattern,
            order: self.order.clone(),
        }
    }
}

impl Display for ConditionRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pattern = self.pattern.iter().map(|s| s.to_string()).join("");
        let order = self.order.iter().join(",");

        write!(f, "{} {}", pattern, order)
    }
}

#[derive(PartialEq, Debug, Eq, Clone, Hash, Copy)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Damaged,
            '.' => Self::Operational,
            '?' => Self::Unknown,
            _ => panic!("Could not resolve spring")
        }
    }
}

impl Display for Spring {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Spring::Operational => '.',
            Spring::Damaged => '#',
            Spring::Unknown => '?',
        };
        write!(f, "{}", c)
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day12::{ConditionRecord, Day12};
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("12");

        assert_eq!("21", Day12.part_one(&input.as_str()));
    }

    #[test]
    fn part_two_example_test() {
        let input = read_example("12");

        assert_eq!("525152", Day12.part_two(&input.as_str()));
    }

    #[test]
    fn unfold_test() {
        assert_eq!(".#?.#?.#?.#?.# 1,1,1,1,1", Day12::unfold(".# 1"));
        assert_eq!("???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3", Day12::unfold("???.### 1,1,3"));
    }

    #[test]
    fn condition_record_is_valid_test() {
        assert!(ConditionRecord::from("#.#.### 1,1,3").is_valid());
        assert!(ConditionRecord::from(".#...#....###. 1,1,3").is_valid());
        assert!(ConditionRecord::from(".#.###.#.###### 1,3,1,6").is_valid());
        assert!(ConditionRecord::from("####.#...#... 4,1,1").is_valid());
        assert!(ConditionRecord::from("#....######..#####. 1,6,5").is_valid());
        assert!(ConditionRecord::from(".###.##....# 3,2,1").is_valid());
        assert!(ConditionRecord::from("## 2").is_valid());
        assert!(ConditionRecord::from("#. 1").is_valid());
        assert!(ConditionRecord::from(".# 1").is_valid());

        assert!(!ConditionRecord::from(".# 2").is_valid());
        assert!(!ConditionRecord::from("?.?.### 1,1,3").is_valid());
    }

    #[test]
    fn condition_record_possible_arrangements_test() {
        assert_eq!(1, ConditionRecord::from("???.### 1,1,3").possible_arrangements());
        assert_eq!(4, ConditionRecord::from(".??..??...?##. 1,1,3").possible_arrangements());
        assert_eq!(1, ConditionRecord::from("?#?#?#?#?#?#?#? 1,3,1,6").possible_arrangements());
        assert_eq!(1, ConditionRecord::from("????.#...#... 4,1,1").possible_arrangements());
        assert_eq!(4, ConditionRecord::from("????.######..#####. 1,6,5").possible_arrangements());
        assert_eq!(10, ConditionRecord::from("?###???????? 3,2,1").possible_arrangements());
    }
}
