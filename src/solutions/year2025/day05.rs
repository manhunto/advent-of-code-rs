use crate::solutions::Solution;
use crate::utils::range::Range;
use itertools::Itertools;
use std::collections::VecDeque;
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

    fn part_two(&self, input: &str) -> String {
        let ranges = self.parse_ranges(input);

        self.unique_ids(ranges.into_iter()).to_string()
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

    fn parse_ranges(&self, input: &str) -> Vec<Range> {
        let (ranges_str, _) = input.split_once("\n\n").unwrap();

        ranges_str
            .lines()
            .map(|line| {
                let tuple = line.split_once("-").unwrap();

                tuple.try_into().unwrap()
            })
            .collect_vec()
    }

    fn unique_ids(&self, ranges: impl Iterator<Item = Range>) -> isize {
        let mut queue = VecDeque::from_iter(ranges);
        let mut result: Vec<Range> = Vec::new();

        while let Some(range) = queue.pop_front() {
            result.push(range);

            queue = queue.iter().flat_map(|other| other.diff(&range)).collect();
        }

        result.iter().map(|range| range.len()).sum::<isize>()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2025::day05::Day05;
    use crate::solutions::Solution;
    use crate::utils::range::Range;
    use std::ops::RangeInclusive;

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

    #[test]
    fn part_two_example_test() {
        assert_eq!("14", Day05.part_two(EXAMPLE));
    }

    #[test]
    fn unique_ids() {
        assert_eq!(8, run_unique_ids(vec![3..=5, 10..=14]));
        assert_eq!(5, run_unique_ids(vec![3..=5, 1..=4]));
        assert_eq!(6, run_unique_ids(vec![3..=7, 6..=8]));
        assert_eq!(7, run_unique_ids(vec![3..=7, 2..=8]));
    }

    fn run_unique_ids(vec: Vec<RangeInclusive<isize>>) -> isize {
        let ranges = vec.into_iter().map(Range::from);

        Day05.unique_ids(ranges)
    }
}
