use crate::solutions::Solution;
use itertools::Itertools;
use std::str::FromStr;

pub struct Day06;

impl Solution for Day06 {
    fn part_one(&self, input: &str) -> String {
        let (numbers, operations) = self.parse(input);
        let column_count = numbers.first().unwrap().len();

        (0..column_count)
            .map(|column| {
                let operation = operations.get(column).unwrap();
                let numbers_in_column = numbers.iter().map(|n_vec| n_vec[column]);

                match operation {
                    Operation::Add => numbers_in_column.sum::<u64>(),
                    Operation::Multiply => numbers_in_column.product(),
                }
            })
            .sum::<u64>()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day06 {
    fn parse(&self, input: &str) -> (Vec<Vec<u64>>, Vec<Operation>) {
        let mut lines = input.lines().collect_vec();
        let operations_str = lines.pop().unwrap();

        let numbers = lines
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect_vec()
            })
            .collect_vec();

        let operations = operations_str
            .split_whitespace()
            .map(|x| x.parse::<Operation>().unwrap())
            .collect_vec();

        (numbers, operations)
    }
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
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
}
