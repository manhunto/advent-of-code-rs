use crate::solutions::Solution;

pub struct Day03;

impl Solution for Day03 {
    fn part_one(&self, input: &str) -> String {
        input
            .lines()
            .map(|bank| self.largest_joltage(bank))
            .sum::<u64>()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day03 {
    fn largest_joltage(&self, bank: &str) -> u64 {
        let numbers: Vec<u64> = bank
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();

        let mut largest_first_digit: Option<u64> = None;
        let mut largest_second_digit: Option<u64> = None;

        for first_digit_index in 0..numbers.len() - 1 {
            let first_digit = numbers[first_digit_index];
            if largest_first_digit.is_none_or(|digit| digit < first_digit) {
                largest_first_digit = Some(first_digit);
                largest_second_digit = None;

                for &second_digit in numbers.iter().skip(first_digit_index + 1) {
                    if largest_second_digit.is_none_or(|digit| digit < second_digit) {
                        largest_second_digit = Some(second_digit);
                    }
                }
            }
        }

        largest_first_digit.unwrap() * 10 + largest_second_digit.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2025::day03::Day03;
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
    fn largest_joltage() {
        assert_eq!(98, Day03.largest_joltage("987654321111111"));
        assert_eq!(89, Day03.largest_joltage("811111111111119"));
        assert_eq!(78, Day03.largest_joltage("234234234234278"));
        assert_eq!(92, Day03.largest_joltage("818181911112111"));
    }
}
