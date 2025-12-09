use crate::solutions::Solution;

pub struct Day08;

impl Solution for Day08 {
    fn part_one(&self, _input: &str) -> String {
        String::from("0")
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2025::day08::Day08;
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#""#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("0", Day08.part_one(EXAMPLE));
    }

    #[test]
    fn part_two_example_test() {
        assert_eq!("0", Day08.part_two(EXAMPLE));
    }
}
