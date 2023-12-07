use crate::solutions::Solution;
use std::str;

pub struct Day05;

impl Solution for Day05 {
    fn part_one(&self, input: &str) -> String {
        String::from("0")
    }

    fn part_two(&self, input: &str) -> String {
        String::from("0")
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day05::Day05;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("05");

        assert_eq!("13", Day05.part_one(&input.as_str()));
    }
}
