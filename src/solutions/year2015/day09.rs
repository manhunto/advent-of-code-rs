use crate::solutions::Solution;
use crate::utils::graphs::travelling_salesman::TravellingSalesman;
use itertools::Itertools;

pub struct Day09;

impl Solution for Day09 {
    fn part_one(&self, input: &str) -> String {
        self.parse(input)
            .find_shortest_path_cost()
            .unwrap()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        self.parse(input)
            .find_longest_path_cost()
            .unwrap()
            .to_string()
    }
}

impl Day09 {
    fn parse<'a>(&self, input: &'a str) -> TravellingSalesman<&'a str> {
        let mut ts = TravellingSalesman::default();

        input.lines().for_each(|line| {
            let parts = line.split_whitespace().collect_vec();

            let from = parts[0];
            let to = parts[2];
            let distance = parts[4].parse::<usize>().unwrap();

            ts.add(from, to, distance);
        });

        ts
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

    #[test]
    fn part_two_example_test() {
        assert_eq!("982", Day09.part_two(EXAMPLE));
    }
}
