use crate::solutions::Solution;
use crate::utils::graphs::a_star::AStarBuilder;
use crate::utils::point::Point;
use std::collections::HashSet;

const MAX_STEPS: usize = 50;
const START_POSITION: Point = Point::new(1, 1);

pub struct Day13 {
    destination: Point,
}

impl Solution for Day13 {
    fn part_one(&self, input: &str) -> String {
        let favorite_number = self.parse_favorite_number(input);
        let neighbours_fn = move |point: Point| {
            point
                .adjacent()
                .into_iter()
                .filter(move |adjacent_point| self.is_open_space(adjacent_point, favorite_number))
        };

        let a_star = AStarBuilder::init(&neighbours_fn, &Self::manhattan_heuristic).build();

        a_star
            .path(START_POSITION, self.destination)
            .map(|path| path.len().saturating_sub(1))
            .unwrap_or(0)
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let favorite_number = self.parse_favorite_number(input);

        self.count_reachable_positions(favorite_number, MAX_STEPS)
            .to_string()
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
    fn parse_favorite_number(&self, input: &str) -> usize {
        input
            .trim()
            .parse()
            .expect("Input should be a valid number")
    }

    fn manhattan_heuristic(from: Point, to: Point) -> usize {
        from.manhattan_distance(&to) as usize
    }

    fn is_open_space(&self, point: &Point, favorite_number: usize) -> bool {
        if point.x < 0 || point.y < 0 {
            return false;
        }

        let (x, y) = (*point).into();

        let result = x * x + 3 * x + 2 * x * y + y + y * y;
        let result = result + favorite_number;

        result.count_ones().is_multiple_of(2)
    }

    fn count_reachable_positions(&self, favorite_number: usize, max_steps: usize) -> usize {
        let neighbour_fn = move |point: Point| {
            point
                .adjacent()
                .into_iter()
                .filter(move |adjacent_point| self.is_open_space(adjacent_point, favorite_number))
        };

        let mut visited: HashSet<Point> = HashSet::new();
        visited.insert(START_POSITION);

        let mut current_frontier: Vec<Point> = vec![START_POSITION];

        for _ in 0..max_steps {
            let mut next_frontier: Vec<Point> = Vec::new();

            for current_point in current_frontier {
                for neighbour in neighbour_fn(current_point) {
                    if visited.insert(neighbour) {
                        next_frontier.push(neighbour);
                    }
                }
            }

            current_frontier = next_frontier;
        }

        visited.len()
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
