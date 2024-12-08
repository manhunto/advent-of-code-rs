use crate::solutions::Solution;
use itertools::Itertools;
pub struct Day07;

impl Solution for Day07 {
    fn part_one(&self, input: &str) -> String {
        input
            .lines()
            .map(|l| {
                let (left, right) = l.split(": ").collect_tuple().unwrap();
                let test_value: usize = left.parse().unwrap();
                let mut numbers: Vec<usize> = right
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();

                let current = numbers.remove(0);
                let value = Self::solve(test_value, current, numbers.clone());

                if value {
                    return test_value;
                }

                0
            })
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from('0')
    }
}

impl Day07 {
    fn solve(expected: usize, current: usize, number_lefts: Vec<usize>) -> bool {
        let mut numbers = number_lefts.clone();

        if numbers.is_empty() {
            return expected == current;
        }

        let next = numbers.remove(0);

        let current_add = current + next;
        let current_multiply = current * next;

        Self::solve(expected, current_add, numbers.clone())
            || Self::solve(expected, current_multiply, numbers)
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
}
