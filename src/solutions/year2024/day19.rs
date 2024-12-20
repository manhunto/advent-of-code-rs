use crate::solutions::Solution;
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
            .map(|design| Self::count_matches(&patterns, design, &mut memo))
            .sum::<usize>()
            .to_string()
    }
}

impl Day19 {
    fn parse<'a>(&self, input: &'a str) -> (Vec<&'a [u8]>, Vec<&'a [u8]>) {
        input
            .split_once("\n\n")
            .map(|(patterns, designs)| {
                let patterns = patterns.split(", ").map(str::as_bytes).collect();

                let designs = designs.lines().map(str::as_bytes).collect();

                (patterns, designs)
            })
            .unwrap()
    }

    fn matches_any(patterns: &Vec<&[u8]>, design: &[u8]) -> bool {
        if design.is_empty() {
            return true;
        }

        patterns.iter().any(|&pattern| {
            design.starts_with(pattern) && Self::matches_any(patterns, &design[pattern.len()..])
        })
    }

    fn count_matches<'a>(patterns: &Vec<&[u8]>, design: &'a [u8], memo: &mut Memo<'a>) -> usize {
        if design.is_empty() {
            return 1;
        }

        if let Some(&count) = memo.get(design) {
            return count;
        }

        let count = patterns
            .iter()
            .filter(|&&pattern| design.starts_with(pattern))
            .map(|&pattern| Self::count_matches(patterns, &design[pattern.len()..], memo))
            .sum();

        memo.insert(design, count);

        count
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
        Day19::count_matches(patterns, design.as_bytes(), &mut Memo::new())
    }
}
