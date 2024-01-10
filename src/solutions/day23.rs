use crate::direction::Direction;
use crate::grid::Grid;
use crate::point::Point;
use crate::solutions::Solution;
use crate::utils::graphs::longest_path::LongestPath;
use crate::utils::vector::Vector;
use std::collections::HashMap;

pub struct Day23;

impl Solution for Day23 {
    fn part_one(&self, input: &str) -> String {
        let can_move = |tile: char, next: Vector| -> bool {
            match tile {
                '.' => true,
                '^' => next.facing() == Direction::North,
                '<' => next.facing() == Direction::West,
                '>' => next.facing() == Direction::East,
                'v' => next.facing() == Direction::South,
                _ => unreachable!(),
            }
        };

        Self::solve(input, can_move)
    }

    fn part_two(&self, input: &str) -> String {
        let can_move = |_, _| -> bool { true };

        Self::solve(input, can_move)
    }
}

impl Day23 {
    fn solve(input: &str, can_move: fn(tile: char, next: Vector) -> bool) -> String {
        let grid: Grid<char> = Grid::from(input);
        let crossroads = Self::crossroads(&grid);

        let mut graph: HashMap<Point, Vec<Point>> = HashMap::new();
        let mut costs: HashMap<(Point, Point), usize> = HashMap::new();

        for crossroad in &crossroads {
            let movable: Vec<Vector> = crossroad
                .adjacent_vectors()
                .into_iter()
                .filter(|adj| {
                    let tile = grid.get_for_point(&adj.position());

                    tile.is_some_and(|t| t != &'#') && can_move(*tile.unwrap(), *adj)
                })
                .collect();

            for v in movable {
                let mut current = v.position();
                let mut visited: Vec<Point> = vec![*crossroad, v.position()];

                loop {
                    let moves: Vec<Vector> = current
                        .adjacent_vectors()
                        .into_iter()
                        .filter(|adj| {
                            let tile = grid.get_for_point(&adj.position());

                            tile.is_some_and(|t| t != &'#')
                                && can_move(*tile.unwrap(), *adj)
                                && !visited.contains(&adj.position())
                        })
                        .collect();

                    debug_assert!(moves.len() == 1 || moves.is_empty());

                    if moves.is_empty() {
                        break;
                    }

                    let only_available_move = moves.first().unwrap().position();
                    visited.push(only_available_move);

                    if crossroads.contains(&only_available_move) {
                        costs.insert((*crossroad, only_available_move), visited.len() - 1);
                        graph
                            .entry(*crossroad)
                            .or_default()
                            .push(only_available_move);

                        break;
                    }

                    current = only_available_move;
                }
            }
        }

        let surface = grid.surface_range();
        let start = surface.top_left_corner().east();
        let end = surface.bottom_right_corner().west();

        Self::longest_path(start, end, graph, costs).to_string()
    }

    fn crossroads(grid: &Grid<char>) -> Vec<Point> {
        let surface = grid.surface_range();
        let start = surface.top_left_corner().east();
        let end = surface.bottom_right_corner().west();

        let crossroads: Vec<Point> = grid
            .get_all_positions(&'.')
            .into_iter()
            .filter(|p| {
                p.adjacent()
                    .into_iter()
                    .filter(|adj| grid.get_for_point(adj).is_some_and(|t| t != &'#'))
                    .collect::<Vec<_>>()
                    .len()
                    .gt(&2)
            })
            .collect();

        crossroads.into_iter().chain(vec![start, end]).collect()
    }

    fn longest_path(
        start: Point,
        end: Point,
        graph: HashMap<Point, Vec<Point>>,
        costs: HashMap<(Point, Point), usize>,
    ) -> usize {
        let adjacency = |p: Point| graph.get(&p).unwrap().to_vec();
        let cost = |from: Point, to: Point| *costs.get(&(from, to)).unwrap();

        let longest_path = LongestPath::new(&adjacency, &cost);

        longest_path.cost(start, end)
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day23::Day23;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("23");

        assert_eq!("94", Day23.part_one(input.as_str()));
    }

    #[test]
    fn part_two_example_test() {
        let input = read_example("23");

        assert_eq!("154", Day23.part_two(input.as_str()));
    }
}
