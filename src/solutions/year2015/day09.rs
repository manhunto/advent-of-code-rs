use crate::solutions::Solution;
use itertools::Itertools;
use std::collections::HashMap;

type Distances = HashMap<(String, String), u64>;

pub struct Day09;

impl Solution for Day09 {
    fn part_one(&self, input: &str) -> String {
        let distances = self.parse(input);
        let cities = self.cities(&distances);

        println!("cities: {:?}", cities);
        println!("distances: {:?}", distances);

        cities
            .iter()
            .tuple_combinations()
            .map(|(_, _)| 0)
            .min()
            .unwrap()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day09 {
    fn parse(&self, input: &str) -> Distances {
        input
            .lines()
            .flat_map(|line| {
                let parts = line.split_whitespace().collect_vec();

                let from = parts[0];
                let to = parts[2];
                let distance = parts[4].parse::<u64>().unwrap();

                [
                    ((from.to_string(), to.to_string()), distance),
                    ((to.to_string(), from.to_string()), distance),
                ]
            })
            .collect()
    }

    fn cities(&self, distances: &Distances) -> Vec<String> {
        distances
            .keys()
            .flat_map(|(from, to)| [from.to_string(), to.to_string()])
            .unique()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("605", Day09.part_one(EXAMPLE));
    }
}
