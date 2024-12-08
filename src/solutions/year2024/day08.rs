use crate::solutions::Solution;
use crate::utils::grid::Grid;
use crate::utils::point::Point;
use crate::utils::surface_range::SurfaceRange;
use itertools::{concat, Itertools};

pub struct Day08;

impl Solution for Day08 {
    fn part_one(&self, input: &str) -> String {
        let grid: Grid<char> = Grid::from(input);
        let surface_range = grid.surface_range();

        grid.elements_with_points()
            .iter()
            .filter(|(element, _)| **element != '.')
            .flat_map(|(_, points)| {
                points
                    .iter()
                    .combinations(2)
                    .flat_map(|pair| self.antinodes_part_one(*pair[0], *pair[1], &surface_range))
                    .collect::<Vec<Point>>()
            })
            .unique()
            .count()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let grid: Grid<char> = Grid::from(input);
        let surface_range = grid.surface_range();

        grid.elements_with_points()
            .iter()
            .filter(|(element, _)| **element != '.')
            .flat_map(|(_, points)| {
                points
                    .iter()
                    .combinations(2)
                    .flat_map(|pair| self.antinodes_part_two(*pair[0], *pair[1], &surface_range))
                    .collect::<Vec<Point>>()
            })
            .unique()
            .count()
            .to_string()
    }
}

impl Day08 {
    fn antinodes_part_one(&self, p1: Point, p2: Point, surface_range: &SurfaceRange) -> Vec<Point> {
        let diff = p1 - p2;

        vec![p1 + diff, p1 - diff, p2 + diff, p2 - diff]
            .into_iter()
            .filter(|p| *p != p1 && *p != p2)
            .filter(|p| surface_range.contains(*p))
            .collect()
    }

    fn antinodes_part_two(&self, p1: Point, p2: Point, surface_range: &SurfaceRange) -> Vec<Point> {
        let diff = p1 - p2;

        let first = self.antipodes_in_dir(p1, diff, surface_range);
        let second = self.antipodes_in_dir(p2, -diff, surface_range);

        let vec = concat(vec![first, second]);

        vec.into_iter().unique().collect()
    }

    fn antipodes_in_dir(
        &self,
        point: Point,
        diff: Point,
        surface_range: &SurfaceRange,
    ) -> Vec<Point> {
        let mut vec = Vec::new();
        let mut current = point;

        while surface_range.contains(current) {
            vec.push(current);

            current = current + diff;
        }

        vec
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day08::Day08;
    use crate::solutions::Solution;
    use crate::utils::grid::Grid;
    use itertools::{concat, Itertools};

    const EXAMPLE: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("14", Day08.part_one(EXAMPLE));
    }

    #[test]
    fn part_two_example_test() {
        assert_eq!("34", Day08.part_two(EXAMPLE));
    }

    #[test]
    fn antinodes_part_one() {
        const GRID: &str = r#"..........
...#......
..........
....a.....
..........
.....a....
..........
......#...
..........
.........."#;

        let grid: Grid<char> = Grid::from(GRID);
        let elements = grid.elements_with_points();

        let (p1, p2) = elements.get(&'a').unwrap().iter().collect_tuple().unwrap();

        let mut result = Day08.antinodes_part_one(*p1, *p2, &grid.surface_range());
        let mut expected = elements.get(&'#').unwrap().to_vec();

        result.sort();
        expected.sort();

        assert_eq!(expected, result);
    }

    #[test]
    fn part_two_second_example() {
        const GRID: &str = r#"T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
.........."#;

        let grid: Grid<char> = Grid::from(GRID);
        let elements = grid.elements_with_points();

        let (p1, p2, p3) = elements.get(&'T').unwrap().iter().collect_tuple().unwrap();

        let result1 = Day08.antinodes_part_two(*p1, *p2, &grid.surface_range());
        let result2 = Day08.antinodes_part_two(*p1, *p3, &grid.surface_range());
        let result3 = Day08.antinodes_part_two(*p2, *p3, &grid.surface_range());

        let result = concat(vec![result1, result2, result3])
            .iter()
            .unique()
            .count();

        assert_eq!(9, result);
    }
}
