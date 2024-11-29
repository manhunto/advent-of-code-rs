use crate::solutions::Solution;

pub struct Day09;

impl Solution for Day09 {
    fn part_one(&self, input: &str) -> String {
        let history = self.parse_input(input);

        history
            .iter()
            .map(|h| self.calculate_at_the_end(h))
            .sum::<i32>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let history = self.parse_input(input);

        history
            .iter()
            .map(|h| self.calculate_at_the_beginning(h))
            .sum::<i32>()
            .to_string()
    }
}

impl Day09 {
    fn parse_input(&self, input: &str) -> Vec<Vec<i32>> {
        input
            .lines()
            .map(|line| -> Vec<i32> {
                line.split_whitespace()
                    .filter_map(|part| part.parse::<i32>().ok())
                    .collect()
            })
            .collect()
    }

    fn calculate_at_the_end(&self, history: &[i32]) -> i32 {
        let differences = self.solve(history);

        let mut result: i32 = 0;

        for i in (0..differences.len() - 1).rev() {
            let last = differences[i].last().unwrap();

            result += last;
        }

        result
    }

    fn calculate_at_the_beginning(&self, history: &[i32]) -> i32 {
        let differences = self.solve(history);

        let mut result: i32 = 0;

        for i in (0..differences.len() - 1).rev() {
            let first = differences[i].first().unwrap();

            result = first - result;
        }

        result
    }

    fn solve(&self, history: &[i32]) -> Vec<Vec<i32>> {
        let mut differences: Vec<Vec<i32>> = vec![history.to_owned()];

        loop {
            let mut t: Vec<i32> = vec![];
            let last = differences.last().unwrap();

            for i in 1..last.len() {
                t.push(last[i] - last[i - 1]);
            }

            differences.push(t.clone());

            if t.iter().all(|n| n.eq(&0i32)) {
                return differences;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2023::read_2023_example;
    use crate::solutions::year2023::day09::Day09;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_2023_example("09");

        assert_eq!("114", Day09.part_one(input.as_str()));
    }

    #[test]
    fn part_two_example_test() {
        let input = read_2023_example("09");

        assert_eq!("2", Day09.part_two(input.as_str()));
    }

    #[test]
    fn calculate_at_the_end() {
        assert_eq!(18, Day09.calculate_at_the_end(&[0, 3, 6, 9, 12, 15]));
        assert_eq!(28, Day09.calculate_at_the_end(&[1, 3, 6, 10, 15, 21]));
        assert_eq!(68, Day09.calculate_at_the_end(&[10, 13, 16, 21, 30, 45]));
    }

    #[test]
    fn calculate_at_the_beginning() {
        assert_eq!(
            5,
            Day09.calculate_at_the_beginning(&[10, 13, 16, 21, 30, 45])
        )
    }

    #[test]
    fn parse_input() {
        let input = read_2023_example("09");

        let expected: Vec<Vec<i32>> = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45],
        ];

        assert_eq!(expected, Day09.parse_input(input.as_str()));
    }
}
