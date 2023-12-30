use std::ops::Mul;
use crate::solutions::Solution;

pub struct Day15;

impl Solution for Day15 {
    fn part_one(&self, input: &str) -> String {
        let steps: Vec<&str> = input.split_terminator(',').collect();

        steps
            .into_iter()
            .map(Day15::hash)
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        String::from("0")
    }
}

impl Day15 {
    fn hash(step: &str) -> usize {
        step
            .chars()
            .fold(0, |current, char| (current + char as usize).mul(17) % 256)
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

    #[test]
    fn hash_test() {
        assert_eq!(52, Day15::hash("HASH"));
        assert_eq!(30, Day15::hash("rn=1"));
        assert_eq!(253, Day15::hash("cm-"));
        assert_eq!(97, Day15::hash("qp=3"));
        assert_eq!(14, Day15::hash("qp-"));
        assert_eq!(180, Day15::hash("pc=4"));
        assert_eq!(9, Day15::hash("ot=9"));
        assert_eq!(197, Day15::hash("ab=5"));
        assert_eq!(48, Day15::hash("pc-"));
        assert_eq!(214, Day15::hash("pc=6"));
        assert_eq!(231, Day15::hash("ot=7"));
    }
}
