use crate::solutions::Solution;
use crate::utils::point::Point;
use itertools::Itertools;
use std::str::FromStr;

pub struct Day13;

impl Solution for Day13 {
    fn part_one(&self, input: &str) -> String {
        input
            .split_terminator("\n\n")
            .map(|s| s.parse::<Machine>().unwrap())
            .filter_map(|machine| {
                machine
                    .solve_2x2_system()
                    .map(|solution| solution.0 * 3 + solution.1)
            })
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        input
            .split_terminator("\n\n")
            .map(|s| s.parse::<Machine>().unwrap())
            .filter_map(|machine| {
                machine
                    .move_prize_by_10000000000000()
                    .solve_2x2_system()
                    .map(|solution| solution.0 * 3 + solution.1)
            })
            .sum::<usize>()
            .to_string()
    }
}

#[derive(Debug)]
struct Machine {
    a: Point,
    b: Point,
    prize_location: Point,
}

impl Machine {
    fn solve_2x2_system(&self) -> Option<(usize, usize)> {
        let a = self.a.x;
        let b = self.b.x;
        let c = self.a.y;
        let d = self.b.y;
        let e = self.prize_location.x;
        let f = self.prize_location.y;

        // Compute the determinant
        let det = a * d - b * c;

        // Check if determinant is zero (no unique solution)
        if det == 0 {
            return None; // System is singular
        }

        // Compute the numerators for x and y
        let numerator_x = e * d - b * f;
        let numerator_y = a * f - e * c;

        // Check if solutions are divisible by determinant
        if numerator_x % det != 0 || numerator_y % det != 0 {
            return None; // No integer solution
        }

        // Compute the solutions
        let x = numerator_x / det;
        let y = numerator_y / det;

        Some((x as usize, y as usize))
    }

    fn move_prize_by_10000000000000(&self) -> Self {
        const DIFF: isize = 10000000000000;
        let diff = Point::new(DIFF, DIFF);

        Self {
            a: self.a,
            b: self.b,
            prize_location: self.prize_location + diff,
        }
    }
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
        assert_eq!("480", Day13.part_one(EXAMPLE));
    }

    #[test]
    fn part_two_example_test() {
        assert_eq!("875318608908", Day13.part_two(EXAMPLE));
    }
}
