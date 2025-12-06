use crate::solutions::Solution;
use itertools::Itertools;
use std::str::FromStr;

pub struct Day06;

impl Solution for Day06 {
    fn part_one(&self, input: &str) -> String {
        let (numbers, operations) = self.parse_part_one(input);

        operations
            .iter()
            .enumerate()
            .map(|(column, operation)| {
                let numbers_in_column = numbers.iter().map(|n_vec| n_vec[column]);

                operation.calculate(numbers_in_column)
            })
            .sum::<u64>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let (numbers, operations) = self.parse_part_two(input);

        operations
            .iter()
            .rev()
            .enumerate()
            .map(|(column, operation)| {
                let numbers_in_column = numbers.get(column).unwrap().iter().copied();

                operation.calculate(numbers_in_column)
            })
            .sum::<u64>()
            .to_string()
    }
}

impl Day06 {
    fn parse_part_one(&self, input: &str) -> (Vec<Vec<u64>>, Vec<Operation>) {
        let mut lines = input.lines().collect_vec();
        let operations_str = lines.pop().unwrap();
        let operations = self.parse_operations(operations_str);

        let numbers = lines
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect_vec()
            })
            .collect_vec();

        (numbers, operations)
    }

    fn parse_part_two(&self, input: &str) -> (Vec<Vec<u64>>, Vec<Operation>) {
        let mut lines = input.lines().collect_vec();
        let operations_str = lines.pop().unwrap();
        let operations = self.parse_operations(operations_str);

        let column_width = operations_str.len();

        let column_results: Vec<_> = (0..column_width)
            .rev()
            .map(|column| {
                lines
                    .iter()
                    .filter_map(|line| line.chars().nth(column))
                    .filter(|ch| !ch.is_whitespace())
                    .join("")
                    .parse::<u64>()
            })
            .collect();

        let groups = column_results
            .split(|r| r.is_err())
            .filter(|slice| !slice.is_empty())
            .map(|slice| slice.iter().map(|r| *r.as_ref().unwrap()).collect())
            .collect();

        (groups, operations)
    }

    fn parse_operations(&self, input: &str) -> Vec<Operation> {
        input
            .split_whitespace()
            .map(|x| x.parse::<Operation>().unwrap())
            .collect_vec()
    }
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn calculate(&self, numbers: impl Iterator<Item = u64>) -> u64 {
        match self {
            Operation::Add => numbers.sum(),
            Operation::Multiply => numbers.product(),
        }
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Multiply),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2025::day06::Day06;
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  "#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("4277556", Day06.part_one(EXAMPLE));
    }

    #[test]
    fn part_two_example_test() {
        assert_eq!("3263827", Day06.part_two(EXAMPLE));
    }
}
