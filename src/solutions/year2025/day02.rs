use crate::solutions::Solution;
use crate::utils::range::Range;
use itertools::Itertools;

pub struct Day02;

impl Solution for Day02 {
    fn part_one(&self, input: &str) -> String {
        let parsed = self.parse(input);

        parsed
            .iter()
            .flat_map(|range| self.find_invalid_ids(range))
            .sum::<isize>()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day02 {
    fn parse(&self, input: &str) -> Vec<Range> {
        input
            .lines()
            .flat_map(|line| {
                line.split(',').filter_map(|range_str| {
                    if let Some((left, right)) = range_str.split('-').collect_tuple() {
                        Some(
                            Range::new(
                                left.parse::<isize>().unwrap(),
                                right.parse::<isize>().unwrap(),
                            )
                            .unwrap(),
                        )
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    fn find_invalid_ids(&self, range: &Range) -> Vec<isize> {
        range
            .iter()
            .filter(|id| {
                let string = id.to_string();
                let len = string.len();
                let left = &string[..len / 2];
                let right = &string[len / 2..];

                left == right
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2025::day02::Day02;
    use crate::solutions::Solution;
    use crate::utils::range::Range;

    const EXAMPLE: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("1227775554", Day02.part_one(EXAMPLE));
    }

    #[test]
    fn parse_test() {
        const SIMPLIFIED_EXAMPLE: &str = r#"11-22,95-115,
1698522-1698528,446443-446449,
824824821-824824827"#;

        let result = Day02.parse(SIMPLIFIED_EXAMPLE);
        let expected = vec![
            Range::new(11, 22).unwrap(),
            Range::new(95, 115).unwrap(),
            Range::new(1698522, 1698528).unwrap(),
            Range::new(446443, 446449).unwrap(),
            Range::new(824824821, 824824827).unwrap(),
        ];

        assert_eq!(expected, result);
    }

    #[test]
    fn find_invalid_ids() {
        assert_invalid_ids(11, 22, vec![11, 22]);
        assert_invalid_ids(95, 115, vec![99]);
        assert_invalid_ids(998, 1012, vec![1010]);
        assert_invalid_ids(1188511880, 1188511890, vec![1188511885]);
        assert_invalid_ids(222220, 222224, vec![222222]);
        assert_invalid_ids(1698522, 1698528, vec![]);
        assert_invalid_ids(446443, 446449, vec![446446]);
        assert_invalid_ids(38593856, 38593862, vec![38593859]);
    }

    fn assert_invalid_ids(left: isize, right: isize, invalid_ids: Vec<isize>) {
        assert_eq!(
            invalid_ids,
            Day02.find_invalid_ids(&Range::new(left, right).unwrap())
        );
    }
}
