use crate::solutions::Solution;
use crate::utils::direction::Direction;
use crate::utils::grid::Grid;
use crate::utils::point::Point;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

pub struct Day12;

impl Solution for Day12 {
    fn part_one(&self, _input: &str) -> String {
        let grid: Grid<char> = Grid::from(_input);
        let surface = grid.surface_range();
        let mut visited: HashSet<Point> = HashSet::new();

        let mut price = 0;

        while visited.len() != surface.area() {
            let not_visited_func = |point: &Point, _element: &char| !visited.contains(point);
            let not_visited = grid.find(&not_visited_func).unwrap();

            let (flood_filled, perimeter, area) =
                self.flood_fill(not_visited.0, not_visited.1, &grid);

            price += perimeter * area;

            visited.extend(flood_filled);
        }

        price.to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        let grid: Grid<char> = Grid::from(_input);
        let surface = grid.surface_range();
        let mut visited: HashSet<Point> = HashSet::new();

        let mut price = 0;

        while visited.len() != surface.area() {
            let not_visited_func = |point: &Point, _element: &char| !visited.contains(point);
            let not_visited = grid.find(&not_visited_func).unwrap();

            let (flood_filled, _, area) = self.flood_fill(not_visited.0, not_visited.1, &grid);

            let sides = self.sides(&flood_filled);
            price += sides * area;

            visited.extend(flood_filled);
        }

        price.to_string()
    }
}

impl Day12 {
    fn flood_fill(
        &self,
        point: &Point,
        element: &char,
        grid: &Grid<char>,
    ) -> (Vec<Point>, usize, usize) {
        let mut checked: HashSet<Point> = HashSet::from_iter(vec![*point]);
        let mut queue = VecDeque::from(vec![*point]);
        let mut perimeter: usize = 0;

        while let Some(point) = queue.pop_front() {
            let adjacent = point.adjacent();
            let neighbours_with_the_same_element = adjacent
                .iter()
                .filter(|p| grid.get_for_point(p).is_some_and(|e| e == element));

            let how_many_surrounding = &neighbours_with_the_same_element.clone().count();

            let neighbours_not_processed = neighbours_with_the_same_element
                .filter(|p| !checked.contains(p))
                .collect_vec();

            perimeter += 4 - how_many_surrounding;
            checked.extend(neighbours_not_processed.clone());

            for neighbour in neighbours_not_processed {
                queue.push_back(*neighbour);
            }
        }

        let vec = checked.into_iter().collect_vec();
        let area = vec.len();

        (vec, perimeter, area)
    }

    /// Calculate corners
    fn sides(&self, region: &[Point]) -> usize {
        let edges: HashSet<(Point, Direction)> = region
            .iter()
            .flat_map(|&p| {
                let mut edges = vec![];
                if !region.contains(&p.north()) {
                    edges.push((p, Direction::North))
                }

                if !region.contains(&p.south()) {
                    edges.push((p, Direction::South))
                }

                if !region.contains(&p.west()) {
                    edges.push((p, Direction::West))
                }

                if !region.contains(&p.east()) {
                    edges.push((p, Direction::East))
                }

                edges
            })
            .collect();

        edges
            .iter()
            .filter(|(p, dir)| match dir {
                Direction::North if !edges.contains(&(p.east(), Direction::North)) => true,
                Direction::South if !edges.contains(&(p.east(), Direction::South)) => true,
                Direction::West if !edges.contains(&(p.south(), Direction::West)) => true,
                Direction::East if !edges.contains(&(p.south(), Direction::East)) => true,
                _ => false,
            })
            .count()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day12::Day12;
    use crate::solutions::Solution;
    use crate::utils::grid::Grid;

    const EXAMPLE_1: &str = r#"AAAA
BBCD
BBCC
EEEC"#;

    #[test]
    fn part_one_example_1() {
        let result = (10 * 4) + (10 * 4) + (8 * 4) + (3 * 8) + 4;

        assert_eq!(result.to_string(), Day12.part_one(EXAMPLE_1));
    }

    #[test]
    #[ignore]
    fn part_two_example_2() {
        assert_eq!("80", Day12.part_one(EXAMPLE_1));
    }

    const EXAMPLE_2: &str = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;

    #[test]
    fn part_one_example_2() {
        let result = 4 * 4 + (21 * 36);

        assert_eq!(result.to_string(), Day12.part_one(EXAMPLE_2));
    }

    const EXAMPLE_3: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

    #[test]
    fn part_one_example_3() {
        assert_eq!("1930", Day12.part_one(EXAMPLE_3));
    }

    #[test]
    fn sides() {
        let grid = Grid::<char>::from(EXAMPLE_1);

        assert_eq!(4, Day12.sides(&grid.get_all_positions(&'A')));
        assert_eq!(4, Day12.sides(&grid.get_all_positions(&'B')));
        assert_eq!(4, Day12.sides(&grid.get_all_positions(&'D')));
        assert_eq!(4, Day12.sides(&grid.get_all_positions(&'E')));
        assert_eq!(8, Day12.sides(&grid.get_all_positions(&'C')));
    }
}
