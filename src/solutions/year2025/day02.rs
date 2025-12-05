use crate::solutions::Solution;
use crate::utils::range::Range;
use fancy_regex::Regex;
use itertools::Itertools;

pub struct Day02;

impl Solution for Day02 {
    fn part_one(&self, input: &str) -> String {
        let regex = self.part_one_regex();

        self.solve(input, &regex).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let regex = self.part_two_regex();

        self.solve(input, &regex).to_string()
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

    fn part_one_regex(&self) -> Regex {
        Regex::new(r"^(\d+)\1$").unwrap()
    }

    fn part_two_regex(&self) -> Regex {
        Regex::new(r"^(\d+)\1+$").unwrap()
    }

    fn solve(&self, input: &str, regex: &Regex) -> String {
        let parsed = self.parse(input);

        parsed
            .iter()
            .flat_map(|range| self.find_invalid_ids(range, regex))
            .sum::<isize>()
            .to_string()
    }

    fn find_invalid_ids(&self, range: &Range, regex: &Regex) -> Vec<isize> {
        range
            .iter()
            .filter(|id| regex.is_match(&id.to_string()).unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2025::day02::Day02;
    use crate::solutions::Solution;
    use crate::utils::range::Range;
    use fancy_regex::Regex;

    const EXAMPLE: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("1227775554", Day02.part_one(EXAMPLE));
    }

    #[test]
    fn part_two_example_test() {
        assert_eq!("4174379265", Day02.part_two(EXAMPLE));
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
    fn find_invalid_ids_part_one() {
        let regex = Day02.part_one_regex();

        assert_invalid_ids(&regex, 11, 22, vec![11, 22]);
        assert_invalid_ids(&regex, 95, 115, vec![99]);
        assert_invalid_ids(&regex, 998, 1012, vec![1010]);
        assert_invalid_ids(&regex, 1188511880, 1188511890, vec![1188511885]);
        assert_invalid_ids(&regex, 222220, 222224, vec![222222]);
        assert_invalid_ids(&regex, 1698522, 1698528, vec![]);
        assert_invalid_ids(&regex, 446443, 446449, vec![446446]);
        assert_invalid_ids(&regex, 38593856, 38593862, vec![38593859]);
    }

    #[test]
    fn find_invalid_ids_part_two() {
        let regex = Day02.part_two_regex();

        assert_invalid_ids(&regex, 11, 22, vec![11, 22]);
        assert_invalid_ids(&regex, 95, 115, vec![99, 111]);
        assert_invalid_ids(&regex, 998, 1012, vec![999, 1010]);
        assert_invalid_ids(&regex, 1188511880, 1188511890, vec![1188511885]);
        assert_invalid_ids(&regex, 222220, 222224, vec![222222]);
        assert_invalid_ids(&regex, 1698522, 1698528, vec![]);
        assert_invalid_ids(&regex, 446443, 446449, vec![446446]);
        assert_invalid_ids(&regex, 38593856, 38593862, vec![38593859]);
        assert_invalid_ids(&regex, 565653, 565659, vec![565656]);
        assert_invalid_ids(&regex, 824824821, 824824827, vec![824824824]);
        assert_invalid_ids(&regex, 2121212118, 2121212124, vec![2121212121]);
    }

    fn assert_invalid_ids(regex: &Regex, left: isize, right: isize, invalid_ids: Vec<isize>) {
        assert_eq!(
            invalid_ids,
            Day02.find_invalid_ids(&Range::new(left, right).unwrap(), regex)
        );
    }
}
