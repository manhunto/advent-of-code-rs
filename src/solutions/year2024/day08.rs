use crate::solutions::Solution;
use crate::utils::grid::Grid;
use crate::utils::point::Point;
use itertools::Itertools;

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
                    .flat_map(|pair| self.antinodes(*pair[0], *pair[1]))
                    .collect::<Vec<Point>>()
            })
            .filter(|p| surface_range.contains(*p))
            .unique()
            .count()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from('0')
    }
}

impl Day08 {
    fn antinodes(&self, p1: Point, p2: Point) -> Vec<Point> {
        let diff = p1 - p2;

        vec![p1 + diff, p1 - diff, p2 + diff, p2 - diff]
            .into_iter()
            .filter(|p| *p != p1 && *p != p2)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day08::Day08;
    use crate::solutions::Solution;
    use crate::utils::grid::Grid;
    use itertools::Itertools;

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
    fn antinodes() {
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

        let mut result = Day08.antinodes(*p1, *p2);
        let mut expected = elements.get(&'#').unwrap().to_vec();

        result.sort();
        expected.sort();

        assert_eq!(expected, result);
    }
}
