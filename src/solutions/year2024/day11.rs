use crate::solutions::Solution;

pub struct Day11;

type Number = usize;

impl Solution for Day11 {
    fn part_one(&self, input: &str) -> String {
        let mut numbers = self.parse(input);

        for _ in 0..25 {
            numbers = self.blink(numbers)
        }

        numbers.len().to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from('0')
    }
}

impl Day11 {
    fn parse(&self, input: &str) -> Vec<Number> {
        input
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect()
    }

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
    fn blink_for_number() {
        assert_eq!(vec![1], Day11.blink_for_number(0));
        assert_eq!(vec![2024], Day11.blink_for_number(1));
        assert_eq!(vec![1, 0], Day11.blink_for_number(10));
        assert_eq!(vec![9, 9], Day11.blink_for_number(99));
        assert_eq!(vec![2021976], Day11.blink_for_number(999));
        assert_eq!(vec![253, 0], Day11.blink_for_number(253000));
    }

    #[test]
    fn blink() {
        let blink_result: Vec<Number> = Day11.blink(Day11.parse(EXAMPLE));
        let result_str = blink_result.iter().map(|n| n.to_string()).join(" ");

        assert_eq!("1 2024 1 0 9 9 2021976", result_str);
    }
}
