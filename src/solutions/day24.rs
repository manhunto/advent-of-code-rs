use crate::solutions::Solution;

pub struct Day24;

impl Solution for Day24 {
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
    use crate::solutions::day24::Day24;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("24");

        assert_eq!("2", Day24.part_one(input.as_str()));
    }
}
