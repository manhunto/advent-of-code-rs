use crate::solutions::Solution;
use itertools::Itertools;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const BANNED: [&str; 4] = ["ab", "cd", "pq", "xy"];

pub struct Day05;

impl Solution for Day05 {
    fn part_one(&self, input: &str) -> String {
        input
            .lines()
            .filter(|word| self.is_nice(word))
            .count()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day05 {
    fn is_nice(&self, word: &str) -> bool {
        self.vowels(word) && self.letter_twice_in_row(word) && self.not_contains_banned(word)
    }

    fn vowels(&self, word: &str) -> bool {
        word.chars().filter(|c| VOWELS.contains(c)).count() >= 3
    }

    fn letter_twice_in_row(&self, word: &str) -> bool {
        word.chars().tuple_windows().any(|(c1, c2)| c1 == c2)
    }

    fn not_contains_banned(&self, word: &str) -> bool {
        word.chars()
            .tuple_windows()
            .all(|(c1, c2)| !BANNED.contains(&format!("{}{}", c1, c2).as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vowels() {
        assert!(Day05.vowels("aei"));
        assert!(Day05.vowels("xazegov"));
        assert!(Day05.vowels("aeiouaeiouaeiou"));
        assert!(!Day05.vowels("aed"));
    }

    #[test]
    fn letter_twice_in_rowe() {
        assert!(Day05.letter_twice_in_row("abcdde"));
        assert!(!Day05.letter_twice_in_row("abcde"));
        assert!(Day05.letter_twice_in_row("xxabcde"));
        assert!(Day05.letter_twice_in_row("abcdezz"));
        assert!(!Day05.letter_twice_in_row("ababababab"));
    }

    #[test]
    fn not_contains_banned() {
        assert!(!Day05.not_contains_banned("abcdde"));
        assert!(Day05.not_contains_banned("other"));
    }

    #[test]
    fn is_nice() {
        assert!(Day05.is_nice("ugknbfddgicrmopn"));
        assert!(Day05.is_nice("aaa"));
        assert!(!Day05.is_nice("jchzalrnumimnmhp"));
        assert!(!Day05.is_nice("haegwjzuvuyypxyu"));
        assert!(!Day05.is_nice("dvszwmarrgswjxmb"));
    }
}
