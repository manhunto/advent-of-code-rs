use crate::solutions::Solution;
use crate::utils::combinatorics::subset_sum::find_subset_sum;

pub struct Day17 {
    liters_of_eggnog: u32,
}

impl Solution for Day17 {
    fn part_one(&self, input: &str) -> String {
        let containers = self.parse(input);

        find_subset_sum(&containers, self.liters_of_eggnog)
            .len()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day17 {
    fn parse(&self, input: &str) -> Vec<u32> {
        input.lines().map(|line| line.parse().unwrap()).collect()
    }
}

impl Default for Day17 {
    fn default() -> Self {
        Self {
            liters_of_eggnog: 150,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"20
15
10
5
5"#;

    #[test]
    fn part_one_example() {
        assert_eq!("4", day().part_one(EXAMPLE));
    }

    fn day() -> Day17 {
        Day17 {
            liters_of_eggnog: 25,
        }
    }
}
