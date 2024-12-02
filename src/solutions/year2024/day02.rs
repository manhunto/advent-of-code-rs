use crate::solutions::Solution;
use std::cmp::Ordering;

pub struct Day02;

impl Solution for Day02 {
    fn part_one(&self, input: &str) -> String {
        self.parse(input)
            .iter()
            .filter(|report| self.is_report_safe(report))
            .count()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        self.parse(input)
            .iter()
            .filter(|report| {
                self.is_report_safe(report)
                    || (0..report.len()).any(|i| {
                        let mut new: Vec<i32> = report.to_vec();
                        new.remove(i);

                        self.is_report_safe(&new)
                    })
            })
            .count()
            .to_string()
    }
}

impl Day02 {
    fn parse(&self, input: &str) -> Vec<Vec<i32>> {
        input
            .lines()
            .map(|line| {
                line.split_terminator(" ")
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect()
    }

    fn is_report_safe(&self, report: &[i32]) -> bool {
        let mut state: Option<Ordering> = None;

        for i in 0..report.len() - 1 {
            let (first, second) = (report[i], report[i + 1]);
            let current_state = first.cmp(&second);

            if (first - second).abs() > 3 || current_state == Ordering::Equal {
                return false;
            }

            state.get_or_insert(current_state);

            if state.is_some_and(|inner| inner != current_state) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day02::Day02;
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("2", Day02.part_one(EXAMPLE));
    }

    #[test]
    fn part_one_with_equals_test() {
        assert_eq!("0", Day02.part_one("2 2 2"));
        assert_eq!("0", Day02.part_one("2 2 1"));
        assert_eq!("0", Day02.part_one("2 2 3"));
        assert_eq!("0", Day02.part_one("1 2 2"));
        assert_eq!("0", Day02.part_one("2 2 2"));
        assert_eq!("0", Day02.part_one("3 2 2"));
    }

    #[test]
    fn part_two_example_test() {
        assert_eq!("4", Day02.part_two(EXAMPLE));
    }
}
