use crate::solutions::Solution;
use regex::Regex;

pub struct Day03;

impl Solution for Day03 {
    fn part_one(&self, input: &str) -> String {
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

        input
            .lines()
            .map(|line| {
                re.captures_iter(line)
                    .map(|capture| {
                        let left = capture[1].parse::<usize>().unwrap();
                        let right = capture[2].parse::<usize>().unwrap();

                        left * right
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let dont_positions: Vec<usize> = Regex::new(r"don't\(\)")
            .unwrap()
            .find_iter(input)
            .map(|m| m.end())
            .collect();
        let do_positions: Vec<usize> = Regex::new(r"do\(\)")
            .unwrap()
            .find_iter(input)
            .map(|m| m.end())
            .collect();

        println!("{:?}", dont_positions);
        println!("{:?}", do_positions);

        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

        input
            .lines()
            .map(|line| {
                re.captures_iter(line)
                    .map(|capture| {
                        let mul_position = capture.get(0).unwrap().start();
                        let left = capture[1].parse::<usize>().unwrap();
                        let right = capture[2].parse::<usize>().unwrap();

                        println!("{} {} {}", mul_position, left, right);

                        let do_matched = do_positions
                            .clone()
                            .into_iter()
                            .filter(|x| x < &mul_position)
                            .collect::<Vec<usize>>();

                        let do_before = do_matched.last().unwrap_or(&0);

                        let dont_matched = dont_positions
                            .clone()
                            .into_iter()
                            .filter(|x| x < &mul_position)
                            .collect::<Vec<usize>>();

                        let dont_before = dont_matched.last();

                        println!("do: {}", do_before);
                        println!("dont: {:?}", dont_before);

                        if dont_before.is_none() {
                            return left * right;
                        }

                        if dont_before.is_some_and(|p| p < do_before) {
                            return left * right;
                        }

                        0usize
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day03::Day03;
    use crate::solutions::Solution;

    const EXAMPLE: &str =
        r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
    const EXAMPLE_TWO: &str =
        r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("161", Day03.part_one(EXAMPLE));
    }

    #[test]
    fn part_two_example_test() {
        assert_eq!("48", Day03.part_two(EXAMPLE_TWO));
    }
}
