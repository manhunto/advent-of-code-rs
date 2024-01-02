use crate::solutions::Solution;

pub struct Day19;

impl Solution for Day19 {
    fn part_one(&self, input: &str) -> String {
        String::from('0')
    }

    fn part_two(&self, input: &str) -> String {
        String::from('0')
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day19::Day19;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("19");

        assert_eq!("19114", Day19.part_one(&input.as_str()));
    }
}
