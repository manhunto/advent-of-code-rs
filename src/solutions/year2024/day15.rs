use crate::solutions::Solution;
use crate::utils::direction::Direction;
use crate::utils::grid::Grid;
use crate::utils::point::Point;
use itertools::Itertools;
use std::collections::HashSet;

pub struct Day15;

const BOX: char = 'O';
const OBSTACLE: char = '#';
const ROBOT: char = '@';

impl Solution for Day15 {
    fn part_one(&self, input: &str) -> String {
        let (grid, directions) = self.parse(input);

        let obstacles: HashSet<Point> = grid.get_all_positions(&OBSTACLE).into_iter().collect();

        let mut boxes: HashSet<Movable> = grid
            .get_all_positions(&BOX)
            .iter()
            .map(|p| Movable::new(vec![*p]))
            .collect();

        let mut robot = grid
            .get_first_position(&ROBOT)
            .map(|r| Movable::new(vec![r]))
            .unwrap();

        for direction in directions {
            if Self::can_move(&robot, direction, &mut boxes, &obstacles) {
                robot = robot.forward(direction);
            }
        }

        boxes.iter().map(|b| b.gps()).sum::<isize>().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let (grid, directions) = self.parse(input);

        let obstacles: HashSet<Point> = grid
            .get_all_positions(&OBSTACLE)
            .iter()
            .flat_map(|p| {
                let left = Point::new(p.x * 2, p.y);
                let right = left.east();

                vec![left, right]
            })
            .collect();

        let mut boxes: HashSet<Movable> = grid
            .get_all_positions(&BOX)
            .iter()
            .map(|p| {
                let left = Point::new(p.x * 2, p.y);
                let right = left.east();

                Movable::new(vec![left, right])
            })
            .collect();

        let mut robot = grid
            .get_first_position(&ROBOT)
            .map(|p| Movable::new(vec![Point::new(p.x * 2, p.y)]))
            .unwrap();

        for direction in directions {
            if Self::can_move(&robot, direction, &mut boxes, &obstacles) {
                robot = robot.forward(direction);
            }
        }

        boxes.iter().map(|b| b.gps()).sum::<isize>().to_string()
    }
}

impl Day15 {
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
        boxes: &mut HashSet<Movable>,
        obstacles: &HashSet<Point>,
    ) -> bool {
        let next = movable.forward(direction);
        if obstacles.iter().any(|o| next.collide(o)) {
            return false;
        }

        let boxes_collides = boxes
            .clone()
            .into_iter()
            .filter(|b| b != movable && b.collide_with(&next))
            .collect_vec();

        if !boxes_collides.is_empty() {
            let all_can_move = boxes_collides
                .iter()
                .all(|b| Self::can_move(b, direction, boxes, obstacles));

            if all_can_move {
                for boxes_collide in boxes_collides {
                    boxes.remove(&boxes_collide);
                    boxes.insert(boxes_collide.forward(direction));
                }

                return true;
            }

            return false;
        }

        true
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
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

    fn collide(&self, point: &Point) -> bool {
        self.points.contains(point)
    }

    fn collide_with(&self, movable: &Self) -> bool {
        self.points.iter().any(|p| movable.points.contains(p))
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
}
