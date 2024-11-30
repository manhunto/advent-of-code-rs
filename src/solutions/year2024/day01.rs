use crate::solutions::Solution;

pub struct Day01;

impl Solution for Day01 {
    fn part_one(&self, _input: &str) -> String {
        String::new()
    }

    fn part_two(&self, _input: &str) -> String {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day01::Day01;
    use crate::solutions::Solution;

    const EXAMPLE: &str = "";

    #[test]
    fn part_one_example_test() {
        assert_eq!("", Day01.part_one(EXAMPLE));
    }
}
