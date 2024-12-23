use crate::solutions::Solution;
use crate::utils::direction::Direction::East;
use crate::utils::graphs::dijkstra::Dijkstra;
use crate::utils::grid::Grid;
use crate::utils::point::Point;
use crate::utils::vector::Vector;
use itertools::Itertools;

pub struct Day16;

impl Solution for Day16 {
    fn part_one(&self, input: &str) -> String {
        let (grid, start, end) = Self::setup_grid(input);
        let dijkstra = Self::create_dijkstra(grid);
        let is_end = Self::is_end_closure(end);

        dijkstra.cost(vec![start], &is_end).unwrap().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let (grid, start, end) = Self::setup_grid(input);
        let dijkstra = Self::create_dijkstra(grid);
        let is_end = Self::is_end_closure(end);

        dijkstra
            .all_paths(vec![start], &is_end)
            .iter()
            .flat_map(|path| path.iter().map(|p| p.position()))
            .unique()
            .count()
            .to_string()
    }
}

impl Day16 {
    fn setup_grid(input: &str) -> (Grid<char>, Vector, Point) {
        let grid: Grid<char> = Grid::from(input);
        let start = grid.get_first_position(&'S').unwrap();
        let start = Vector::new(start, East);
        let end = grid.get_first_position(&'E').unwrap();

        (grid, start, end)
    }

    fn create_dijkstra(grid: Grid<char>) -> Dijkstra<Vector> {
        let adjacency = Self::adjacency_closure(grid);
        let cost = Self::cost_closure();

        Dijkstra::new(adjacency, cost)
    }

    fn adjacency_closure(grid: Grid<char>) -> Box<dyn Fn(Vector) -> Vec<Vector>> {
        Box::new(move |vector: Vector| {
            let vectors = vec![
                vector.forward(),
                vector.rotate_cw().forward(),
                vector.rotate_ccw().forward(),
            ];

            vectors
                .into_iter()
                .filter(|vec| {
                    grid.get_for_point(&vec.position())
                        .is_some_and(|element| ['.', 'E'].contains(element))
                })
                .collect_vec()
        })
    }

    fn cost_closure() -> Box<dyn Fn(Vector, Vector) -> usize> {
        Box::new(
            move |current: Vector, next: Vector| {
                if current.forward() == next {
                    1
                } else {
                    1001
                }
            },
        )
    }

    fn is_end_closure(end: Point) -> Box<dyn Fn(Vector) -> bool> {
        Box::new(move |vector: Vector| vector.position() == end)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day16::Day16;
    use crate::solutions::Solution;

    const FIRST_EXAMPLE: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

    #[test]
    fn part_one_example_1() {
        assert_eq!("7036", Day16.part_one(FIRST_EXAMPLE));
    }

    #[test]
    fn part_two_example_1() {
        assert_eq!("45", Day16.part_two(FIRST_EXAMPLE));
    }

    const SECOND_EXAMPLE: &str = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;

    #[test]
    fn part_one_example_2() {
        assert_eq!("11048", Day16.part_one(SECOND_EXAMPLE));
    }

    #[test]
    fn part_two_example_2() {
        assert_eq!("64", Day16.part_two(SECOND_EXAMPLE));
    }
}
