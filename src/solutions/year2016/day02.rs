use crate::solutions::Solution;
use crate::utils::direction::Direction;
use crate::utils::direction::Direction::{East, North, South, West};
use itertools::Itertools;

pub struct Day02;

impl Solution for Day02 {
    fn part_one(&self, input: &str) -> String {
        self.solve(Keypad::normal(), input)
    }

    fn part_two(&self, input: &str) -> String {
        self.solve(Keypad::complex(), input)
    }
}

impl Day02 {
    fn solve(&self, keypad: Keypad, input: &str) -> String {
        self.parse(input)
            .iter()
            .scan(keypad, |state, directions| {
                *state = state.push(directions);

                Some(state.current_as_string())
            })
            .join("")
    }

    fn parse(&self, input: &str) -> Vec<Vec<Direction>> {
        input.lines().map(|line| self.parse_line(line)).collect()
    }

    fn parse_line(&self, line: &str) -> Vec<Direction> {
        line.chars()
            .map(|c| match c {
                'U' => North,
                'D' => South,
                'L' => West,
                'R' => East,
                _ => unreachable!(),
            })
            .collect()
    }
}

#[derive(Copy, Clone)]
enum KeypadDesign {
    Normal,
    Complex,
}

#[derive(Copy, Clone)]
struct Keypad {
    current: u8,
    design: KeypadDesign,
}

impl Keypad {
    fn normal() -> Self {
        Self {
            current: 5,
            design: KeypadDesign::Normal,
        }
    }

    fn complex() -> Self {
        Self {
            current: 5,
            design: KeypadDesign::Complex,
        }
    }

    fn push(self, directions: &[Direction]) -> Self {
        directions
            .iter()
            .fold(self, |keypad, direction| keypad.move_direction(*direction))
    }

    fn move_direction(self, direction: Direction) -> Self {
        Self {
            current: self.move_by_design(direction),
            ..self
        }
    }

    fn move_by_design(&self, direction: Direction) -> u8 {
        match self.design {
            KeypadDesign::Normal => match direction {
                North if !matches!(self.current, 1..=3) => self.current - 3,
                East if !matches!(self.current, 3 | 6 | 9) => self.current + 1,
                West if !matches!(self.current, 1 | 4 | 7) => self.current - 1,
                South if !matches!(self.current, 7..=9) => self.current + 3,
                _ => self.current,
            },
            KeypadDesign::Complex => match direction {
                North => match self.current {
                    1 | 2 | 4 | 5 | 9 => self.current,
                    3 | 13 => self.current - 2,
                    6..=8 | 10..=12 => self.current - 4,
                    _ => unreachable!(),
                },
                South => match self.current {
                    5 | 9 | 10 | 12 | 13 => self.current,
                    1 | 11 => self.current + 2,
                    2..=4 | 6..=8 => self.current + 4,
                    _ => unreachable!(),
                },
                West if !matches!(self.current, 1 | 2 | 5 | 10 | 13) => self.current - 1,
                East if !matches!(self.current, 1 | 4 | 9 | 12 | 13) => self.current + 1,
                _ => self.current,
            },
        }
    }

