use crate::solutions::Solution;
use itertools::Itertools;
use std::ops::RangeInclusive;

pub struct Day05;

impl Solution for Day05 {
    fn part_one(&self, input: &str) -> String {
        let (ranges, ids) = self.parse(input);

        ids.iter()
            .filter(|id| ranges.iter().any(|range| range.contains(id)))
            .count()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day05 {
    fn parse(&self, input: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
        let (ranges_str, ids_str) = input.split_once("\n\n").unwrap();

        let ranges = ranges_str
            .lines()
            .map(|line| {
                let (start, end) = line.split_once("-").unwrap();

                let start = start.parse::<usize>().unwrap();
                let end = end.parse::<usize>().unwrap();

                start..=end
            })
            .collect_vec();

        let ids = ids_str
            .lines()
            .map(|line| line.parse::<usize>().unwrap())
            .collect_vec();

        (ranges, ids)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2025::day05::Day05;
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("3", Day05.part_one(EXAMPLE));
    }
}
