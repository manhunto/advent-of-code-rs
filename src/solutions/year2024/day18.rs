use crate::solutions::Solution;

pub struct Day18;

impl Solution for Day18 {
    fn part_one(&self, _input: &str) -> String {
        String::from('0')
    }

    fn part_two(&self, _input: &str) -> String {
        String::from('0')
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day18::Day18;
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#""#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("0", Day18.part_one(EXAMPLE));
    }
}
