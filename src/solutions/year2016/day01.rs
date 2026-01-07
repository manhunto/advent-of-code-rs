use crate::solutions::Solution;
use crate::utils::direction::Direction;
use crate::utils::point::Point;
use crate::utils::vector::Vector;
use std::collections::HashSet;

pub struct Day01;

impl Solution for Day01 {
    fn part_one(&self, input: &str) -> String {
        let start = Point::new(0, 0);
        let mut vector = Vector::new(start, Direction::North);

        self.parse(input.trim()).for_each(|(rotation, distance)| {
            vector = match rotation {
                RotationDirection::Right => vector.rotate_cw(),
                RotationDirection::Left => vector.rotate_ccw(),
            };

            vector = vector.forward_with_length(distance);
        });

        vector.position().manhattan_distance(&start).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let start = Point::new(0, 0);
        let mut vector = Vector::new(start, Direction::North);

        let mut visited = HashSet::new();
        visited.insert(start);

        'outer: for (rotation, distance) in self.parse(input.trim()) {
            vector = match rotation {
                RotationDirection::Right => vector.rotate_cw(),
                RotationDirection::Left => vector.rotate_ccw(),
            };

            for _ in 0..distance {
                vector = vector.forward();

                let position = vector.position();
                if visited.contains(&position) {
                    break 'outer;
                }

                visited.insert(position);
            }
        }

        vector.position().manhattan_distance(&start).to_string()
    }
}

enum RotationDirection {
    Right,
    Left,
}

impl Day01 {
    fn parse<'a>(&self, input: &'a str) -> impl Iterator<Item = (RotationDirection, isize)> + 'a {
        input.split(", ").map(|instruction| {
            let rotation = match &instruction[0..1] {
                "R" => RotationDirection::Right,
                "L" => RotationDirection::Left,
                _ => unreachable!(),
            };

            let distance = instruction[1..].parse::<isize>().unwrap();

            (rotation, distance)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        assert_eq!("5", Day01.part_one("R2, L3"));
        assert_eq!("2", Day01.part_one("R2, R2, R2"));
        assert_eq!("12", Day01.part_one("R5, L5, R5, R3"));
    }

    #[test]
    fn part_two_example() {
        assert_eq!("4", Day01.part_two("R8, R4, R4, R8"));
    }

    #[test]
    fn part_two_bug_break_just_inner_loop() {
        assert_eq!("0", Day01.part_two("R1, R1, R1, R1, R100"));
    }
}
