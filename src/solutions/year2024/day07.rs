use crate::solutions::Solution;
use itertools::Itertools;

pub struct Day07;

impl Solution for Day07 {
    fn part_one(&self, input: &str) -> String {
        self.solve_generic(input, Self::solve_part_one)
    }

    fn part_two(&self, input: &str) -> String {
        self.solve_generic(input, Self::solve_part_two)
    }
}

impl Day07 {
    fn solve_generic(&self, input: &str, solve_fn: fn(usize, usize, &[usize]) -> bool) -> String {
        input
            .lines()
            .filter_map(|l| {
                let (left, right) = l.split(": ").collect_tuple().unwrap();
                let test_value: usize = left.parse().unwrap();
                let numbers: Vec<usize> = right
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();

                let current = numbers[0];
                let remaining = &numbers[1..];

                if solve_fn(test_value, current, remaining) {
                    Some(test_value)
                } else {
                    None
                }
            })
            .sum::<usize>()
            .to_string()
    }

    fn solve_part_one(expected: usize, current: usize, numbers: &[usize]) -> bool {
        if numbers.is_empty() {
            return expected == current;
        }

        if expected < current {
            return false;
        }

        let next = numbers[0];
        let remaining = &numbers[1..];

        Self::solve_part_one(expected, current + next, remaining)
            || Self::solve_part_one(expected, current * next, remaining)
    }

    fn solve_part_two(expected: usize, current: usize, numbers: &[usize]) -> bool {
        if numbers.is_empty() {
            return expected == current;
        }

        if expected < current {
            return false;
        }

        let next = numbers[0];
        let remaining = &numbers[1..];

        Self::solve_part_two(expected, current + next, remaining)
            || Self::solve_part_two(expected, current * next, remaining)
            || Self::solve_part_two(
                expected,
                format!("{}{}", current, next).parse().unwrap(),
                remaining,
            )
    }
}
#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day07::Day07;
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("3749", Day07.part_one(EXAMPLE));
    }

    #[test]
    fn part_two_example_test() {
        assert_eq!("11387", Day07.part_two(EXAMPLE));
    }
}
