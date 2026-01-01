use crate::solutions::Solution;
use std::collections::HashSet;

pub struct Day19;

// TODO: refactor me :(
impl Solution for Day19 {
    fn part_one(&self, input: &str) -> String {
        let (replacements, word) = self.parse_vec(input, false);

        self.generate_new_words(word, &replacements)
            .len()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let (replacements, start_word) = self.parse_vec(input, true);

        const TARGET: &str = "e";

        let mut count = 0usize;
        let mut current = start_word;

        while current != TARGET {
            for (pattern, replacement) in &replacements {
                let pattern_length = pattern.len();

                if current.len() < pattern_length {
                    continue;
                }

                for i in 0..=current.len().saturating_sub(pattern_length) {
                    let slice = &current[i..i + pattern_length];
                    if slice == *pattern {
                        current = format!(
                            "{}{}{}",
                            &current[..i],
                            replacement,
                            &current[i + pattern_length..]
                        );
                        count += 1;

                        break;
                    }
                }
            }
        }

        count.to_string()
    }
}

impl Day19 {
    fn parse_vec<'a>(&self, input: &'a str, reverse: bool) -> (Vec<(&'a str, &'a str)>, String) {
        let (replacements_str, word) = input.split_once("\n\n").unwrap();
        let mut replacements: Vec<(&str, &str)> =
            Vec::with_capacity(replacements_str.lines().count());

        for line in replacements_str.lines() {
            let (left, right) = line.split_once(" => ").unwrap();
            let (from, to) = if reverse {
                (right, left)
            } else {
                (left, right)
            };

            replacements.push((from, to));
        }

        (replacements, word.trim().to_string())
    }

    fn generate_new_words(
        &self,
        word: String,
        replacements: &Vec<(&str, &str)>,
    ) -> HashSet<String> {
        let mut new_words = HashSet::new();
        let str = word.as_str();

        for (pattern, replacement) in replacements {
            let length = pattern.len();

            if str.len() < length {
                continue;
            }

            for i in 0..=str.len().saturating_sub(length) {
                let slice = &str[i..i + length];
                if slice == *pattern {
                    new_words.insert(format!(
                        "{}{}{}",
                        &str[..i],
                        replacement,
                        &str[i + length..]
                    ));
                }
            }
        }

        new_words
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_ONE: &str = r#"H => HO
H => OH
O => HH

HOH"#;

    #[test]
    fn part_one_example_one() {
        assert_eq!("4", Day19.part_one(EXAMPLE_ONE));
    }

    const EXAMPLE_TWO: &str = r#"H => HO
H => OH
O => HH

HOHOHO"#;

    #[test]
    fn part_one_example_two() {
        assert_eq!("7", Day19.part_one(EXAMPLE_TWO));
    }

    const EXAMPLE_TWO_LETTERS: &str = r#"HO => HH

HOHOHO"#;

    #[test]
    fn part_one_two_letters() {
        assert_eq!("3", Day19.part_one(EXAMPLE_TWO_LETTERS));
    }

    const EXAMPLE_TWO_LETTERS_TO_ONE: &str = r#"HO => H

HOHOHO"#;

    #[test]
    fn part_one_two_letters_to_one() {
        assert_eq!("3", Day19.part_one(EXAMPLE_TWO_LETTERS_TO_ONE));
    }

    #[test]
    fn one_to_many() {
        let input = r#"A => ABC

AAA"#;
        assert_eq!("3", Day19.part_one(input));
    }

    #[test]
    fn same_position_different_lengths() {
        let input = r#"A => X
AB => Y

ABC"#;
        assert_eq!("2", Day19.part_one(input));
    }

    #[test]
    fn replacement_at_end() {
        let input = r#"C => XYZ

ABC"#;
        assert_eq!("1", Day19.part_one(input));
    }

    #[test]
    fn replacement_at_start() {
        let input = r#"A => XYZ

ABC"#;
        assert_eq!("1", Day19.part_one(input));
    }
    //
    //     const REPLACEMENTS_PART_TWO: &str = r#"e => H
    // e => O
    // H => HO
    // H => OH
    // O => HH"#;
    //
    //     #[test]
    //     fn part_two_example_one() {
    //         let input = format!("{}\n\n{}", REPLACEMENTS_PART_TWO, "HOH");
    //
    //         assert_eq!("3", Day19.part_two(&input));
    //     }
    //
    //     #[test]
    //     fn part_two_example_two() {
    //         let input = format!("{}\n\n{}", REPLACEMENTS_PART_TWO, "HOHOHO");
    //
    //         assert_eq!("6", Day19.part_two(&input));
    //     }
}
