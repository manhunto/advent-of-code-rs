use crate::solutions::Solution;
use std::collections::{HashMap, HashSet};
use std::ops::Range;

pub struct Day19;

impl Solution for Day19 {
    fn part_one(&self, input: &str) -> String {
        let (replacements, word) = self.parse(input);
        let mut new_words = HashSet::new();

        self.search_for_replacement(&replacements, &word, 1, &mut new_words);
        self.search_for_replacement(&replacements, &word, 2, &mut new_words);

        new_words.len().to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day19 {
    fn parse<'a>(&self, input: &'a str) -> (HashMap<&'a str, Vec<&'a str>>, Word) {
        let (replacements_str, word) = input.split_once("\n\n").unwrap();
        let mut replacements: HashMap<&str, Vec<&str>> = HashMap::new();

        for replacement in replacements_str.lines() {
            let (from, to) = replacement.split_once(" => ").unwrap();
            replacements.entry(from).or_default().push(to);
        }

        (replacements, Word::new(word))
    }

    fn search_for_replacement(
        &self,
        replacements: &HashMap<&str, Vec<&str>>,
        word: &Word,
        length: usize,
        new_words: &mut HashSet<Word>,
    ) {
        let str = word.word.as_str();

        for i in 0..=str.len().saturating_sub(length) {
            let pattern = &str[i..i + length];

            if let Some(char_replacements) = replacements.get(pattern) {
                for &replacement in char_replacements {
                    new_words.insert(word.replace(i..i + length, replacement));
                }
            }
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
struct Word {
    word: String,
    replacements: usize,
}

impl Word {
    fn new(word: &str) -> Self {
        Self {
            word: word.to_string(),
            replacements: 0,
        }
    }

    fn replace(&self, i: Range<usize>, replacement: &str) -> Self {
        let word = format!(
            "{}{}{}",
            &self.word[..i.start],
            replacement,
            &self.word[i.end..]
        );

        Self {
            word,
            replacements: self.replacements + 1,
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

    const REPLACEMENTS_PART_TWO: &str = r#"e => H
e => O
H => HO
H => OH
O => HH"#;

    #[test]
    fn part_two_example_one() {
        let input = format!("{}\n\n{}", REPLACEMENTS_PART_TWO, "HOH");

        assert_eq!("3", Day19.part_two(&input));
    }

    #[test]
    fn part_two_example_two() {
        let input = format!("{}\n\n{}", REPLACEMENTS_PART_TWO, "HOHOHO");

        assert_eq!("6", Day19.part_two(&input));
    }
}
