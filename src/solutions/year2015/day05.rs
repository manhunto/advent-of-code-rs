use crate::solutions::Solution;
use itertools::Itertools;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const BANNED: [&str; 4] = ["ab", "cd", "pq", "xy"];

pub struct Day05;

impl Solution for Day05 {
    fn part_one(&self, input: &str) -> String {
        input
            .lines()
            .filter(|word| self.is_nice_part_one(word))
            .count()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        input
            .lines()
            .filter(|word| self.is_nice_part_two(word))
            .count()
            .to_string()
    }
}

impl Day05 {
    fn is_nice_part_one(&self, word: &str) -> bool {
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

    fn is_nice_part_two(&self, word: &str) -> bool {
        self.pair_twice(word) && self.repeated_with_letter_between(word)
    }

    fn pair_twice(&self, word: &str) -> bool {
        let vec = word.chars().collect_vec();

        for (i, w) in vec.windows(2).enumerate() {
            let pair = format!("{}{}", w[0], w[1]);

            for j in i + 2..vec.len() - 1 {
                if pair == format!("{}{}", vec[j], vec[j + 1]) {
                    return true;
                }
            }
        }

        false
    }

    fn repeated_with_letter_between(&self, word: &str) -> bool {
        word.chars().tuple_windows().any(|(c1, _, c2)| c1 == c2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_nice_part_one() {
        assert!(Day05.is_nice_part_one("ugknbfddgicrmopn"));
        assert!(Day05.is_nice_part_one("aaa"));
        assert!(!Day05.is_nice_part_one("jchzalrnumimnmhp"));
        assert!(!Day05.is_nice_part_one("haegwjzuvuyypxyu"));
        assert!(!Day05.is_nice_part_one("dvszwmarrgswjxmb"));
    }

    #[test]
    fn vowels() {
        assert!(Day05.vowels("aei"));
        assert!(Day05.vowels("xazegov"));
        assert!(Day05.vowels("aeiouaeiouaeiou"));
        assert!(!Day05.vowels("aed"));
    }

    #[test]
    fn letter_twice_in_row() {
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
    fn is_nice_part_two() {
        assert!(Day05.is_nice_part_two("qjhvhtzxzqqjkmpb"));
        assert!(Day05.is_nice_part_two("xxyxx"));
        assert!(!Day05.is_nice_part_two("uurcxstgmygtbstg"));
        assert!(!Day05.is_nice_part_two("ieodomkazucvgmuy"));
    }

    #[test]
    fn pair_twice() {
        assert!(Day05.pair_twice("xyxy"));
        assert!(Day05.pair_twice("aabcdefgaa"));
        assert!(!Day05.pair_twice("aaa"));
    }

    #[test]
    fn repeated_with_letter_between() {
        assert!(Day05.repeated_with_letter_between("xyx"));
        assert!(!Day05.repeated_with_letter_between("xx"));
        assert!(Day05.repeated_with_letter_between("abcdefeghi"));
        assert!(Day05.repeated_with_letter_between("aaa"));
    }
}
