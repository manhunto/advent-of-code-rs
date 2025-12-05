use crate::solutions::Solution;

pub struct Day02;

impl Solution for Day02 {
    fn part_one(&self, _input: &str) -> String {
        String::from("0")
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2025::day02::Day02;
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("0", Day02.part_one(EXAMPLE));
    }
}
