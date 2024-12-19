use crate::solutions::Solution;
use itertools::Itertools;

pub struct Day19;

impl Solution for Day19 {
    fn part_one(&self, input: &str) -> String {
        let (patterns, designs) = input
            .split_once("\n\n")
            .map(|(patterns, designs)| {
                let patterns = patterns
                    .split_terminator(", ")
                    .map(|p| p.as_bytes())
                    .collect_vec();

                let designs = designs.lines().map(|line| line.as_bytes()).collect_vec();

                (patterns, designs)
            })
            .unwrap();

        designs
            .iter()
            .filter(|design| Self::matches_any(patterns.clone(), design))
            .count()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from('0')
    }
}

impl Day19 {
    fn matches_any(patterns: Vec<&[u8]>, design: &[u8]) -> bool {
        patterns.iter().any(|&pattern| {
            if design.is_empty() {
                return true;
            }

            if design.len() < pattern.len() {
                return false;
            }

            if &design[..pattern.len()] == pattern {
                let new = &design[pattern.len()..];

                return Self::matches_any(patterns.clone(), new);
            }

            false
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day19::Day19;
    use crate::solutions::Solution;

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
    fn matches_any() {
        const PATTERNS: [&str; 8] = ["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];

        let patterns = PATTERNS.map(|pat| pat.as_bytes()).to_vec();

        assert!(Day19::matches_any(patterns.clone(), "brwrr".as_bytes()));
        assert!(Day19::matches_any(patterns.clone(), "bggr".as_bytes()));
        assert!(Day19::matches_any(patterns.clone(), "gbbr".as_bytes()));
        assert!(Day19::matches_any(patterns.clone(), "rrbgbr".as_bytes()));
        assert!(!Day19::matches_any(patterns.clone(), "ubwu".as_bytes()));
        assert!(Day19::matches_any(patterns.clone(), "bwurrg".as_bytes()));
        assert!(Day19::matches_any(patterns.clone(), "brgr".as_bytes()));
        assert!(!Day19::matches_any(patterns.clone(), "bbrgwb".as_bytes()));
    }
}
