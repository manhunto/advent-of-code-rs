use crate::solutions::Solution;
use crate::utils::grid::Grid;
use crate::utils::point::Point;
use itertools::Itertools;
use std::collections::HashMap;
use std::ops::RangeBounds;

pub struct Day20;

impl Solution for Day20 {
    fn part_one(&self, input: &str) -> String {
        self.cheats_in_range(input, 100..).to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from('0')
    }
}

impl Day20 {
    fn cheats_in_range<R>(&self, input: &str, range: R) -> usize
    where
        R: RangeBounds<usize>,
    {
        let grid: Grid<char> = Grid::from(input);
        let start = grid.get_first_position(&'S').unwrap();
        let end = grid.get_first_position(&'E').unwrap();

        let path_without_cheats = self.get_path(start, end, &grid);

        path_without_cheats
            .path()
            .iter()
            .flat_map(|(current_time, current)| {
                current
                    .adjacent_vectors()
                    .iter()
                    .filter(|v| grid.is_for_point(&v.position(), '#'))
                    .map(|p| p.forward())
                    .filter(|v| {
                        grid.get_for_point(&v.position())
                            .is_some_and(|element| ['.', 'E'].contains(element))
                    })
                    .filter_map(|v| {
                        if let Some(time_after_cheat) =
                            path_without_cheats.picoseconds_from(v.position())
                        {
                            if time_after_cheat > *current_time {
                                return Some(time_after_cheat - current_time - 2);
                                // why -2
                            }
                        }

                        None
                    })
                    .collect_vec()
            })
            .filter(|time| range.contains(time))
            .count()
    }

    fn get_path(&self, start: Point, end: Point, grid: &Grid<char>) -> PathWithoutCheats {
        let mut current = start;
        let mut index: usize = 0;
        let mut path: HashMap<Point, usize> = HashMap::new();

        while current != end {
            let tmp = current.adjacent();
            let next = tmp
                .iter()
                .find(|p| {
                    !path.contains_key(p)
                        && grid
                            .get_for_point(p)
                            .is_some_and(|element| ['.', 'E'].contains(element))
                })
                .unwrap();

            index += 1;
            path.insert(current, index);
            current = *next;
        }

        index += 1;
        path.insert(current, index);

        PathWithoutCheats { path }
    }
}

struct PathWithoutCheats {
    path: HashMap<Point, usize>,
}

impl PathWithoutCheats {
    fn picoseconds_from(&self, point: Point) -> Option<usize> {
        self.path.get(&point).copied()
    }

    fn path(&self) -> Vec<(usize, Point)> {
        self.path
            .iter()
            .map(|(point, time)| (*time, *point))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day20::Day20;

    const EXAMPLE: &str = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;

    #[test]
    fn test_solve() {
        assert_eq!(14, Day20.cheats_in_range(EXAMPLE, 2..=2));
        assert_eq!(14, Day20.cheats_in_range(EXAMPLE, 4..=4));
        assert_eq!(2, Day20.cheats_in_range(EXAMPLE, 6..=6));
        assert_eq!(4, Day20.cheats_in_range(EXAMPLE, 8..=8));
        assert_eq!(2, Day20.cheats_in_range(EXAMPLE, 10..=10));
        assert_eq!(3, Day20.cheats_in_range(EXAMPLE, 12..=12));
        assert_eq!(1, Day20.cheats_in_range(EXAMPLE, 20..=20));
        assert_eq!(1, Day20.cheats_in_range(EXAMPLE, 36..=36));
        assert_eq!(1, Day20.cheats_in_range(EXAMPLE, 38..=38));
        assert_eq!(1, Day20.cheats_in_range(EXAMPLE, 40..=40));
        assert_eq!(1, Day20.cheats_in_range(EXAMPLE, 64..=64));
    }
}
