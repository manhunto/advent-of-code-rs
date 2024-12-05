use regex::Regex;
use crate::solutions::Solution;

pub struct Day03;

impl Solution for Day03 {
    fn part_one(&self, input: &str) -> String {
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

        input.lines().map(|line| {
            re.captures_iter(line).map(|capture| {
                let left = capture[1].parse::<usize>().unwrap();
                let right = capture[2].parse::<usize>().unwrap();

                left * right
            }).sum::<usize>()
        }).sum::<usize>().to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::new()
    }
}



#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day03::Day03;
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("161", Day03.part_one(EXAMPLE));
    }
}
