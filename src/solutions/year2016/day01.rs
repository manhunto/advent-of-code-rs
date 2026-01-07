use crate::solutions::Solution;
use crate::utils::direction::Direction;
use crate::utils::point::Point;
use crate::utils::rotation::Rotation;
use crate::utils::vector::Vector;
use std::collections::HashSet;

pub struct Day01;

impl Solution for Day01 {
    fn part_one(&self, input: &str) -> String {
        let start = Point::new(0, 0);

        self.parse(input.trim())
            .fold(Vector::new(start, Direction::North), |vector, (r, d)| {
                vector.rotate(r).forward_with_length(d)
            })
            .position()
            .manhattan_distance(&start)
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let start = Point::new(0, 0);

        self.find_first_revisit(input.trim(), start)
            .unwrap()
            .manhattan_distance(&start)
            .to_string()
    }
}

impl Day01 {
    fn parse<'a>(&self, input: &'a str) -> impl Iterator<Item = (Rotation, isize)> + 'a {
        input.split(", ").map(|instruction| {
            let rotation = match &instruction[0..1] {
                "R" => Rotation::Clockwise,
                "L" => Rotation::CounterClockwise,
                _ => unreachable!(),
            };

            let distance = instruction[1..].parse::<isize>().unwrap();

            (rotation, distance)
        })
    }

    fn find_first_revisit(&self, input: &str, start: Point) -> Option<Point> {
        let mut vector = Vector::new(start, Direction::North);
        let mut visited = HashSet::new();
        visited.insert(start);

        for (rotation, distance) in self.parse(input) {
            vector = vector.rotate(rotation);

            for _ in 0..distance {
                vector = vector.forward();

                if !visited.insert(vector.position()) {
                    return Some(vector.position());
                }
            }
        }

        None
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
