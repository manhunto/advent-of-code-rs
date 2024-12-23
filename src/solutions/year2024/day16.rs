use crate::solutions::Solution;
use crate::utils::direction::Direction::East;
use crate::utils::graphs::dijkstra::Dijkstra;
use crate::utils::grid::Grid;
use crate::utils::vector::Vector;
use itertools::Itertools;

pub struct Day16;

impl Solution for Day16 {
    fn part_one(&self, input: &str) -> String {
        let grid: Grid<char> = Grid::from(input);
        let start = grid.get_first_position(&'S').unwrap();
        let start = Vector::new(start, East);
        let end = grid.get_first_position(&'E').unwrap();

        let adjacency = |vector: Vector| {
            let vectors = vec![
                vector.forward(),
                vector.rotate_cw().forward(),
                vector.rotate_ccw().forward(),
            ];

            let result = vectors
                .into_iter()
                .filter(|vec| {
                    let element = grid.get_for_point(&vec.position());

                    element == Some(&'.') || element == Some(&'E')
                })
                .collect_vec();

            result
        };

        let cost = |current: Vector, next: Vector| {
            if current.forward() == next {
                return 1;
            }

            1001
        };

        let is_end = |vector: Vector| vector.position() == end;

        let dijkstra: Dijkstra<Vector> = Dijkstra::new(&adjacency, &cost, &is_end);

        dijkstra.cost(vec![start]).unwrap().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let grid: Grid<char> = Grid::from(input);
        let start = grid.get_first_position(&'S').unwrap();
        let start = Vector::new(start, East);
        let end = grid.get_first_position(&'E').unwrap();

        let adjacency = |vector: Vector| {
            let vectors = vec![
                vector.forward(),
                vector.rotate_cw().forward(),
                vector.rotate_ccw().forward(),
            ];

            let result = vectors
                .into_iter()
                .filter(|vec| {
                    let element = grid.get_for_point(&vec.position());

                    element == Some(&'.') || element == Some(&'E')
                })
                .collect_vec();

            result
        };

        let cost = |current: Vector, next: Vector| {
            if current.forward() == next {
                return 1;
            }

            1001
        };

        let is_end = |vector: Vector| vector.position() == end;
        let dijkstra: Dijkstra<Vector> = Dijkstra::new(&adjacency, &cost, &is_end);

        let paths = dijkstra.all_paths(vec![start]);

        paths
            .iter()
            .flat_map(|path| path.iter().map(|p| p.position()))
            .unique()
            .count()
            .to_string()
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
