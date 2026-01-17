use crate::solutions::Solution;
use crate::utils::binary::Binary;
use crate::utils::graphs::a_star::AStarBuilder;
use crate::utils::point::Point;
use itertools::Itertools;
use std::ops::Sub;

pub struct Day13 {
    destination: Point,
}

impl Solution for Day13 {
    fn part_one(&self, input: &str) -> String {
        let favorite_number = input.trim().parse::<usize>().unwrap();

        let neighbours = |point: Point| -> Vec<Point> {
            point
                .adjacent()
                .into_iter()
                .filter(|adj| adj.x >= 0 && adj.y >= 0)
                .filter(|adj| matches!(self.determine_type(adj, favorite_number), Type::OpenSpace))
                .collect_vec()
        };

        let distance = |from: Point, to: Point| from.manhattan_distance(&to) as usize;
        let a_star = AStarBuilder::init(&neighbours, &distance).build();

        a_star
            .path(Point::new(1, 1), self.destination)
            .unwrap()
            .len()
            .sub(1)
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Default for Day13 {
    fn default() -> Self {
        Self {
            destination: Point::new(31, 39),
        }
    }
}

impl Day13 {
    fn determine_type(&self, point: &Point, favorite_number: usize) -> Type {
        let (x, y) = (*point).into();
        let result = x * x + 3 * x + 2 * x * y + y + y * y;
        let result = result + favorite_number;

        let ones = Binary::from(result)
            .to_string()
            .chars()
            .filter(|c| *c == '1')
            .count();

        if ones % 2 == 0 {
            Type::OpenSpace
        } else {
            Type::Wall
        }
    }
}

enum Type {
    OpenSpace,
    Wall,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "10";

    #[test]
    fn part_one_example() {
        assert_eq!("11", day().part_one(EXAMPLE));
    }

    fn day() -> Day13 {
        Day13 {
            destination: Point::new(7, 4),
        }
    }
}
