use crate::solutions::Solution;

pub struct Day02;

impl Solution for Day02 {
    fn part_one(&self, _input: &str) -> String {
        String::new()
    }

    fn part_two(&self, _input: &str) -> String {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day02::Day02;
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#""#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("", Day02.part_one(EXAMPLE));
    }
}
