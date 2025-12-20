use crate::solutions::Solution;

pub struct Day11;

impl Solution for Day11 {
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

    #[test]
    fn part_one_example_test() {
        assert_eq!("0", Day11.part_one("0"));
    }
}
