use crate::solutions::Solution;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Range;

pub struct Day19;

impl Solution for Day19 {
    fn part_one(&self, input: &str) -> String {
        let (replacements, word) = self.parse(input);

        self.generate_new_words(&word, &replacements)
            .len()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let (replacements, target) = self.parse(input);

        let start_word = Word::new("e");

        let mut new_words: VecDeque<Word> =
            VecDeque::from_iter(self.generate_new_words(&start_word, &replacements));

        let mut results = Vec::new();

        while let Some(current) = new_words.pop_front() {
            if current.word == target.word {
                results.push(current);

                continue;
            }

            if current.word.len() <= target.word.len() {
                new_words.extend(self.generate_new_words(&current, &replacements));
            }
        }

        results
            .iter()
            .map(|word| word.replacements)
            .min()
            .unwrap_or(0)
            .to_string()
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

    fn generate_new_words(
        &self,
        word: &Word,
        replacements: &HashMap<&str, Vec<&str>>,
    ) -> HashSet<Word> {
        let mut new_words = HashSet::new();
        let str = word.word.as_str();

        for (haystack, values) in replacements {
            let length = haystack.len();

            if str.len() < length {
                continue;
            }

            for i in 0..=str.len().saturating_sub(length) {
                let slice = &str[i..i + length];
                if slice == *haystack {
                    for &replacement in values {
                        new_words.insert(word.replace(i..i + length, replacement));
                    }
                }
            }
        }

        new_words
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
