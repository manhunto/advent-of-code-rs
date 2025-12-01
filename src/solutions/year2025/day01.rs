use crate::solutions::Solution;
use std::str::FromStr;

pub struct Day01;

impl Solution for Day01 {
    fn part_one(&self, input: &str) -> String {
        let dial = self.run(input);

        dial.zero_stops.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let dial = self.run(input);

        (dial.zero_stops + dial.zero_clicks).to_string()
    }
}

impl Day01 {
    fn parse(&self, input: &str) -> Vec<Rotation> {
        input.lines().map(|line| line.parse().unwrap()).collect()
    }

    fn run(&self, input: &str) -> SafeDial {
        let rotations = self.parse(input);
        let mut dial = SafeDial::new();

        for rotation in rotations {
            dial = dial.rotate(rotation);
        }

        dial
    }
}

struct SafeDial {
    value: u16,
    zero_stops: u16,
    zero_clicks: u16,
}

impl SafeDial {
    const DIAL_NUMBERS_COUNT: i16 = 100;

    fn new() -> Self {
        Self {
            value: 50,
            zero_stops: 0,
            zero_clicks: 0,
        }
    }

    fn rotate(&self, rotation: Rotation) -> Self {
        let new_value: i16 = match rotation.direction {
            Direction::Left => self.value as i16 - rotation.distance as i16,
            Direction::Right => self.value as i16 + rotation.distance as i16,
        };

        let mut zero_stops = self.zero_stops;
        let mut zero_clicks = self.zero_clicks;

        let value = new_value.rem_euclid(Self::DIAL_NUMBERS_COUNT) as u16;
        let new_zero_clicks = new_value
            .div_euclid(Self::DIAL_NUMBERS_COUNT)
            .unsigned_abs();

        zero_clicks += new_zero_clicks;

        if value == 0 {
            zero_stops += 1;
        }

        if (value == 0 || self.value == 0) && new_zero_clicks > 0 {
            zero_clicks -= 1;
        }

        Self {
            value,
            zero_stops,
            zero_clicks,
        }
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
    fn part_two_example_test() {
        assert_eq!("6", Day01.part_two(EXAMPLE));
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
        assert_eq!(0, dial.zero_stops);
        assert_eq!(1, dial.zero_clicks);

        dial = dial.rotate(Rotation::left(30));
        assert_eq!(52, dial.value);
        assert_eq!(0, dial.zero_stops);
        assert_eq!(1, dial.zero_clicks);

        dial = dial.rotate(Rotation::right(48));
        assert_eq!(0, dial.value);
        assert_eq!(1, dial.zero_stops);
        assert_eq!(1, dial.zero_clicks);

        dial = dial.rotate(Rotation::left(5));
        assert_eq!(95, dial.value);
        assert_eq!(1, dial.zero_stops);
        assert_eq!(1, dial.zero_clicks);

        dial = dial.rotate(Rotation::right(60));
        assert_eq!(55, dial.value);
        assert_eq!(1, dial.zero_stops);
        assert_eq!(2, dial.zero_clicks);

        dial = dial.rotate(Rotation::left(55));
        assert_eq!(0, dial.value);
        assert_eq!(2, dial.zero_stops);
        assert_eq!(2, dial.zero_clicks);

        dial = dial.rotate(Rotation::left(1));
        assert_eq!(99, dial.value);
        assert_eq!(2, dial.zero_stops);
        assert_eq!(2, dial.zero_clicks);

        dial = dial.rotate(Rotation::left(99));
        assert_eq!(0, dial.value);
        assert_eq!(3, dial.zero_stops);
        assert_eq!(2, dial.zero_clicks);

        dial = dial.rotate(Rotation::right(14));
        assert_eq!(14, dial.value);
        assert_eq!(3, dial.zero_stops);
        assert_eq!(2, dial.zero_clicks);

        dial = dial.rotate(Rotation::left(82));
        assert_eq!(32, dial.value);
        assert_eq!(3, dial.zero_stops);
        assert_eq!(3, dial.zero_clicks);
    }
}
