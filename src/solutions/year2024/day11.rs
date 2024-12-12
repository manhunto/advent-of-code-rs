use crate::solutions::Solution;

pub struct Day11;

type Number = usize;

impl Solution for Day11 {
    fn part_one(&self, input: &str) -> String {
        self.solve(input, 25)
    }

    fn part_two(&self, input: &str) -> String {
        self.solve(input, 75)
    }
}

impl Day11 {
    fn solve(&self, input: &str, times: u8) -> String {
        let numbers = self.parse(input);

        self.blink_fold(numbers, 0, times, 0).to_string()
    }

    fn parse(&self, input: &str) -> Vec<Number> {
        input
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect()
    }

    fn blink_fold(
        &self,
        numbers: Vec<Number>,
        iteration: u8,
        max_iteration: u8,
        stones_count: usize,
    ) -> usize {
        if iteration == max_iteration {
            return stones_count + numbers.len();
        }

        let mut tmp_stones_count: usize = 0;

        for number in numbers {
            let new_numbers = self.blink_for_number(number);

            // todo cache it - key is number + iteration
            tmp_stones_count +=
                self.blink_fold(new_numbers, iteration + 1, max_iteration, stones_count);
        }

        stones_count + tmp_stones_count
    }

    #[allow(dead_code)]
    fn blink(&self, numbers: Vec<Number>) -> Vec<Number> {
        numbers
            .into_iter()
            .flat_map(|n| self.blink_for_number(n))
            .collect()
    }

    fn blink_for_number(&self, number: Number) -> Vec<Number> {
        if number == 0 {
            return vec![1];
        }

        let number_str = number.to_string();
        let len = number_str.len();
        if len % 2 == 0 {
            let middle = len / 2;

            return vec![
                number_str[..middle].parse::<Number>().unwrap(),
                number_str[middle..].parse::<Number>().unwrap(),
            ];
        }

        vec![number * 2024]
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day11::{Day11, Number};
    use crate::solutions::Solution;
    use itertools::Itertools;

    const EXAMPLE: &str = r#"0 1 10 99 999"#;
    const EXAMPLE_2: &str = r#"125 17"#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("55312", Day11.part_one(EXAMPLE_2));
    }

    #[test]
    fn blink_for_number_test() {
        assert_eq!(vec![1], blink_for_number(0));
        assert_eq!(vec![2024], blink_for_number(1));
        assert_eq!(vec![1, 0], blink_for_number(10));
        assert_eq!(vec![9, 9], blink_for_number(99));
        assert_eq!(vec![2021976], blink_for_number(999));
        assert_eq!(vec![253, 0], blink_for_number(253000));
    }

    #[test]
    fn blink_test() {
        let parsed = Day11.parse(EXAMPLE);
        let blink_result: Vec<Number> = blink(parsed);
        let result_str = blink_result.iter().map(|n| n.to_string()).join(" ");

        assert_eq!("1 2024 1 0 9 9 2021976", result_str);
    }

    fn blink(numbers: Vec<Number>) -> Vec<Number> {
        Day11.blink(numbers)
    }

    fn blink_for_number(number: Number) -> Vec<Number> {
        Day11.blink_for_number(number)
    }
}
