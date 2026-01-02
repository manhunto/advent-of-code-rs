use crate::solutions::Solution;
use std::collections::HashSet;

pub struct Day19;

impl Solution for Day19 {
    fn part_one(&self, input: &str) -> String {
        let (replacements, molecule) = self.parse(input);

        let mut new_molecules = HashSet::new();

        for (from, to) in &replacements {
            for (i, _) in molecule.match_indices(from) {
                let mut new_mol = molecule.clone();
                new_mol.replace_range(i..i + from.len(), to);
                new_molecules.insert(new_mol);
            }
        }

        new_molecules.len().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let (replacements, start_molecule) = self.parse(input);

        let replacements_rev: Vec<(&str, &str)> =
            replacements.iter().map(|(from, to)| (*to, *from)).collect();

        let target = "e";
        let mut steps = 0usize;
        let mut current = start_molecule;

        while current != target {
            for (from, to) in &replacements_rev {
                if *to == target && current.len() != from.len() {
                    continue;
                }

                if let Some(index) = current.find(from) {
                    current.replace_range(index..index + from.len(), to);
                    steps += 1;
                    break;
                }
            }
        }

        steps.to_string()
    }
}

impl Day19 {
    fn parse<'a>(&self, input: &'a str) -> (Vec<(&'a str, &'a str)>, String) {
        let (replacements_str, word) = input.split_once("\n\n").unwrap();

        let replacements = replacements_str
            .lines()
            .map(|line| line.split_once(" => ").unwrap())
            .collect();

        (replacements, word.trim().to_string())
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

    #[test]
    fn trap_test_premature_reduction_to_e() {
        // Tests if the algorithm avoids reducing "CA" -> "Ce" (error),
        // instead of "CA" -> "AA" -> "A" -> "e" (success).
        let replacements = "e => A\nA => C\nA => AA";
        let input = format!("{}\n\n{}", replacements, "CA");

        assert_eq!("3", Day19.part_two(&input));
    }
}
