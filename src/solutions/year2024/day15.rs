use crate::solutions::Solution;
use crate::utils::direction::Direction;
use crate::utils::grid::Grid;
use crate::utils::point::Point;
use crate::utils::range::Range;
use crate::utils::surface_range::SurfaceRange;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Day15;

const BOX: char = 'O';
const OBSTACLE: char = '#';
const ROBOT: char = '@';

impl Solution for Day15 {
    fn part_one(&self, input: &str) -> String {
        self.solve(input, 1)
    }

    fn part_two(&self, input: &str) -> String {
        self.solve(input, 2)
    }
}

impl Day15 {
    fn solve(&self, input: &str, scale: isize) -> String {
        let (grid, directions) = self.parse(input);

        let obstacles = self.create_scaled_obstacles(&grid, scale);
        let mut boxes = self.create_scaled_boxes(&grid, scale);
        let mut robot = self.create_scaled_robot(&grid, scale);

        for direction in directions {
            if Self::can_move(&robot, direction, &boxes, &obstacles) {
                robot = Self::move_(&robot, direction, &mut boxes);
            }
        }

        boxes
            .values()
            .unique()
            .map(|b| b.gps())
            .sum::<isize>()
            .to_string()
    }

    fn create_scaled_obstacles(&self, grid: &Grid<char>, scale: isize) -> HashSet<Point> {
        grid.get_all_positions(&OBSTACLE)
            .iter()
            .flat_map(|p| self.points_in_scale(p, scale, scale))
            .collect()
    }

    fn create_scaled_boxes(&self, grid: &Grid<char>, scale: isize) -> HashMap<Point, Movable> {
        grid.get_all_positions(&BOX)
            .iter()
            .flat_map(|p| {
                let offsets = self.points_in_scale(p, scale, scale);
                let movable = Movable::new(offsets.clone());

                offsets
                    .clone()
                    .into_iter()
                    .map(move |p| (p, movable.clone()))
            })
            .collect()
    }

    fn create_scaled_robot(&self, grid: &Grid<char>, scale: isize) -> Movable {
        grid.get_first_position(&ROBOT)
            .map(|p| {
                let offsets = self.points_in_scale(&p, scale, 1);

                Movable::new(offsets)
            })
            .unwrap()
    }

    fn points_in_scale(&self, point: &Point, scale: isize, size: isize) -> Vec<Point> {
        let base = Point::new(point.x * scale, point.y);

        (0..size)
            .map(|i| base.move_in_with_length(Direction::East, i))
            .collect()
    }

    fn parse(&self, input: &str) -> (Grid<char>, Vec<Direction>) {
        input
            .split_once("\n\n")
            .map(|(grid, directions)| {
                (
                    Grid::<char>::from(grid),
                    directions
                        .lines()
                        .flat_map(|s| {
                            s.chars().map(|c| match c {
                                '^' => Direction::North,
                                '>' => Direction::East,
                                '<' => Direction::West,
                                'v' => Direction::South,
                                _ => unreachable!("bad direction: {}", c),
                            })
                        })
                        .collect(),
                )
            })
            .unwrap()
    }

    fn can_move(
        movable: &Movable,
        direction: Direction,
        boxes: &HashMap<Point, Movable>,
        obstacles: &HashSet<Point>,
    ) -> bool {
        let next = movable.forward(direction);
        if next.collide_with_any(obstacles) {
            return false;
        }

        let boxes_collides: HashSet<&Movable> = next
            .points
            .iter()
            .filter_map(|p| boxes.get(p))
            .filter(|&b| b != movable)
            .collect();

        if !boxes_collides.is_empty() {
            let all_can_move = boxes_collides
                .iter()
                .all(|b| Self::can_move(b, direction, boxes, obstacles));

            return all_can_move;
        }

        true
    }

    fn move_(
        movable: &Movable,
        direction: Direction,
        boxes: &mut HashMap<Point, Movable>,
    ) -> Movable {
        let next = movable.forward(direction);

        let colliding_boxes: HashSet<Movable> = next
            .points
            .iter()
            .filter_map(|p| boxes.get(p))
            .filter(|&b| b != movable)
            .cloned()
            .collect();

        for b in colliding_boxes {
            let moved_box = Self::move_(&b, direction, boxes);

            for point in &b.points {
                boxes.remove(point);
            }

            for point in &moved_box.points {
                boxes.insert(*point, moved_box.clone());
            }
        }

        next
    }

