use crate::solutions::Solution;

pub struct Day22;

impl Solution for Day22 {
    fn part_one(&self, _input: &str) -> String {
        String::from("0")
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day22::Day22;
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#""#;

    #[test]
    fn part_one_example() {
        assert_eq!("0", Day22.part_one(EXAMPLE));
    }
}
