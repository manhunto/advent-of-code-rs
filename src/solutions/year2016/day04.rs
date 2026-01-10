use crate::solutions::Solution;
use itertools::Itertools;
use std::collections::HashMap;

pub struct Day04;

impl Solution for Day04 {
    fn part_one(&self, input: &str) -> String {
        self.parse(input)
            .filter(|r| r.is_real())
            .map(|r| r.sector_id)
            .sum::<u32>()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        self.parse(_input)
            .find(|r| r.decrypt() == "northpole object storage")
            .unwrap()
            .sector_id
            .to_string()
    }
}

impl Day04 {
    fn parse<'a>(&self, input: &'a str) -> impl Iterator<Item = Room<'a>> {
        input.lines().map(Room::from)
    }
}

struct Room<'a> {
    checksum: &'a str,
    sector_id: u32,
    encrypted_parts: Vec<&'a str>,
}

impl<'a> From<&'a str> for Room<'a> {
    fn from(value: &'a str) -> Self {
        let parts: Vec<&str> = value.split_terminator(&['-', '[', ']']).collect();

        let checksum = parts[parts.len() - 1];
        let number = parts[parts.len() - 2].parse::<u32>().unwrap();
        let rest = &parts[..parts.len() - 2];

        Self {
            checksum,
            sector_id: number,
            encrypted_parts: rest.to_vec(),
        }
    }
}

impl<'a> Room<'a> {
    fn is_real(&self) -> bool {
        self.calculate_checksum() == self.checksum
    }

    fn calculate_checksum(&self) -> String {
        let mut counts: HashMap<char, u32> = HashMap::new();

        self.encrypted_parts
            .iter()
            .flat_map(|s| s.chars())
            .for_each(|c| *counts.entry(c).or_insert(0) += 1);

        counts
            .into_iter()
            .sorted_by(|(a_char, a_count), (b_char, b_count)| {
                b_count.cmp(a_count).then_with(|| a_char.cmp(b_char))
            })
            .take(5)
            .map(|(c, _)| c)
            .collect()
    }

    fn decrypt(&self) -> String {
        self.encrypted_parts
            .iter()
            .map(|p| p.chars().map(|c| self.rotate_letter(c)).collect::<String>())
            .join(" ")
    }

    fn rotate_letter(&self, letter: char) -> char {
        let a = 'a' as u32;

        let as_int = letter as u32 - a;
        let rotated = (as_int + self.sector_id) % 26;
        let new = rotated + a;

        new as u8 as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]"#;

    #[test]
    fn part_one_example() {
        assert_eq!("1514", Day04.part_one(EXAMPLE));
    }

    #[test]
    fn room_is_real() {
        assert!(Room::from("aaaaa-bbb-z-y-x-123[abxyz]").is_real());
        assert!(Room::from("a-b-c-d-e-f-g-h-987[abcde]").is_real());
        assert!(Room::from("not-a-real-room-404[oarel]").is_real());
        assert!(!Room::from("totally-real-room-200[decoy]").is_real());
    }

    #[test]
    fn room_decrypt() {
        assert_eq!(
            "very encrypted name",
            Room::from("qzmt-zixmtkozy-ivhz-343[dummy]").decrypt()
        );
    }
}