    fn _print_grid(
        &self,
        grid_surface: &SurfaceRange,
        obstacles: &HashSet<Point>,
        boxes: &HashMap<Point, Movable>,
        robot: &Movable,
        dir: Option<Direction>,
    ) {
        let grid_surface = SurfaceRange::new(
            Range::new(grid_surface.x().start(), grid_surface.x().end() * 2 + 1).unwrap(),
            grid_surface.y(),
        );
        let mut grid_print: Grid<char> = Grid::filled(grid_surface, '.');
        grid_print.modify_many(obstacles.clone().into_iter().collect_vec(), OBSTACLE);

        let boxes: HashSet<Movable> = boxes.values().cloned().collect();

        for box_ in boxes {
            grid_print.modify(box_.points[0], '[');
            grid_print.modify(box_.points[1], ']');
        }

        grid_print.modify(robot.points[0], ROBOT);

        println!("Move: {:?}", dir);
        println!("{}", grid_print);
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Movable {
    points: Vec<Point>,
}

impl Movable {
    fn new(points: Vec<Point>) -> Self {
        if points.is_empty() || points.len() > 2 {
            unreachable!("Box should be located on one or two points");
        }

        Self { points }
    }

    fn gps(&self) -> isize {
        let x = self.points.iter().map(|p| p.x).min().unwrap();
        let y = self.points.first().unwrap().y;

        100 * y + x
    }

    fn forward(&self, direction: Direction) -> Self {
        let points = self.points.iter().map(|p| p.move_in(direction)).collect();

        Self { points }
    }

    fn collide_with_any(&self, points: &HashSet<Point>) -> bool {
        self.points.iter().any(|p| points.contains(p))
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day15::Day15;
    use crate::solutions::Solution;

    const SMALL_EXAMPLE: &str = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;

    #[test]
    fn part_one_small_example_test() {
        assert_eq!("2028", Day15.part_one(SMALL_EXAMPLE));
    }

    const BIG_EXAMPLE: &str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;

    #[test]
    fn part_one_big_example_test() {
        assert_eq!("10092", Day15.part_one(BIG_EXAMPLE));
    }

    #[test]
    fn part_two_big_example_test() {
        assert_eq!("9021", Day15.part_two(BIG_EXAMPLE));
    }

    #[test]
    fn part_two_reddit_case_2() {
        const INPUT: &str = r#"#######
#.....#
#.OO@.#
#.....#
#######

<<"#;

        assert_eq!("406", Day15.part_two(INPUT));
    }

    #[test]
    fn part_two_reddit_case_3() {
        const INPUT: &str = r#"#######
#.....#
#.O#..#
#..O@.#
#.....#
#######

<v<<^"#;

        assert_eq!("509", Day15.part_two(INPUT));
    }

    #[test]
    fn part_two_reddit_case_4() {
        const INPUT: &str = r#"#######
#.....#
#.#O..#
#..O@.#
#.....#
#######

<v<^"#;

        assert_eq!("511", Day15.part_two(INPUT));
    }

    #[test]
    fn part_two_reddit_case_5() {
        const INPUT: &str = r#"######
#....#
#.O..#
#.OO@#
#.O..#
#....#
######

<vv<<^"#;

        assert_eq!("816", Day15.part_two(INPUT));
    }

    #[test]
    fn part_two_reddit_case_6() {
        const INPUT: &str = r#"#######
#...#.#
#.....#
#.....#
#.....#
#.....#
#.OOO@#
#.OOO.#
#..O..#
#.....#
#.....#
#######

v<vv<<^^^^^"#;

        assert_eq!("2339", Day15.part_two(INPUT));
    }

    #[test]
    fn part_two_my_case() {
        const INPUT: &str = r#"######
#....#
#..@.#
#.OO.#
#....#
#....#
######

<v"#;

        assert_eq!("710", Day15.part_two(INPUT));
    }
}
