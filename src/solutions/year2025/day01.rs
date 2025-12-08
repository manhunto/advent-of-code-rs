use crate::solutions::Solution;
use std::fmt::{Display, Formatter};
use std::ops::Div;
use std::str::FromStr;

pub struct Day01;

impl Solution for Day01 {
    fn part_one(&self, input: &str) -> String {
        SafeDial::new()
            .apply_rotations(&self.parse(input))
            .stops_at_zero()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        SafeDial::new()
            .apply_rotations(&self.parse(input))
            .points_to_zero()
            .to_string()
    }
}

impl Day01 {
    fn parse(&self, input: &str) -> Vec<Rotation> {
        input.lines().map(|line| line.parse().unwrap()).collect()
    }
}

struct SafeDial {
    position: u16,
    zero_stops: u16,
    zero_clicks: u16,
}

impl SafeDial {
    const DIAL_NUMBERS_COUNT: i16 = 100;

    fn new() -> Self {
        Self {
            position: 50,
            zero_stops: 0,
            zero_clicks: 0,
        }
    }

    fn apply_rotations(&self, rotations: &[Rotation]) -> Self {
        rotations
            .iter()
            .fold(SafeDial::new(), |acc, rotation| acc.rotate(rotation))
    }

    fn rotate(&self, rotation: &Rotation) -> Self {
        let position = self.new_position(rotation);
        let zero_stops = self.new_zero_stops(position);
        let zero_clicks = self.new_zero_clicks(rotation);

        Self {
            position,
            zero_stops,
            zero_clicks,
        }
    }

    fn new_position(&self, rotation: &Rotation) -> u16 {
        let diff: i16 = match rotation.direction {
            Direction::Left => self.position as i16 - rotation.distance as i16,
            Direction::Right => self.position as i16 + rotation.distance as i16,
        };

        diff.rem_euclid(Self::DIAL_NUMBERS_COUNT) as u16
    }

    fn new_zero_stops(&self, new_position: u16) -> u16 {
        match new_position {
            0 => self.zero_stops + 1,
            _ => self.zero_stops,
        }
    }

    fn new_zero_clicks(&self, rotation: &Rotation) -> u16 {
        let numerator = match rotation.direction {
            Direction::Right => self.position + rotation.distance,
            Direction::Left => {
                rotation.distance
                    + if self.position == 0 {
                        0
                    } else {
                        100 - self.position
                    }
            }
        };

        self.zero_clicks + numerator.div(Self::DIAL_NUMBERS_COUNT as u16)
    }

    fn stops_at_zero(&self) -> u16 {
        self.zero_stops
    }

    fn points_to_zero(&self) -> u16 {
        self.zero_clicks
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => write!(f, "L"),
            Direction::Right => write!(f, "R"),
        }
    }
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

impl Display for Rotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.direction, self.distance)
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
    fn part_two_example_test() {
        assert_eq!("6", Day01.part_two(EXAMPLE));
    }

    #[test]
    fn rotation_parse_test() {
        assert_eq!("L2".parse::<Rotation>().unwrap(), Rotation::left(2));
        assert_eq!("L30".parse::<Rotation>().unwrap(), Rotation::left(30));
        assert_eq!("R48".parse::<Rotation>().unwrap(), Rotation::right(48));
    }

    #[test]
    fn zero_stops_test() {
        let mut dial = SafeDial::new();

        dial = dial.rotate(&Rotation::left(68));
        assert_eq!(82, dial.position);
        assert_eq!(0, dial.stops_at_zero());

        dial = dial.rotate(&Rotation::left(30));
        assert_eq!(52, dial.position);
        assert_eq!(0, dial.stops_at_zero());

        dial = dial.rotate(&Rotation::right(48));
        assert_eq!(0, dial.position);
        assert_eq!(1, dial.stops_at_zero());

        dial = dial.rotate(&Rotation::left(5));
        assert_eq!(95, dial.position);
        assert_eq!(1, dial.stops_at_zero());

        dial = dial.rotate(&Rotation::right(60));
        assert_eq!(55, dial.position);
        assert_eq!(1, dial.stops_at_zero());

        dial = dial.rotate(&Rotation::left(55));
        assert_eq!(0, dial.position);
        assert_eq!(2, dial.stops_at_zero());

        dial = dial.rotate(&Rotation::left(1));
        assert_eq!(99, dial.position);
        assert_eq!(2, dial.stops_at_zero());

        dial = dial.rotate(&Rotation::left(99));
        assert_eq!(0, dial.position);
        assert_eq!(3, dial.stops_at_zero());

        dial = dial.rotate(&Rotation::right(14));
        assert_eq!(14, dial.position);
        assert_eq!(3, dial.stops_at_zero());

        dial = dial.rotate(&Rotation::left(82));
        assert_eq!(32, dial.position);
        assert_eq!(3, dial.stops_at_zero());
    }

    #[test]
    fn position_at_zero_on_right() {
        let mut dial = SafeDial::new(); // 50

        dial = dial.rotate(&Rotation::right(12)); // 62
        assert_eq!(0, dial.points_to_zero());

        dial = dial.rotate(&Rotation::right(52)); // 14
        assert_eq!(1, dial.points_to_zero());

        dial = dial.rotate(&Rotation::right(265)); // 79
        assert_eq!(3, dial.points_to_zero());

        dial = dial.rotate(&Rotation::right(421)); // 0
        assert_eq!(8, dial.points_to_zero());

        dial = dial.rotate(&Rotation::right(500)); // 0
        assert_eq!(13, dial.points_to_zero());

        dial = dial.rotate(&Rotation::right(27)); // 27
        assert_eq!(13, dial.points_to_zero());
    }

    #[test]
    fn position_at_zero_on_left() {
        let mut dial = SafeDial::new(); // 50

        dial = dial.rotate(&Rotation::left(12));
        assert_eq!(38, dial.position);
        assert_eq!(0, dial.points_to_zero());

        dial = dial.rotate(&Rotation::left(99));
        assert_eq!(39, dial.position);
        assert_eq!(1, dial.points_to_zero());

        // 1 39, 165
        // 2 39, 65
        // 3 52
        dial = dial.rotate(&Rotation::left(265));
        assert_eq!(74, dial.position);
        assert_eq!(4, dial.points_to_zero());

        // 1 74, 89
        // 2 85
        dial = dial.rotate(&Rotation::left(189));
        assert_eq!(85, dial.position);
        assert_eq!(6, dial.points_to_zero());

        // 1 85, 120
        // 2 85, 20
        // 2 65
        dial = dial.rotate(&Rotation::left(220));
        assert_eq!(65, dial.position);
        assert_eq!(8, dial.points_to_zero());

        // 1 65, 265
        // 2 65, 165
        // 3 65, 65
        // 4 0
        dial = dial.rotate(&Rotation::left(365)); // 0
        assert_eq!(12, dial.points_to_zero());

        // 1 0, 400
        // 2 0, 300
        // 3 0, 200
        // 4 0, 100
        // 5 0
        dial = dial.rotate(&Rotation::left(500)); // 0
        assert_eq!(17, dial.points_to_zero());

        dial = dial.rotate(&Rotation::left(27)); // 73
        assert_eq!(17, dial.points_to_zero());
    }
}
