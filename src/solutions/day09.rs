use crate::solutions::Solution;

pub struct Day09;

impl Solution for Day09 {
    fn part_one(&self, input: &str) -> String {
        String::from("0")
    }

    fn part_two(&self, input: &str) -> String {
        String::from("0")
    }
}

impl Day09 {
    fn parse_input(&self, input: &str) -> Vec<Vec<i32>> {
        input
            .lines()
            .map(|line| -> Vec<i32> {
                line
                    .split_whitespace()
                    .filter_map(|part| part.parse::<i32>().ok())
                    .collect()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day09::Day09;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("09");

        assert_eq!("114", Day09.part_one(&input.as_str()));
    }

    #[test]
    fn parse_input() {
        let input = read_example("09");

        let expected: Vec<Vec<i32>> = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45]
        ];

        assert_eq!(expected, Day09.parse_input(&input.as_str()));
    }
}