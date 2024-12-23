use crate::solutions::year2024::day21::Key::{Activate, Dir};
use crate::solutions::Solution;
use crate::utils::direction::Direction;
use crate::utils::graphs::a_star::AStarBuilder;
use crate::utils::grid::Grid;
use crate::utils::point::Point;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

type Positions = HashMap<u8, Point>;
type Adjacent = HashMap<Point, Vec<Point>>;

const NUM_PAD: &str = r#"789
456
123
.0A"#;
const NUM_PAD_ELEMENTS: [u8; 11] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A',
];
const KEY_PAD: &str = r#".^A
<v>"#;
const KEY_PAD_ELEMENTS: [u8; 5] = [b'^', b'v', b'<', b'>', b'A'];

pub struct Day21;

impl Solution for Day21 {
    fn part_one(&self, input: &str) -> String {
        let pads = vec![Pad::numeric(), Pad::key(), Pad::key()];

        input
            .lines()
            .map(|line| {
                let path_len = self.path_len(line, &pads);
                let num: usize = line.trim_end_matches('A').parse().unwrap();

                num * path_len
            })
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from('0')
    }
}

impl Day21 {
    fn path_len(&self, code: &str, pads: &Vec<Pad>) -> usize {
        let mut current = code.to_string();

        for pad in pads {
            let path = self.path_for_str(&current, pad);

            current = path.iter().map(|key| key.to_string()).collect::<String>();

            // println!("{}", current);
        }

        current.chars().count()
    }

    fn path_for_str(&self, code: &str, pad: &Pad) -> Vec<Key> {
        let code = "A".to_owned() + code;

        let neighbours = |p: Point| pad.adjacent(&p);
        let distance = |_, _| 1;

        let a_star = AStarBuilder::init(&neighbours, &distance).build();

        code.chars()
            .tuple_windows()
            .flat_map(|(from, to)| {
                let path = a_star
                    .path(
                        pad.position(from as u8).unwrap(),
                        pad.position(to as u8).unwrap(),
                    )
                    .unwrap();
                let mut directions: Vec<Key> = path
                    .windows(2)
                    .map(|pair| Dir(pair[0].direction(&pair[1]).unwrap()))
                    .collect();
                directions.push(Activate);

                // println!("{from} {to} -> {:?}", directions.iter().map(|d| d.to_string()).collect::<String>());

                directions.into_iter()
            })
            .collect()
    }
}

#[derive(Debug, PartialEq)]
enum Key {
    Dir(Direction),
    Activate,
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Dir(d) => match d {
                Direction::North => "^",
                Direction::South => "v",
                Direction::East => ">",
                Direction::West => "<",
                _ => unreachable!(),
            },
            Activate => "A",
        };

        write!(f, "{}", v)
    }
}

struct Pad {
    positions: Positions,
    adjacent: Adjacent,
}

impl Pad {
    fn numeric() -> Self {
        let positions = Self::build_positions(NUM_PAD, &NUM_PAD_ELEMENTS);
        let adjacent = Self::build_adjacent(&positions);

        Self {
            positions,
            adjacent,
        }
    }

    fn key() -> Self {
        let positions = Self::build_positions(KEY_PAD, &KEY_PAD_ELEMENTS);
        let adjacent = Self::build_adjacent(&positions);

        Self {
            positions,
            adjacent,
        }
    }

    fn build_positions(map: &str, elements: &[u8]) -> Positions {
        let num_grid: Grid<u8> = Grid::from_custom(map, |c| c as u8);
        let mut num_pad_positions: Positions = HashMap::with_capacity(num_grid.surface().area());

        for i in elements {
            num_pad_positions.insert(*i, num_grid.get_first_position(i).unwrap());
        }

        num_pad_positions
    }

    fn build_adjacent(num_pad_map: &Positions) -> Adjacent {
        let mut adjacent_map = HashMap::with_capacity(num_pad_map.len());

        for pos in num_pad_map.values() {
            let adjacent: Vec<Point> = pos
                .adjacent()
                .iter()
                .filter_map(|p| num_pad_map.iter().find(|(_, v)| **v == *p))
                .map(|(_, p)| *p)
                .collect();

            adjacent_map.insert(*pos, adjacent);
        }

        adjacent_map
    }

    fn position(&self, element: u8) -> Option<Point> {
        self.positions.get(&element).copied()
    }

    fn adjacent(&self, position: &Point) -> Vec<Point> {
        self.adjacent.get(position).unwrap().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day21::Key::{Activate, Dir};
    use crate::solutions::year2024::day21::{Day21, Pad};
    use crate::solutions::Solution;
    use crate::utils::direction::Direction::{East, North, South, West};

    const EXAMPLE: &str = r#"029A
980A
179A
456A
379A"#;

    #[test]
    #[ignore]
    fn part_one_example() {
        assert_eq!("126384", Day21.part_one(EXAMPLE));
    }

    #[test]
    #[ignore]
    fn path_len() {
        let pads = vec![Pad::numeric(), Pad::key(), Pad::key()];

        assert_eq!(68, Day21.path_len("029A", &pads));
        assert_eq!(60, Day21.path_len("980A", &pads));
        assert_eq!(68, Day21.path_len("179A", &pads));
        assert_eq!(64, Day21.path_len("456A", &pads));
        assert_eq!(64, Day21.path_len("379A", &pads));
        assert_eq!(78, Day21.path_len("739A", &pads));
    }

    #[test]
    fn path_for_str() {
        let numeric = &Pad::numeric();
        let key = &Pad::key();

        assert_eq!(Day21.path_for_str("AA", numeric), vec![Activate, Activate]);
        assert_eq!(
            Day21.path_for_str("A1", numeric),
            vec![Activate, Dir(North), Dir(West), Dir(West), Activate]
        );
        assert_eq!(
            Day21.path_for_str("A4", numeric),
            vec![
                Activate,
                Dir(North),
                Dir(North),
                Dir(West),
                Dir(West),
                Activate
            ]
        );
        assert_eq!(
            Day21.path_for_str("A7", numeric),
            vec![
                Activate,
                Dir(North),
                Dir(North),
                Dir(North),
                Dir(West),
                Dir(West),
                Activate
            ]
        );
        assert_eq!(
            Day21.path_for_str("<A", key),
            vec![
                Dir(South),
                Dir(West),
                Dir(West),
                Activate,
                Dir(East),
                Dir(East),
                Dir(North),
                Activate
            ]
        );
    }
}
