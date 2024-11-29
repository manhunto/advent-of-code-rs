use crate::solutions::Solution;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub struct Day12;

impl Solution for Day12 {
    fn part_one(&self, input: &str) -> String {
        let records = Self::parse_input_part_one(input);

        Self::solve(records)
    }

    fn part_two(&self, input: &str) -> String {
        let records = Self::parse_input_part_two(input);

        Self::solve(records)
    }
}

impl Day12 {
    fn parse_input_part_one(input: &str) -> Vec<ConditionRecord> {
        input.lines().map(ConditionRecord::from).collect()
    }

    fn parse_input_part_two(input: &str) -> Vec<ConditionRecord> {
        input
            .lines()
            .map(Self::unfold)
            .map(ConditionRecord::from)
            .collect()
    }

    fn unfold(input: &str) -> String {
        let mut parts = input.split_whitespace();

        let springs = parts.next().unwrap();
        let springs = (0..5).map(|_| springs).join("?");

        let groups = parts.next().unwrap();
        let groups = (0..5).map(|_| groups).join(",");

        format!("{} {}", springs, groups)
    }

    fn solve(records: Vec<ConditionRecord>) -> String {
        let mut cache: HashMap<ConditionRecord, usize> = HashMap::new();

        records
            .iter()
            .map(|c| c.possible_arrangements(&mut cache))
            .sum::<usize>()
            .to_string()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct ConditionRecord {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

impl From<&str> for ConditionRecord {
    fn from(value: &str) -> Self {
        let mut parts = value.split_whitespace();

        let springs: Vec<Spring> = parts.next().unwrap().chars().map(Spring::from).collect();
        let groups: Vec<usize> = parts
            .next()
            .unwrap()
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect();

        ConditionRecord::new(springs, groups)
    }
}

impl From<String> for ConditionRecord {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl ConditionRecord {
    fn new(springs: Vec<Spring>, groups: Vec<usize>) -> Self {
        Self { springs, groups }
    }

    fn possible_arrangements(&self, cache: &mut HashMap<ConditionRecord, usize>) -> usize {
        if let Some(&solutions) = cache.get(self) {
            return solutions;
        }

        if self.groups.is_empty() {
            let v = match self.springs.iter().any(|c| *c == Spring::Damaged) {
                true => 0,
                false => 1,
            };

            cache.insert(self.clone(), v);

            return v;
        }

        let needed_space = self.groups.iter().sum::<usize>() + self.groups.len() - 1;
        if self.springs.len() < needed_space {
            cache.insert(self.clone(), 0);

            return 0;
        }

        let first = self.springs[0];
        if first == Spring::Operational {
            let result = Self::new(self.springs[1..].to_vec(), self.groups.clone())
                .possible_arrangements(cache);
            cache.insert(self.clone(), result);

            return result;
        }

        let group = self.groups[0];
        let are_all_non_operational = self.springs[..group]
            .iter()
            .all(|c| *c != Spring::Operational);
        let end = (group + 1).min(self.springs.len());

        let mut solutions: usize = 0;

        if are_all_non_operational
            && ((self.springs.len() > group && self.springs[group] != Spring::Damaged)
                || self.springs.len() <= group)
        {
            solutions += Self::new(self.springs[end..].to_vec(), self.groups[1..].to_vec())
                .possible_arrangements(cache);
        }

        if first == Spring::Unknown {
            solutions += Self::new(self.springs[1..].to_vec(), self.groups.clone())
                .possible_arrangements(cache);
        }

        cache.insert(self.clone(), solutions);

        solutions
    }
}

impl Display for ConditionRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pattern = self.springs.iter().map(|s| s.to_string()).join("");
        let order = self.groups.iter().join(",");

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
            _ => panic!("Could not resolve spring"),
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
    use crate::file_system::read_2023_example;
    use crate::solutions::year2023::day12::{ConditionRecord, Day12, Spring};
    use crate::solutions::Solution;
    use std::collections::HashMap;

    #[test]
    fn part_one_example_test() {
        let input = read_2023_example("12");

        assert_eq!("21", Day12.part_one(input.as_str()));
    }

    #[test]
    fn part_two_example_test() {
        let input = read_2023_example("12");

        assert_eq!("525152", Day12.part_two(input.as_str()));
    }

    #[test]
    fn unfold_test() {
        assert_eq!(".#?.#?.#?.#?.# 1,1,1,1,1", Day12::unfold(".# 1"));
        assert_eq!(
            "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3",
            Day12::unfold("???.### 1,1,3")
        );
    }

    #[test]
    fn condition_record_possible_arrangements_test() {
        let mut cache: HashMap<ConditionRecord, usize> = HashMap::new();

        let empty_group = ConditionRecord::new(vec![Spring::Damaged], vec![]);
        assert_eq!(0, empty_group.possible_arrangements(&mut cache));

        let empty_group = ConditionRecord::new(vec![], vec![]);
        assert_eq!(1, empty_group.possible_arrangements(&mut cache));

        assert_eq!(
            0,
            ConditionRecord::from("## 3").possible_arrangements(&mut cache)
        );
        assert_eq!(
            4,
            ConditionRecord::from(".??..??...?##. 1,1,3").possible_arrangements(&mut cache)
        );
        assert_eq!(
            1,
            ConditionRecord::from("?#?#?#?#?#?#?#? 1,3,1,6").possible_arrangements(&mut cache)
        );
        assert_eq!(
            1,
            ConditionRecord::from("????.#...#... 4,1,1").possible_arrangements(&mut cache)
        );
        assert_eq!(
            4,
            ConditionRecord::from("????.######..#####. 1,6,5").possible_arrangements(&mut cache)
        );
        assert_eq!(
            10,
            ConditionRecord::from("?###???????? 3,2,1").possible_arrangements(&mut cache)
        );
    }
}
