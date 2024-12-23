use crate::solutions::year2024::day21::Key::{Activate, Dir};
use crate::solutions::Solution;
use crate::utils::direction::Direction;
use crate::utils::graphs::dijkstra::Dijkstra;
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
                let path_len = self.path(line, &pads).chars().count();
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
    fn path(&self, code: &str, pads: &[Pad]) -> String {
        if pads.is_empty() {
            return code.to_string();
        }

        let code = "A".to_owned() + code;
        let pad = &pads[0];
        let pad_left = &pads[1..];

        code.chars()
            .tuple_windows()
            .map(|(from, to)| {
                self.all_shortest_paths_between_buttons(from, to, pad)
                    .iter()
                    .map(|path| self.path(path, pad_left))
                    .min_by_key(|path| path.chars().count())
                    .unwrap()
            })
            .collect()
    }

    fn all_shortest_paths_between_buttons(&self, from: char, to: char, pad: &Pad) -> Vec<String> {
        let adjacent = pad.adjacent.clone();

        let neighbours = Box::new(move |p: Point| adjacent.get(&p).unwrap().to_vec());
        let distance = Box::new(|_, _| 1);

        let dijkstra = Dijkstra::new(neighbours, distance);

        let start = pad.position(from as u8).unwrap();
        let end = pad.position(to as u8).unwrap();

        let is_end = |p: Point| p == end;
        let paths = dijkstra.all_paths(vec![start], &is_end);

        paths
            .iter()
            .map(|path| {
                let mut directions: Vec<Key> = path
                    .iter()
                    .collect_vec()
                    .windows(2)
                    .map(|pair| Dir(pair[0].direction(pair[1]).unwrap()))
                    .collect();
                directions.push(Activate);

                directions.iter().map(|d| d.to_string()).collect()
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
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day21::{Day21, Pad};
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#"029A
980A
179A
456A
379A"#;

    #[test]
    fn part_one_example() {
        assert_eq!("126384", Day21.part_one(EXAMPLE));
    }

    #[test]
    fn path_len() {
        let pads = vec![Pad::numeric(), Pad::key(), Pad::key()];

        assert_eq!(68, Day21.path("029A", &pads).len());
        assert_eq!(60, Day21.path("980A", &pads).len());
        assert_eq!(68, Day21.path("179A", &pads).len());
        assert_eq!(64, Day21.path("456A", &pads).len());
        assert_eq!(64, Day21.path("379A", &pads).len());
        assert_eq!(78, Day21.path("739A", &pads).len());
    }
}
