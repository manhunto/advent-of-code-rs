use crate::solutions::Solution;
use crate::utils::direction::Direction;
use crate::utils::grid::Grid;
use crate::utils::point::Point;
use crate::utils::surface_range::SurfaceRange;
use crate::utils::vector::Vector;
use std::collections::HashSet;

pub struct Day15;

const BOX: char = 'O';
const OBSTACLE: char = '#';
const ROBOT: char = '@';

impl Solution for Day15 {
    fn part_one(&self, input: &str) -> String {
        let (grid, directions) = self.parse(input);
        let surface = grid.surface_range().shrink(1);

        let obstacles: HashSet<Point> = grid
            .elements_in_surface(OBSTACLE, surface)
            .into_iter()
            .collect();

        let mut boxes: HashSet<Point> = grid.get_all_positions(&BOX).into_iter().collect();
        let mut robot = grid.get_first_position(&ROBOT).unwrap();

        for direction in directions {
            let vector = Vector::new(robot, direction);

            if Self::can_move(vector, &mut boxes, &obstacles, &surface) {
                robot = vector.forward().position()
            }

            // let mut grid: Grid<char> = Grid::filled(grid.surface_range(), '.');
            // grid.modify_many(boxes.clone().into_iter().collect_vec(), BOX);
            // grid.modify_many(obstacles.clone().into_iter().collect_vec(), OBSTACLE);
            // grid.modify(robot, ROBOT);
            //
            // println!("{}", grid);

            // println!("Press Enter to continue...");
            // let mut input = String::new();
            // std::io::stdin().read_line(&mut input).expect("Failed to read line");
        }

        boxes
            .iter()
            .map(|b| 100 * b.y + b.x)
            .sum::<isize>()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from('0')
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
        vector: Vector,
        boxes: &mut HashSet<Point>,
        obstacles: &HashSet<Point>,
        surface: &SurfaceRange,
    ) -> bool {
        let next = vector.forward();
        let next_position = next.position();

        if obstacles.contains(&next_position) {
            return false;
        }

        if !surface.contains(next_position) {
            return false;
        }

        if boxes.contains(&next_position) {
            let box_ = Self::can_move(next, boxes, obstacles, surface);

            if box_ {
                boxes.remove(&next_position);
                boxes.insert(next.forward().position());

                return true;
            }

            return false;
        }

        true
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
}
