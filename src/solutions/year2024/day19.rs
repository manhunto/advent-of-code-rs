use crate::solutions::Solution;
use itertools::Itertools;
use std::str;

pub struct Day19;

impl Solution for Day19 {
    fn part_one(&self, input: &str) -> String {
        let (patterns, designs) = self.parse(input);

        designs
            .iter()
            .filter(|design| Self::matches_any(&patterns, design))
            .count()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let (patterns, designs) = self.parse(input);

        designs
            .iter()
            .map(|design| Self::count_matches(&patterns, design, 0))
            .sum::<usize>()
            .to_string()
    }
}

impl Day19 {
    fn parse<'a>(&self, input: &'a str) -> (Vec<&'a [u8]>, Vec<&'a [u8]>) {
        input
            .split_once("\n\n")
            .map(|(patterns, designs)| {
                let patterns = patterns
                    .split_terminator(", ")
                    .map(|p| p.as_bytes())
                    .collect_vec();

                let designs = designs.lines().map(|line| line.as_bytes()).collect_vec();

                (patterns, designs)
            })
            .unwrap()
    }

    fn matches_any(patterns: &Vec<&[u8]>, design: &[u8]) -> bool {
        if design.is_empty() {
            return true;
        }

        patterns.iter().any(|&pattern| {
            if design.len() < pattern.len() {
                return false;
            }

            if &design[..pattern.len()] == pattern {
                let new = &design[pattern.len()..];

                return Self::matches_any(patterns, new);
            }

            false
        })
    }

    fn count_matches(patterns: &Vec<&[u8]>, design: &[u8], current: usize) -> usize {
        if design.is_empty() {
            return current + 1;
        }

        patterns
            .iter()
            .map(|&pattern| {
                if design.len() < pattern.len() {
                    return current;
                }

                if &design[..pattern.len()] == pattern {
                    let new = &design[pattern.len()..];

                    return Self::count_matches(patterns, new, current);
                }

                current
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day19::Day19;
    use crate::solutions::Solution;

    const PATTERNS: [&str; 8] = ["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];
    const EXAMPLE: &str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;

    #[test]
    fn part_one_example() {
        assert_eq!("6", Day19.part_one(EXAMPLE));
    }

    #[test]
    fn part_two_example() {
        assert_eq!("16", Day19.part_two(EXAMPLE));
    }

    #[test]
    fn matches_any() {
        let patterns = PATTERNS.map(|pat| pat.as_bytes()).to_vec();

        assert!(Day19::matches_any(&patterns, "brwrr".as_bytes()));
        assert!(Day19::matches_any(&patterns, "bggr".as_bytes()));
        assert!(Day19::matches_any(&patterns, "gbbr".as_bytes()));
        assert!(Day19::matches_any(&patterns, "rrbgbr".as_bytes()));
        assert!(!Day19::matches_any(&patterns, "ubwu".as_bytes()));
        assert!(Day19::matches_any(&patterns, "bwurrg".as_bytes()));
        assert!(Day19::matches_any(&patterns, "brgr".as_bytes()));
        assert!(!Day19::matches_any(&patterns, "bbrgwb".as_bytes()));
    }

    #[test]
    fn count_matches() {
        let patterns = PATTERNS.map(|pat| pat.as_bytes()).to_vec();

        assert_eq!(2, Day19::count_matches(&patterns, "brwrr".as_bytes(), 0));
        assert_eq!(1, Day19::count_matches(&patterns, "bggr".as_bytes(), 0));
        assert_eq!(4, Day19::count_matches(&patterns, "gbbr".as_bytes(), 0));
        assert_eq!(6, Day19::count_matches(&patterns, "rrbgbr".as_bytes(), 0));
        assert_eq!(1, Day19::count_matches(&patterns, "bwurrg".as_bytes(), 0));
        assert_eq!(2, Day19::count_matches(&patterns, "brgr".as_bytes(), 0));
    }
}
