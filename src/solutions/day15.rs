use crate::solutions::Solution;

pub struct Day15;

impl Solution for Day15 {
    fn part_one(&self, input: &str) -> String {
        let steps: Vec<&str> = input.split_terminator(',').collect();

        println!("{:?}", steps);

        String::from("0")
    }

    fn part_two(&self, input: &str) -> String {
        String::from("0")
    }
}


#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day15::Day15;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("15");

        assert_eq!("1320", Day15.part_one(&input.as_str()));
    }
}
