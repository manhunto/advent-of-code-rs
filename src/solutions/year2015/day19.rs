use crate::solutions::Solution;
use std::collections::{HashMap, HashSet};

pub struct Day19;

impl Solution for Day19 {
    fn part_one(&self, input: &str) -> String {
        let (replacements_str, word) = input.split_once("\n\n").unwrap();
        let mut replacements: HashMap<String, Vec<String>> = HashMap::new();
        let word = word.to_string();

        let mut count = 0usize;

        for replacement in replacements_str.lines() {
            let (from, to) = replacement.split_once(" => ").unwrap();

            replacements
                .entry(from.to_string())
                .or_default()
                .push(to.to_string());

            count += 1;
        }

        let mut new_words = HashSet::with_capacity(count);

        for (i, w) in word.char_indices() {
            if let Some(char_replacements) = replacements.get(&w.to_string()) {
                for char_replacement in char_replacements {
                    let mut new_word = word.clone();

                    new_word.replace_range(i..i + 1, char_replacement);
                    new_words.insert(new_word);
                }
            }

            if i < word.len() - 1 {
                let two = word[i..i + 2].to_string();

                if let Some(char_replacements) = replacements.get(&two) {
                    for char_replacement in char_replacements {
                        let mut new_word = word.clone();

                        new_word.replace_range(i..i + two.len(), char_replacement);
                        new_words.insert(new_word);
                    }
                }
            }
        }

        new_words.len().to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
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
}