    fn current_as_string(&self) -> String {
        match self.current {
            n @ 1..=9 => n.to_string(),
            10 => "A".to_string(),
            11 => "B".to_string(),
            12 => "C".to_string(),
            13 => "D".to_string(),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Keypad {
        fn new_normal(current: u8) -> Self {
            Self {
                current,
                design: KeypadDesign::Normal,
            }
        }
    }

    const EXAMPLE: &str = r#"ULL
RRDDD
LURDL
UUUUD"#;

    #[test]
    fn part_one_example() {
        assert_eq!("1985", Day02.part_one(EXAMPLE));
    }

    #[test]
    fn part_two_example() {
        assert_eq!("5DB3", Day02.part_two(EXAMPLE));
    }

    #[test]
    fn keypad_normal_push_part_one_example() {
        let keypad = Keypad::normal();

        let keypad = keypad.push(&Day02.parse_line("ULL"));
        assert_eq!(1, keypad.current);

        let keypad = keypad.push(&Day02.parse_line("RRDDD"));
        assert_eq!(9, keypad.current);

        let keypad = keypad.push(&Day02.parse_line("LURDL"));
        assert_eq!(8, keypad.current);

        let keypad = keypad.push(&Day02.parse_line("UUUUD"));
        assert_eq!(5, keypad.current);
    }

    #[test]
    fn keypad_complex_push_part_one_example() {
        let keypad = Keypad::complex();

        let keypad = keypad.push(&Day02.parse_line("ULL"));
        assert_eq!("5", keypad.current_as_string());

        let keypad = keypad.push(&Day02.parse_line("RRDDD"));
        assert_eq!("D", keypad.current_as_string());

        let keypad = keypad.push(&Day02.parse_line("LURDL"));
        assert_eq!("B", keypad.current_as_string());

        let keypad = keypad.push(&Day02.parse_line("UUUUD"));
        assert_eq!("3", keypad.current_as_string());
    }

    #[test]
    fn keypad_move_direction_north() {
        assert_eq!(1, Keypad::new_normal(1).move_direction(North).current);
        assert_eq!(2, Keypad::new_normal(2).move_direction(North).current);
        assert_eq!(3, Keypad::new_normal(3).move_direction(North).current);
        assert_eq!(1, Keypad::new_normal(4).move_direction(North).current);
        assert_eq!(2, Keypad::new_normal(5).move_direction(North).current);
        assert_eq!(3, Keypad::new_normal(6).move_direction(North).current);
        assert_eq!(4, Keypad::new_normal(7).move_direction(North).current);
        assert_eq!(5, Keypad::new_normal(8).move_direction(North).current);
        assert_eq!(6, Keypad::new_normal(9).move_direction(North).current);
    }

    #[test]
    fn keypad_move_direction_east() {
        assert_eq!(2, Keypad::new_normal(1).move_direction(East).current);
        assert_eq!(3, Keypad::new_normal(2).move_direction(East).current);
        assert_eq!(3, Keypad::new_normal(3).move_direction(East).current);
        assert_eq!(5, Keypad::new_normal(4).move_direction(East).current);
        assert_eq!(6, Keypad::new_normal(5).move_direction(East).current);
        assert_eq!(6, Keypad::new_normal(6).move_direction(East).current);
        assert_eq!(8, Keypad::new_normal(7).move_direction(East).current);
        assert_eq!(9, Keypad::new_normal(8).move_direction(East).current);
        assert_eq!(9, Keypad::new_normal(9).move_direction(East).current);
    }

    #[test]
    fn keypad_move_direction_west() {
        assert_eq!(1, Keypad::new_normal(1).move_direction(West).current);
        assert_eq!(1, Keypad::new_normal(2).move_direction(West).current);
        assert_eq!(2, Keypad::new_normal(3).move_direction(West).current);
        assert_eq!(4, Keypad::new_normal(4).move_direction(West).current);
        assert_eq!(4, Keypad::new_normal(5).move_direction(West).current);
        assert_eq!(5, Keypad::new_normal(6).move_direction(West).current);
        assert_eq!(7, Keypad::new_normal(7).move_direction(West).current);
        assert_eq!(7, Keypad::new_normal(8).move_direction(West).current);
        assert_eq!(8, Keypad::new_normal(9).move_direction(West).current);
    }

    #[test]
    fn keypad_move_direction_south() {
        assert_eq!(4, Keypad::new_normal(1).move_direction(South).current);
        assert_eq!(5, Keypad::new_normal(2).move_direction(South).current);
        assert_eq!(6, Keypad::new_normal(3).move_direction(South).current);
        assert_eq!(7, Keypad::new_normal(4).move_direction(South).current);
        assert_eq!(8, Keypad::new_normal(5).move_direction(South).current);
        assert_eq!(9, Keypad::new_normal(6).move_direction(South).current);
        assert_eq!(7, Keypad::new_normal(7).move_direction(South).current);
        assert_eq!(8, Keypad::new_normal(8).move_direction(South).current);
        assert_eq!(9, Keypad::new_normal(9).move_direction(South).current);
    }
}
