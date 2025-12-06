use crate::solutions::Solution;

pub struct Day03;

impl Solution for Day03 {
    fn part_one(&self, input: &str) -> String {
        self.solve(input, 2)
    }

    fn part_two(&self, input: &str) -> String {
        self.solve(input, 12)
    }
}

impl Day03 {
    fn solve(&self, input: &str, digits: usize) -> String {
        input
            .lines()
            .map(|bank| self.largest_joltage(bank, digits))
            .sum::<u64>()
            .to_string()
    }

    /// This function finds the largest number of a given length (`digits`) that can be formed
    /// by selecting digits from the input `bank` string in order.
    ///
    /// It uses a greedy algorithm. A greedy algorithm makes the locally optimal choice at each
    /// step with the hope of finding a global optimum. In this case, for each position in our
    /// target number, we greedily select the largest possible digit from the available input
    /// digits.
    ///
    /// # Example
    ///
    /// Let's find the largest 3-digit number from "24432".
    ///
    /// 1. **First digit:** We need to leave at least 2 digits for the rest of the number.
    ///    So, we search for the largest digit in the first `5 - 3 + 1 = 3` digits: "244".
    ///    The largest digit is 4. We pick the first occurrence at index 1.
    ///    - Result: `4`
    ///    - We continue searching from the next position, index 2.
    ///
    /// 2. **Second digit:** We need to find 2 more digits. We search from index 2 ("432").
    ///    We need to leave 1 digit for the end. So we search in "43". The largest is 4.
    ///    - Result: `44`
    ///    - We continue searching from index 3.
    ///
    /// 3. **Third digit:** We need 1 more digit. We search from index 3 ("32").
    ///    We can search until the end. The largest digit in "32" is 3.
    ///    - Result: `443`
    ///
    /// The final largest number is 443. This greedy approach works because choosing a larger
    /// digit at a more significant position (further to the left) always yields a larger
    /// overall number.
    fn largest_joltage(&self, bank: &str, digits: usize) -> u64 {
        if digits == 0 {
            return 0;
        }

        let numbers: Vec<u64> = bank
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();

        if numbers.len() < digits {
            return 0;
        }

        let mut result_digits = Vec::with_capacity(digits);
        let mut start_index = 0;

        for i in 0..digits {
            let remaining_digits_to_find = digits - i;
            let end_index = numbers.len() - remaining_digits_to_find;
            let search_slice = &numbers[start_index..=end_index];

            let max_digit = *search_slice.iter().max().unwrap();
            let max_digit_offset = search_slice
                .iter()
                .position(|&digit| digit == max_digit)
                .unwrap();

            result_digits.push(max_digit);
            start_index += max_digit_offset + 1;
        }

        result_digits.iter().fold(0, |acc, &digit| acc * 10 + digit)
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
}
