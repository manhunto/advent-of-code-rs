use crate::solutions::Solution;
use crate::utils::direction::Direction;
use crate::utils::direction::Direction::{East, North, South, West};
use itertools::Itertools;

pub struct Day02;

impl Solution for Day02 {
    fn part_one(&self, input: &str) -> String {
        self.parse(input)
            .iter()
            .scan(Keypad::default(), |state, x| {
                *state = state.push(x);

                Some(state.current)
            })
            .join("")
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day02 {
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
struct Keypad {
    current: u8,
}

impl Keypad {
    fn push(&self, directions: &[Direction]) -> Self {
        directions
            .iter()
            .fold(*self, |keypad, direction| keypad.move_direction(*direction))
    }

    fn move_direction(&self, direction: Direction) -> Self {
        let new = match direction {
            North if ![1, 2, 3].contains(&self.current) => self.current - 3,
            East if ![3, 6, 9].contains(&self.current) => self.current + 1,
            West if ![1, 4, 7].contains(&self.current) => self.current - 1,
            South if ![7, 8, 9].contains(&self.current) => self.current + 3,
            _ => self.current,
        };

        Self { current: new }
    }
}

impl Default for Keypad {
    fn default() -> Self {
        Self { current: 5 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Keypad {
        fn new(current: u8) -> Self {
            Keypad { current }
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
    fn keypad_push_example() {
        let keypad = Keypad::default();

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
    fn keypad_move_direction_north() {
        assert_eq!(1, Keypad::new(1).move_direction(North).current);
        assert_eq!(2, Keypad::new(2).move_direction(North).current);
        assert_eq!(3, Keypad::new(3).move_direction(North).current);
        assert_eq!(1, Keypad::new(4).move_direction(North).current);
        assert_eq!(2, Keypad::new(5).move_direction(North).current);
        assert_eq!(3, Keypad::new(6).move_direction(North).current);
        assert_eq!(4, Keypad::new(7).move_direction(North).current);
        assert_eq!(5, Keypad::new(8).move_direction(North).current);
        assert_eq!(6, Keypad::new(9).move_direction(North).current);
    }

    #[test]
    fn keypad_move_direction_east() {
        assert_eq!(2, Keypad::new(1).move_direction(East).current);
        assert_eq!(3, Keypad::new(2).move_direction(East).current);
        assert_eq!(3, Keypad::new(3).move_direction(East).current);
        assert_eq!(5, Keypad::new(4).move_direction(East).current);
        assert_eq!(6, Keypad::new(5).move_direction(East).current);
        assert_eq!(6, Keypad::new(6).move_direction(East).current);
        assert_eq!(8, Keypad::new(7).move_direction(East).current);
        assert_eq!(9, Keypad::new(8).move_direction(East).current);
        assert_eq!(9, Keypad::new(9).move_direction(East).current);
    }

    #[test]
    fn keypad_move_direction_west() {
        assert_eq!(1, Keypad::new(1).move_direction(West).current);
        assert_eq!(1, Keypad::new(2).move_direction(West).current);
        assert_eq!(2, Keypad::new(3).move_direction(West).current);
        assert_eq!(4, Keypad::new(4).move_direction(West).current);
        assert_eq!(4, Keypad::new(5).move_direction(West).current);
        assert_eq!(5, Keypad::new(6).move_direction(West).current);
        assert_eq!(7, Keypad::new(7).move_direction(West).current);
        assert_eq!(7, Keypad::new(8).move_direction(West).current);
        assert_eq!(8, Keypad::new(9).move_direction(West).current);
    }

    #[test]
    fn keypad_move_direction_south() {
        assert_eq!(4, Keypad::new(1).move_direction(South).current);
        assert_eq!(5, Keypad::new(2).move_direction(South).current);
        assert_eq!(6, Keypad::new(3).move_direction(South).current);
        assert_eq!(7, Keypad::new(4).move_direction(South).current);
        assert_eq!(8, Keypad::new(5).move_direction(South).current);
        assert_eq!(9, Keypad::new(6).move_direction(South).current);
        assert_eq!(7, Keypad::new(7).move_direction(South).current);
        assert_eq!(8, Keypad::new(8).move_direction(South).current);
        assert_eq!(9, Keypad::new(9).move_direction(South).current);
    }
}
