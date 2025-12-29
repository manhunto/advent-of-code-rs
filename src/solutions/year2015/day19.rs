use crate::solutions::Solution;
use std::collections::{HashMap, HashSet};

pub struct Day19;

impl Solution for Day19 {
    fn part_one(&self, input: &str) -> String {
        let (replacements, word) = self.parse(input);
        let mut new_words = HashSet::new();
        new_words.extend(self.search_for_replacement(&replacements, word.clone(), 1));
        new_words.extend(self.search_for_replacement(&replacements, word, 2));

        new_words.len().to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day19 {
    fn parse(&self, input: &str) -> (HashMap<String, Vec<String>>, String) {
        let (replacements_str, word) = input.split_once("\n\n").unwrap();
        let mut replacements: HashMap<String, Vec<String>> = HashMap::new();

        for replacement in replacements_str.lines() {
            let (from, to) = replacement.split_once(" => ").unwrap();

            replacements
                .entry(from.to_string())
                .or_default()
                .push(to.to_string());
        }

        (replacements, word.to_string())
    }

    fn search_for_replacement(
        &self,
        replacements: &HashMap<String, Vec<String>>,
        word: String,
        length: usize,
    ) -> HashSet<String> {
        let mut new_words = HashSet::new();

        for i in 0..word.len() {
            if i <= word.len() - length {
                let pattern = word[i..i + length].to_string();

                if let Some(char_replacements) = replacements.get(&pattern) {
                    for char_replacement in char_replacements {
                        let mut new_word = word.clone();

                        new_word.replace_range(i..i + pattern.len(), char_replacement);
                        new_words.insert(new_word);
                    }
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
}
