use crate::solutions::Solution;

pub struct Day25;

impl Solution for Day25 {
    fn part_one(&self, _input: &str) -> String {
        String::from("0")
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day25::Day25;
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#""#;

    #[test]
    fn part_one_example() {
        assert_eq!("0", Day25.part_one(EXAMPLE));
    }
}
