use crate::solutions::Solution;
use std::str::FromStr;

pub struct Day01;

impl Solution for Day01 {
    fn part_one(&self, _input: &str) -> String {
        let input = self.parse(_input);
        let mut dial = SafeDial::new();
        let mut password: u16 = 0;

        for rotation in input {
            dial = dial.rotate(rotation);
            if dial.is_zero() {
                password += 1;
            }
        }

        password.to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("")
    }
}

impl Day01 {
    fn parse(&self, input: &str) -> Vec<Rotation> {
        input.lines().map(|line| line.parse().unwrap()).collect()
    }
}

struct SafeDial {
    value: u16,
}

impl SafeDial {
    fn new() -> Self {
        Self { value: 50 }
    }

    fn rotate(&self, rotation: Rotation) -> Self {
        const DIAL_NUMBERS_COUNT: i16 = 100;

        let new_value: i16 = match rotation.direction {
            Direction::Left => self.value as i16 - rotation.distance as i16,
            Direction::Right => self.value as i16 + rotation.distance as i16,
        };

        Self {
            value: new_value.rem_euclid(DIAL_NUMBERS_COUNT) as u16,
        }
    }

    fn is_zero(&self) -> bool {
        self.value == 0
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct Rotation {
    direction: Direction,
    distance: u16,
}

#[cfg(test)]
impl Rotation {
    fn new(direction: Direction, distance: u16) -> Self {
        Self {
            direction,
            distance,
        }
    }

    fn left(distance: u16) -> Self {
        Self::new(Direction::Left, distance)
    }

    fn right(distance: u16) -> Self {
        Self::new(Direction::Right, distance)
    }
}

impl FromStr for Rotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = match s.chars().next() {
            Some('L') => Direction::Left,
            Some('R') => Direction::Right,
            _ => return Err(String::from("Invalid direction")),
        };

        let distance = s[1..].parse::<u16>().unwrap();

        Ok(Self {
            direction,
            distance,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2025::day01::{Day01, Rotation, SafeDial};
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("3", Day01.part_one(EXAMPLE));
    }

    #[test]
    fn rotation_parse_test() {
        assert_eq!("L30".parse::<Rotation>().unwrap(), Rotation::left(30));
        assert_eq!("R48".parse::<Rotation>().unwrap(), Rotation::right(48));
    }

    #[test]
    fn safe_dial_rotate_test() {
        let mut dial = SafeDial::new();

        dial = dial.rotate(Rotation::left(68));
        assert_eq!(82, dial.value);
        assert!(!dial.is_zero());

        dial = dial.rotate(Rotation::left(30));
        assert_eq!(52, dial.value);
        assert!(!dial.is_zero());

        dial = dial.rotate(Rotation::right(48));
        assert_eq!(0, dial.value);
        assert!(dial.is_zero());

        dial = dial.rotate(Rotation::left(5));
        assert_eq!(95, dial.value);
        assert!(!dial.is_zero());
    }
}
