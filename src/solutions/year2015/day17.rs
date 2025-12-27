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

    fn part_two(&self, input: &str) -> String {
        let containers = self.parse(input);
        let subsets = find_subset_sum(&containers, self.liters_of_eggnog);

        let min_len = subsets.iter().map(|s| s.len()).min().unwrap();

        subsets
            .iter()
            .filter(|s| s.len() == min_len)
            .count()
            .to_string()
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

    #[test]
    fn part_two_example() {
        assert_eq!("3", day().part_two(EXAMPLE));
    }

    fn day() -> Day17 {
        Day17 {
            liters_of_eggnog: 25,
        }
    }
}
