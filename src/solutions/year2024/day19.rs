use crate::solutions::Solution;
use itertools::Itertools;
use std::collections::HashMap;
use std::str;

type Memo<'a> = HashMap<&'a [u8], usize>;

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
        let mut memo: Memo = Memo::new();

        designs
            .iter()
            .map(|design| Self::count_matches(&patterns, design, 0, &mut memo))
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

    fn count_matches<'a>(
        patterns: &Vec<&[u8]>,
        design: &'a [u8],
        current: usize,
        memo: &mut Memo<'a>,
    ) -> usize {
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

                    let result = if let Some(count) = memo.get(&new) {
                        *count
                    } else {
                        let count = Self::count_matches(patterns, new, current, memo);
                        memo.insert(new, count);

                        count
                    };

                    return result;
                }

                current
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day19::{Day19, Memo};
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

        assert_eq!(2, count_matches_init(&patterns, "brwrr"));
        assert_eq!(1, count_matches_init(&patterns, "bggr"));
        assert_eq!(4, count_matches_init(&patterns, "gbbr"));
        assert_eq!(6, count_matches_init(&patterns, "rrbgbr"));
        assert_eq!(1, count_matches_init(&patterns, "bwurrg"));
        assert_eq!(2, count_matches_init(&patterns, "brgr"));
    }

    fn count_matches_init(patterns: &Vec<&[u8]>, design: &str) -> usize {
        Day19::count_matches(patterns, design.as_bytes(), 0, &mut Memo::new())
    }
}
