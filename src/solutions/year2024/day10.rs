use crate::solutions::Solution;

pub struct Day10;

impl Solution for Day10 {
    fn part_one(&self, _input: &str) -> String {
        String::from('0')
    }

    fn part_two(&self, _input: &str) -> String {
        String::from('0')
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day10::Day10;
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#""#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("0", Day10.part_one(EXAMPLE));
    }
}
