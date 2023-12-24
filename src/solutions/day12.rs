use itertools::Itertools;
use crate::solutions::Solution;

pub struct Day12;

impl Solution for Day12 {
    fn part_one(&self, input: &str) -> String {
        let records = Self::parse_input(input);

        records
            .iter()
            .map(|c| c.valid_permutations().len() as i32)
            .sum::<i32>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        String::from("0")
    }
}

impl Day12 {
    fn parse_input(input: &str) -> Vec<ConditionRecord> {
        input
            .lines()
            .map(ConditionRecord::from)
            .collect()
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
        let order: Vec<i32> = groups.next().unwrap().chars().filter_map(|c| c.to_string().parse().ok()).collect();

        ConditionRecord {
            pattern,
            order,
        }
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

    fn valid_permutations(&self) -> Vec<Self> {
        let unknown: Vec<(i32, &Spring)> = self.pattern
            .iter()
            .enumerate()
            .filter_map(|(i, s)| {
                if s == &Spring::Unknown {
                    return Some((i as i32, s));
                }

                return None;
            })
            .collect();

        let possible_chars: Vec<char> = vec!['.', '#'];

        let permutations: Vec<Vec<char>> = possible_chars
            .into_iter()
            .combinations_with_replacement(unknown.len())
            .flat_map(|m| {
                m.into_iter().permutations(unknown.len())
            })
            .unique()
            .collect();

        permutations
            .iter()
            .map(|per| {
                let mut c = self.pattern.clone();
                let mut iter = per.iter();

                for (i, _) in &unknown {
                    c[*i as usize] = Spring::from(*iter.next().unwrap());
                }

                self.with_pattern(c)
            })
            .filter(|c| c.is_valid())
            .collect()
    }
    fn with_pattern(&self, pattern: Vec<Spring>) -> Self {
        Self {
            pattern,
            order: self.order.clone()
        }
    }
}

#[derive(PartialEq, Debug, Eq, Clone, Hash)]
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
    fn condition_record_permutation_test() {
        assert_eq!(1, ConditionRecord::from("???.### 1,1,3").valid_permutations().len());
        assert_eq!(4, ConditionRecord::from(".??..??...?##. 1,1,3").valid_permutations().len());
        assert_eq!(1, ConditionRecord::from("?#?#?#?#?#?#?#? 1,3,1,6").valid_permutations().len());
        assert_eq!(1, ConditionRecord::from("????.#...#... 4,1,1").valid_permutations().len());
        assert_eq!(4, ConditionRecord::from("????.######..#####. 1,6,5").valid_permutations().len());
        assert_eq!(10, ConditionRecord::from("?###???????? 3,2,1").valid_permutations().len());
    }
}
