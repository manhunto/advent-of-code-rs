use crate::solutions::Solution;
use crate::utils::point::Point;
use itertools::Itertools;
use std::str::FromStr;

pub struct Day13;

impl Solution for Day13 {
    fn part_one(&self, _input: &str) -> String {
        let machines: Vec<Machine> = _input
            .split_terminator("\n\n")
            .map(|s| s.parse().unwrap())
            .collect();

        println!("{:?}", machines);

        String::from('0')
    }

    fn part_two(&self, _input: &str) -> String {
        String::from('0')
    }
}

#[derive(Debug)]
struct Machine {
    #[allow(dead_code)]
    a: Point,
    #[allow(dead_code)]
    b: Point,
    #[allow(dead_code)]
    prize_location: Point,
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (button_a_str, button_b_str, prize_str) = s.lines().collect_tuple().unwrap();

        let prize_location = prize_str
            .trim_start_matches("Prize: ")
            .split_once(", ")
            .map(|(x, y)| {
                Point::new(
                    x.trim_start_matches("X=").parse().unwrap(),
                    y.trim_start_matches("Y=").parse().unwrap(),
                )
            })
            .unwrap();

        let button_fn = |source: &str| -> Point {
            source
                .trim_start_matches("Button A: ")
                .trim_start_matches("Button B: ")
                .split_once(", ")
                .map(|(x, y)| {
                    Point::new(
                        x.trim_start_matches("X+").parse().unwrap(),
                        y.trim_start_matches("Y+").parse().unwrap(),
                    )
                })
                .unwrap()
        };

        Ok(Self {
            a: button_fn(button_a_str),
            b: button_fn(button_b_str),
            prize_location,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day13::Day13;
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("0", Day13.part_one(EXAMPLE));
    }
}
