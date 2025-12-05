use crate::solutions::Solution;
use std::collections::HashMap;

pub struct Day11;

type Number = usize;
type Cache = HashMap<(Number, u8), usize>;

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
        let mut cache: Cache = HashMap::new();

        self.blink(numbers, 0, times, 0, &mut cache).to_string()
    }

    fn parse(&self, input: &str) -> Vec<Number> {
        input
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect()
    }

    fn blink(
        &self,
        numbers: Vec<Number>,
        iteration: u8,
        max_iteration: u8,
        stones_count: usize,
        cache: &mut Cache,
    ) -> usize {
        if iteration == max_iteration {
            return stones_count + numbers.len();
        }

        numbers
            .iter()
            .map(|&number| {
                let cache_key = (number, iteration);

                if let Some(&cached_result) = cache.get(&cache_key) {
                    return cached_result;
                }

                let new_numbers = self.blink_for_number(number);
                let result = self.blink(
                    new_numbers,
                    iteration + 1,
                    max_iteration,
                    stones_count,
                    cache,
                );

                cache.insert(cache_key, result);

                result
            })
            .sum::<usize>()
            + stones_count
    }

    fn blink_for_number(&self, number: Number) -> Vec<Number> {
        if number == 0 {
            return vec![1];
        }

        let number_str = number.to_string();
        let len = number_str.len();
        if len.is_multiple_of(2) {
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

    const EXAMPLE: &str = r#"125 17"#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("55312", Day11.part_one(EXAMPLE));
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

    fn blink_for_number(number: Number) -> Vec<Number> {
        Day11.blink_for_number(number)
    }
}
