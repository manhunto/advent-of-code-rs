use crate::solutions::Solution;

pub struct Day19;

impl Solution for Day19 {
    fn part_one(&self, _input: &str) -> String {
        String::from('0')
    }

    fn part_two(&self, _input: &str) -> String {
        String::from('0')
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day19::Day19;
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#""#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("0", Day19.part_one(EXAMPLE));
    }
}
