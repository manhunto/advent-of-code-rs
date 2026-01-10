use crate::solutions::Solution;
use std::cmp::Ordering::Equal;
use std::collections::HashMap;

pub struct Day04;

impl Solution for Day04 {
    fn part_one(&self, input: &str) -> String {
        input
            .lines()
            .map(Room::from)
            .filter(|r| r.is_real())
            .map(|r| r.sector_id)
            .sum::<u64>()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

struct Room<'a> {
    checksum: &'a str,
    sector_id: u64,
    encrypted_parts: Vec<&'a str>,
}

impl<'a> From<&'a str> for Room<'a> {
    fn from(value: &'a str) -> Self {
        let parts: Vec<&str> = value.split_terminator(&['-', '[', ']']).collect();

        let checksum = parts[parts.len() - 1];
        let number = parts[parts.len() - 2].parse::<u64>().unwrap();
        let rest = &parts[..parts.len() - 2];

        Self {
            checksum,
            sector_id: number,
            encrypted_parts: rest.into(),
        }
    }
}

impl<'a> Room<'a> {
    fn is_real(&self) -> bool {
        self.calculate_checksum() == self.checksum
    }

    fn calculate_checksum(&self) -> String {
        let mut map: HashMap<char, u8> = HashMap::new();

        for r in self.encrypted_parts.iter() {
            for c in r.chars() {
                *map.entry(c).or_insert(0) += 1;
            }
        }

        let mut items: Vec<_> = map.iter().collect();
        items.sort_by(|(a_k, a_v), (b_k, b_v)| {
            // First compare by keys
            match b_v.cmp(a_v) {
                // And by value
                Equal => a_k.cmp(b_k),
                other => other,
            }
        });

        items.iter().take(5).map(|(k, _)| *k).collect::<String>()
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
}
