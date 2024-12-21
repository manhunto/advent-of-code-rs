use crate::solutions::Solution;
use crate::utils::deltoid_surface::DeltoidSurface;
use crate::utils::grid::Grid;
use crate::utils::point::Point;
use itertools::Itertools;
use std::collections::HashMap;
use std::ops::RangeBounds;

pub struct Day20;

impl Solution for Day20 {
    fn part_one(&self, input: &str) -> String {
        self.part_one_cheats_in_range(input, 100..).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        // extract surface trait and contains method
        // grid has function get every position in area
        // rename surface range as rectangular area
        //
        self.part_two_cheats_in_range(input, 100..).to_string()
    }
}

impl Day20 {
    fn part_one_cheats_in_range(&self, input: &str, range: impl RangeBounds<usize>) -> usize {
        let cheat_range_from_current = |current: Point| {
            current
                .adjacent_vectors()
                .map(|v| v.forward().position())
                .into_iter()
        };

        self.cheats_in_range(input, range, &cheat_range_from_current)
    }

    fn part_two_cheats_in_range(&self, input: &str, range: impl RangeBounds<usize>) -> usize {
        let cheat_range_from_current =
            |current: Point| DeltoidSurface::new(current, 20).points().into_iter();

        self.cheats_in_range(input, range, &cheat_range_from_current)
    }

    fn cheats_in_range<R, I>(
        &self,
        input: &str,
        range: R,
        cheat_positions: &dyn Fn(Point) -> I,
    ) -> usize
    where
        R: RangeBounds<usize>,
        I: Iterator<Item = Point>,
    {
        let grid: Grid<char> = Grid::from(input);
        let start = grid.get_first_position(&'S').unwrap();
        let end = grid.get_first_position(&'E').unwrap();

        let path_without_cheats = self.get_path(start, end, &grid);

        path_without_cheats
            .path()
            .iter()
            .flat_map(|(current_time, current)| {
                cheat_positions(*current)
                    .filter(|v| {
                        grid.get_for_point(v)
                            .is_some_and(|element| ['.', 'E'].contains(element))
                    })
                    .filter_map(|cheat_position| {
                        let time_after_cheat = path_without_cheats
                            .picoseconds_from(cheat_position)
                            .unwrap();
                        let cheat_cost = current.manhattan_distance(&cheat_position) as usize;

                        if time_after_cheat > *current_time + cheat_cost {
                            let time = time_after_cheat - current_time - cheat_cost;
                            if range.contains(&time) {
                                return Some(time);
                            }

                            None
                        } else {
                            None
                        }
                    })
                    .collect_vec()
            })
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
    fn part_one_cheats_in_range() {
        assert_eq!(14, Day20.part_one_cheats_in_range(EXAMPLE, 2..=2));
        assert_eq!(14, Day20.part_one_cheats_in_range(EXAMPLE, 4..=4));
        assert_eq!(2, Day20.part_one_cheats_in_range(EXAMPLE, 6..=6));
        assert_eq!(4, Day20.part_one_cheats_in_range(EXAMPLE, 8..=8));
        assert_eq!(2, Day20.part_one_cheats_in_range(EXAMPLE, 10..=10));
        assert_eq!(3, Day20.part_one_cheats_in_range(EXAMPLE, 12..=12));
        assert_eq!(1, Day20.part_one_cheats_in_range(EXAMPLE, 20..=20));
        assert_eq!(1, Day20.part_one_cheats_in_range(EXAMPLE, 36..=36));
        assert_eq!(1, Day20.part_one_cheats_in_range(EXAMPLE, 38..=38));
        assert_eq!(1, Day20.part_one_cheats_in_range(EXAMPLE, 40..=40));
        assert_eq!(1, Day20.part_one_cheats_in_range(EXAMPLE, 64..=64));
    }

    #[test]
    fn part_two_cheats_in_range() {
        assert_eq!(32, Day20.part_two_cheats_in_range(EXAMPLE, 50..=50));
        assert_eq!(31, Day20.part_two_cheats_in_range(EXAMPLE, 52..=52));
        assert_eq!(29, Day20.part_two_cheats_in_range(EXAMPLE, 54..=54));
        assert_eq!(39, Day20.part_two_cheats_in_range(EXAMPLE, 56..=56));
        assert_eq!(25, Day20.part_two_cheats_in_range(EXAMPLE, 58..=58));
        assert_eq!(23, Day20.part_two_cheats_in_range(EXAMPLE, 60..=60));
        assert_eq!(20, Day20.part_two_cheats_in_range(EXAMPLE, 62..=62));
    }
}
