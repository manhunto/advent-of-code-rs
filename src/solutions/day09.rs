use crate::solutions::Solution;

pub struct Day09;

impl Solution for Day09 {
    fn part_one(&self, input: &str) -> String {
        let history = self.parse_input(input);

        history
            .iter()
            .map(|h| self.solve(h.clone()))
            .sum::<i32>()
            .to_string()
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

    fn solve(&self, history: Vec<i32>) -> i32 {
        let mut last: Vec<i32> = history.clone();
        let mut differences: Vec<Vec<i32>> = vec![last.clone()];

        loop {
            let mut t: Vec<i32> = vec![];
            for i in 1..last.len() {
                t.push(last[i] - last[i - 1]);
            }

            differences.push(t.clone());
            last = t.clone();

            let not_zeros: Vec<i32> = last.clone()
                .into_iter()
                .filter(|n| !n.eq(&0i32))
                .collect();

            if not_zeros.is_empty() {
                break;
            }
        }

        let mut tmp: i32 = 0;

        for i in (0..differences.len() - 1).rev() {
            let a = differences[i].last().unwrap();

            tmp += a;
        }

        tmp
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
    fn solve() {
        assert_eq!(18, Day09.solve(vec![0, 3, 6, 9, 12, 15]));
        assert_eq!(28, Day09.solve(vec![1, 3, 6, 10, 15, 21]));
        assert_eq!(68, Day09.solve(vec![10, 13, 16, 21, 30, 45]));
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