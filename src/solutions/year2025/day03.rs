use crate::solutions::Solution;
use itertools::Itertools;
use std::fmt::Debug;

pub struct Day03;

impl Solution for Day03 {
    fn part_one(&self, input: &str) -> String {
        input
            .lines()
            .map(|bank| self.largest_joltage(bank, 2))
            .sum::<u64>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        input
            .lines()
            .map(|bank| self.largest_joltage(bank, 12))
            .sum::<u64>()
            .to_string()
    }
}

impl Day03 {
    fn largest_joltage(&self, bank: &str, digits: usize) -> u64 {
        let numbers: Vec<u64> = bank
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();

        let mut largest = Joltage::init_for(digits);

        let max = numbers.len() - digits + 1;
        for i in 0..max {
            Self::calculate_level(&mut largest, &numbers, i, 0, digits - 1);
        }

        largest.joltage().unwrap()
    }

    fn calculate_level(
        largest: &mut Joltage,
        numbers: &[u64],
        index: usize,
        digit_position: usize,
        max_digit_position: usize,
    ) {
        let current = numbers[index];

        // println!("i:{} v:{} dp:{}", index, current, digit_position);
        // println!("{:?}", largest);

        let digits_left = max_digit_position - digit_position;
        let numbers_left_in_numbers = numbers.len() - 1 - index;

        if digits_left > numbers_left_in_numbers {
            return;
        }

        // println!("dl:{}, nl:{}", digits_left, numbers_left_in_numbers);

        if largest
            .update_if_grater(digit_position, current)
            .is_ok_and(|result| result)
            && digit_position != max_digit_position
        {
            for i in (index + 1)..numbers.len() {
                Self::calculate_level(largest, numbers, i, digit_position + 1, max_digit_position)
            }
        }
    }
}

struct Joltage {
    digits: Vec<Option<u64>>,
}

impl Joltage {
    fn init_for(digits_number: usize) -> Self {
        let mut digits = Vec::with_capacity(digits_number);
        for i in 0..digits_number {
            digits.insert(i, None);
        }

        Self { digits }
    }

    fn update_if_grater(&mut self, index: usize, new: u64) -> Result<bool, String> {
        if self.digits.iter().take(index).any(|value| value.is_none()) {
            return Err(format!("There is missing digit before index {}", index));
        }

        let current = self.digits.get(index).unwrap();
        if current.is_none_or(|current| current < new) {
            if let Some(elem) = self.digits.get_mut(index) {
                *elem = Some(new);
            }

            self.clear_after_index(index);

            return Ok(true);
        }

        Ok(false)
    }

    fn clear_after_index(&mut self, index: usize) {
        self.digits
            .iter_mut()
            .skip(index + 1)
            .for_each(|value| *value = None);
    }

    fn joltage(&self) -> Result<u64, String> {
        if self.digits.iter().any(|value| value.is_none()) {
            return Err(String::from("Missing digits"));
        }

        let last_index = self.digits.len() as u32 - 1;

        Ok(self
            .digits
            .iter()
            .enumerate()
            .map(|(key, digit)| digit.unwrap() * 10u64.pow(last_index - key as u32))
            .sum::<u64>())
    }
}

impl Debug for Joltage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let digits = self
            .digits
            .iter()
            .map(|value| match value {
                None => "-".to_string(),
                Some(digit) => digit.to_string(),
            })
            .join("");

        write!(f, "{}", digits)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2025::day03::{Day03, Joltage};
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("357", Day03.part_one(EXAMPLE));
    }

    #[test]
    fn part_two_example_test() {
        assert_eq!("3121910778619", Day03.part_two(EXAMPLE));
    }

    #[test]
    fn largest_joltage() {
        // Part one
        assert_eq!(98, Day03.largest_joltage("987654321111111", 2));
        assert_eq!(89, Day03.largest_joltage("811111111111119", 2));
        assert_eq!(78, Day03.largest_joltage("234234234234278", 2));
        assert_eq!(92, Day03.largest_joltage("818181911112111", 2));

        // Part two
        assert_eq!(987654321111, Day03.largest_joltage("987654321111111", 12));
        assert_eq!(811111111119, Day03.largest_joltage("811111111111119", 12));
        assert_eq!(434234234278, Day03.largest_joltage("234234234234278", 12));
        assert_eq!(888911112111, Day03.largest_joltage("818181911112111", 12));

        // Custom
        assert_eq!(43, Day03.largest_joltage("243", 2));
        assert_eq!(443, Day03.largest_joltage("24432", 3));
        assert_eq!(219, Day03.largest_joltage("2119", 3));
    }

    #[test]
    fn joltage_update_if_greater() {
        let mut joltage = Joltage::init_for(2);
        assert!(joltage.update_if_grater(0, 1).is_ok());
        assert!(joltage.update_if_grater(1, 2).is_ok());

        assert_eq!(&Some(1), joltage.digits.first().unwrap());
        assert_eq!(&Some(2), joltage.digits.get(1).unwrap());

        assert!(joltage.update_if_grater(0, 2).is_ok());

        assert_eq!(&Some(2), joltage.digits.first().unwrap());
        assert_eq!(&None, joltage.digits.get(1).unwrap());
    }

    #[test]
    fn joltage_update_if_greater_doesnt_clear_previous_digits() {
        let mut joltage = Joltage::init_for(4);

        assert!(joltage.update_if_grater(0, 2).is_ok_and(|r| r));
        assert!(joltage.update_if_grater(1, 1).is_ok_and(|r| r));
        assert!(joltage.update_if_grater(2, 3).is_ok_and(|r| r));
        assert!(joltage.update_if_grater(3, 7).is_ok_and(|r| r));

        assert!(joltage.update_if_grater(2, 4).is_ok_and(|r| r));
        assert!(joltage.update_if_grater(3, 5).is_ok_and(|r| r));

        assert_eq!(2145, joltage.joltage().unwrap());
    }

    #[test]
    fn joltage_update_if_greater_cannot_update_invalid_index() {
        let mut joltage = Joltage::init_for(3);

        assert!(joltage.update_if_grater(0, 1).is_ok());
        assert!(joltage.update_if_grater(2, 3).is_err());
    }

    #[test]
    fn joltage_get_joltage() {
        let mut joltage = Joltage::init_for(3);

        let _ = joltage.update_if_grater(0, 2).is_ok();
        assert!(joltage.joltage().is_err());

        let _ = joltage.update_if_grater(1, 3).is_ok();
        assert!(joltage.joltage().is_err());

        let _ = joltage.update_if_grater(2, 5).is_ok();
        assert_eq!(235, joltage.joltage().unwrap());
    }
}
