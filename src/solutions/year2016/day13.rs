use crate::solutions::Solution;
use crate::utils::binary::Binary;
use crate::utils::graphs::a_star::AStarBuilder;
use crate::utils::point::Point;
use itertools::Itertools;
use std::collections::HashSet;
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
                .filter(|adj| self.can_move_here(adj, favorite_number))
                .collect_vec()
        };

        let distance = |from: Point, to: Point| from.manhattan_distance(&to) as usize;
        let a_star = AStarBuilder::init(&neighbours, &distance).build();

        a_star
            .path(self.starting_point(), self.destination)
            .unwrap()
            .len()
            .sub(1)
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let favorite_number = input.trim().parse::<usize>().unwrap();

        let neighbours = |point: Point| -> Vec<Point> {
            point
                .adjacent()
                .into_iter()
                .filter(|adj| self.can_move_here(adj, favorite_number))
                .collect_vec()
        };

        let mut visited: HashSet<Point> = HashSet::new();
        visited.insert(self.starting_point());
        let mut on_current_step: Vec<Point> = vec![self.starting_point()];

        for _ in 0..50 {
            let mut new: Vec<Point> = Vec::new();
            for point in on_current_step {
                let nexts = neighbours(point);
                for next in nexts {
                    if visited.insert(next) {
                        new.push(next);
                    }
                }
            }

            on_current_step = new;
        }

        visited.len().to_string()
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
    fn can_move_here(&self, point: &Point, favorite_number: usize) -> bool {
        if point.x < 0 || point.y < 0 {
            return false;
        }

        let (x, y) = (*point).into();

        let result = x * x + 3 * x + 2 * x * y + y + y * y;
        let result = result + favorite_number;

        let ones = Binary::from(result)
            .to_string()
            .chars()
            .filter(|c| *c == '1')
            .count();

        ones % 2 == 0
    }

    fn starting_point(&self) -> Point {
        Point::new(1, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "10";

    #[test]
    fn part_one_example() {
        assert_eq!("11", day().part_one(EXAMPLE));
    }

    #[test]
    fn part_two_example() {
        assert_eq!("151", day().part_two(EXAMPLE));
    }

    fn day() -> Day13 {
        Day13 {
            destination: Point::new(7, 4),
        }
    }
}
