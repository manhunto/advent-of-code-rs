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

            // let mut grid = grid.clone();
            // grid.modify(vector.position(), 'S');
            // for re in &result {
            //     grid.modify(re.position(), 'O')
            // }
            //
            // println!("{} - {}", vector, result.len());
            // println!("{}", grid);
            //

            result
        };

        let cost = |current: Vector, next: Vector| {
            if current.forward() == next {
                return 1;
            }

            if current.rotate_cw().forward() == next {
                return 1001;
            }

            if current.rotate_ccw().forward() == next {
                return 1001;
            }

            unreachable!();
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

        let start = Reindeer::new(start);

        let adjacency = |reindeer: Reindeer| {
            let possible_moves = reindeer.possible_moves();

            let result = possible_moves
                .into_iter()
                .filter(|reindeer| {
                    let element = grid.get_for_point(&reindeer.vector.position());

                    element == Some(&'.') || element == Some(&'E')
                })
                .collect_vec();

            result
        };

        let cost = |current: Reindeer, next: Reindeer| {
            if current.vector.forward() == next.vector {
                return 1;
            }

            if current.vector.rotate_cw().forward() == next.vector {
                return 1001;
            }

            if current.vector.rotate_ccw().forward() == next.vector {
                return 1001;
            }

            unreachable!();
        };

        let is_end = |reindeer: Reindeer| reindeer.vector.position() == end;

        let dijkstra: Dijkstra<Reindeer> = Dijkstra::new(&adjacency, &cost, &is_end);

        let paths = dijkstra.all_possible_paths(vec![start]);
        let min = paths.iter().map(|(c, _)| c).min().unwrap();
        let best_spots = paths
            .iter()
            .filter_map(|(c, r)| if c == min { Some(r) } else { None })
            .flat_map(|r| r.path.clone())
            .unique()
            .count();

        best_spots.to_string()
    }
}

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Clone)]
struct Reindeer {
    vector: Vector,
    path: Vec<Point>,
}
impl Reindeer {
    fn possible_moves(&self) -> Vec<Self> {
        let vectors = [
            self.vector.forward(),
            self.vector.rotate_cw().forward(),
            self.vector.rotate_ccw().forward(),
        ];

        vectors
            .iter()
            .filter(|v| !self.path.contains(&v.position()))
            .map(|vec| self.next(*vec))
            .collect()
    }

    fn next(&self, vector: Vector) -> Self {
        let mut path = self.path.clone();
        path.push(vector.position());

        Self { vector, path }
    }
}

impl Reindeer {
    fn new(vector: Vector) -> Self {
        Self {
            vector,
            path: vec![vector.position()],
        }
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
