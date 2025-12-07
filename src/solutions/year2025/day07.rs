use crate::solutions::Solution;

pub struct Day07;

impl Solution for Day07 {
    fn part_one(&self, _input: &str) -> String {
        String::from("0")
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2025::day07::Day07;
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#""#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("0", Day07.part_one(EXAMPLE));
    }
}
