use crate::solutions::Solution;
use std::collections::{HashMap, HashSet};

pub struct Day19;

impl Solution for Day19 {
    fn part_one(&self, input: &str) -> String {
        let (replacements, word) = self.parse(input);
        let mut new_words = HashSet::new();
        self.search_for_replacement(&replacements, word, 1, &mut new_words);
        self.search_for_replacement(&replacements, word, 2, &mut new_words);

        new_words.len().to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day19 {
    fn parse<'a>(&self, input: &'a str) -> (HashMap<&'a str, Vec<&'a str>>, &'a str) {
        let (replacements_str, word) = input.split_once("\n\n").unwrap();
        let mut replacements: HashMap<&str, Vec<&str>> = HashMap::new();

        for replacement in replacements_str.lines() {
            let (from, to) = replacement.split_once(" => ").unwrap();
            replacements.entry(from).or_default().push(to);
        }

        (replacements, word.trim())
    }

    fn search_for_replacement(
        &self,
        replacements: &HashMap<&str, Vec<&str>>,
        word: &str,
        length: usize,
        new_words: &mut HashSet<String>,
    ) {
        for i in 0..=word.len().saturating_sub(length) {
            let pattern = &word[i..i + length];

            if let Some(char_replacements) = replacements.get(pattern) {
                for &replacement in char_replacements {
                    let new_word = format!("{}{}{}", &word[..i], replacement, &word[i + length..]);
                    new_words.insert(new_word);
                }
            }
        }
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
