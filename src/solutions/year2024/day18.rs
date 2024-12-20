use crate::solutions::Solution;
use crate::utils::graphs::a_star::AStar;
use crate::utils::point::Point;
use crate::utils::surface_range::SurfaceRange;
use itertools::Itertools;
use std::collections::HashSet;

const PUZZLE_GRID_SIZE: usize = 70;
const PUZZLE_MEMORY_SIZE: usize = 1024;

pub struct Day18 {
    surface: SurfaceRange,
    memory_size: usize,
}

impl Solution for Day18 {
    fn part_one(&self, input: &str) -> String {
        let byte_positions: HashSet<Point> = input
            .lines()
            .take(self.memory_size)
            .map(|l| l.parse().unwrap())
            .collect();

        let start = self.surface.top_left_corner();
        let end = self.surface.bottom_right_corner();

        let neighbours = |point: Point| -> Vec<Point> {
            point
                .adjacent()
                .into_iter()
                .filter(|adj| !byte_positions.contains(adj) && self.surface.contains(*adj))
                .collect_vec()
        };

        let distance = |from: Point, to: Point| from.manhattan_distance(&to) as usize;

        let a_start = AStar::new(&neighbours, &distance);

        (a_start.path(start, end).unwrap().len() - 1).to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from('0')
    }
}

impl Default for Day18 {
    fn default() -> Self {
        Self::new(PUZZLE_GRID_SIZE, PUZZLE_MEMORY_SIZE)
    }
}

impl Day18 {
    fn new(grid_size: usize, memory_size: usize) -> Self {
        Self {
            surface: SurfaceRange::from_points(0, grid_size as isize, 0, grid_size as isize),
            memory_size,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day18::Day18;
    use crate::solutions::Solution;

    const EXAMPLE_GRID_SIZE: usize = 6;
    const EXAMPLE_MEMORY_SIZE: usize = 12;

    const EXAMPLE: &str = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("22", solution().part_one(EXAMPLE));
    }

    fn solution() -> Day18 {
        Day18::new(EXAMPLE_GRID_SIZE, EXAMPLE_MEMORY_SIZE)
    }
}
