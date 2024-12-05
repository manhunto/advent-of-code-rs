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
                        let report_without_number_on_index: Vec<i32> =
                            self.remove_on_index(report, i);

                        self.is_report_safe(&report_without_number_on_index)
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
                line.split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect()
    }

    fn is_report_safe(&self, report: &[i32]) -> bool {
        let mut report_ordering: Option<Ordering> = None;

        for i in 0..report.len() - 1 {
            let (current_level, next_level) = (report[i], report[i + 1]);
            let current_ordering = current_level.cmp(&next_level);

            if (current_level - next_level).abs() > 3 || current_ordering == Ordering::Equal {
                return false;
            }

            report_ordering.get_or_insert(current_ordering);

            if report_ordering.is_some_and(|inner| inner != current_ordering) {
                return false;
            }
        }

        true
    }

    fn remove_on_index(&self, report: &[i32], index: usize) -> Vec<i32> {
        let mut report_without_number_on_index: Vec<i32> = report.to_vec();
        report_without_number_on_index.remove(index);

        report_without_number_on_index
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
