use crate::solutions::Solution;
use crate::utils::point::Point;
use crate::utils::surface_range::SurfaceRange;
use itertools::Itertools;

pub struct Day09;

impl Solution for Day09 {
    fn part_one(&self, input: &str) -> String {
        self.parse(input)
            .into_iter()
            .tuple_combinations()
            .map(|(a, b)| SurfaceRange::from((a, b)).area())
            .max()
            .unwrap()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day09 {
    fn parse(&self, input: &str) -> Vec<Point> {
        input.lines().map(|line| line.parse().unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2025::day09::Day09;
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("50", Day09.part_one(EXAMPLE));
    }
}
