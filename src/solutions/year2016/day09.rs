use crate::solutions::Solution;

pub struct Day09;

impl Solution for Day09 {
    fn part_one(&self, _input: &str) -> String {
        String::from("0")
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#""#;

    #[test]
    fn part_one_example() {
        assert_eq!("0", Day09.part_one(EXAMPLE));
    }
}
