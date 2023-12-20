use std::fmt;
use std::fmt::Display;
use Direction::{East, South, West};
use crate::direction::Direction;
use crate::direction::Direction::North;
use crate::point::Point;
use crate::range::Range;
use crate::solutions::Solution;

pub struct Day10;

impl Solution for Day10 {
    fn part_one(&self, input: &str) -> String {
        let pipes: Vec<Vec<Pipe>> = self.parse_input(&input);
        let mut current = self.get_start_pipe(&pipes).expect("No start point");
        let mut visited: Vec<&Point> = vec![&current.position];
        let y_range = Range::new(0, (pipes.len() as i64) - 1).unwrap();

        let first_row = pipes.first().unwrap();
        let x_range = Range::new(0, (first_row.len() as i64) - 1).unwrap();


        loop {
            println!("[Current] {} {:?}", current.tile, current);

            let next_moves: Vec<&Pipe> = current
                .position
                .adjacent()
                .iter()
                .filter(|p| {
                    p.in_ranges(x_range, y_range) && !visited.contains(&&p)
                })
                .map(|p| &pipes[p.y as usize][p.x as usize])
                .filter(|p| {
                    let vec1 = p.position.adjacent_in_directions(p.tile.directions());

                    println!("Filter: {:?}. {}", vec1, vec1.contains(&current.position));

                    vec1.contains(&current.position)
                })
                .filter(|adjacent| !adjacent.tile.eq(&Tile::Ground))
                .collect();

            println!("{:?}", next_moves);

            if visited.len() > 1 && next_moves.is_empty() {
                break;
            }


            let next_move = *next_moves.clone().first().expect("No next move");

            current = next_move;
            visited.push(&current.position);
        }

        (visited.len() / 2).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        String::from("0")
    }
}

impl Day10 {
    fn parse_input(&self, input: &str) -> Vec<Vec<Pipe>> {
        input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line
                    .chars()
                    .enumerate()
                    .map(|(x, c)| Pipe::from_primitives(c, x as i32, y as i32))
                    .collect()
            })
            .collect()
    }

    fn get_start_pipe<'a>(&'a self, pipes: &'a Vec<Vec<Pipe>>) -> Option<&Pipe> {
        for pipe in pipes {
            for p in pipe {
                if p.tile == Tile::Start {
                    return Some(&p);
                }
            }
        }

        return None;
    }
}

#[derive(Debug, PartialEq)]
#[derive(Clone)]
enum Tile {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl Tile {
    fn from(char: char) -> Self {
        match char {
            '|' => Self::NS,
            '-' => Self::EW,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            'S' => Self::Start,
            '.' => Self::Ground,
            _ => panic!("{}", format!("Unknown tile: {}", char))
        }
    }

    fn directions(&self) -> Vec<Direction> {
        match self {
            Tile::NS => { vec![North, South] }
            Tile::EW => { vec![East, West] }
            Tile::NE => { vec![North, East] }
            Tile::NW => { vec![North, West] }
            Tile::SW => { vec![South, West] }
            Tile::SE => { vec![South, East] }
            Tile::Ground => { vec![] }
            Tile::Start => { vec![] }
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let char: char = match self {
            Self::NS => '|',
            Self::EW => '-',
            Self::NE => 'L',
            Self::NW => 'J',
            Self::SW => '7',
            Self::SE => 'F',
            Self::Start => 'S',
            Self::Ground => '.',
        };

        write!(f, "{}", char)
    }
}

#[derive(Debug, Clone)]
struct Pipe {
    tile: Tile,
    position: Point,
}

impl Pipe {
    fn new(tile: Tile, position: Point) -> Self {
        Self { tile, position }
    }

    fn from_primitives(char: char, x: i32, y: i32) -> Self {
        Self::new(Tile::from(char), Point::new(x, y))
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::point::Point;
    use crate::solutions::day10::{Day10, Tile};
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("10");

        assert_eq!("4", Day10.part_one(&input.as_str()));
    }

    #[test]
    fn part_one_example_2_test() {
        let input = read_example("10_2");

        assert_eq!("4", Day10.part_one(&input.as_str()));
    }

    #[test]
    fn part_one_example_3_test() {
        let input = read_example("10_3");

        assert_eq!("8", Day10.part_one(&input.as_str()));
    }

    #[test]
    fn tile_in_direction() {
        let point = Point::new(1, 1);

        assert_eq!(vec![Point::new(1, 0), Point::new(1, 2)], point.adjacent_in_directions(Tile::from('|').directions()));
        assert_eq!(vec![Point::new(2, 1), Point::new(0, 1)], point.adjacent_in_directions(Tile::from('-').directions()));
        assert_eq!(vec![Point::new(1, 0), Point::new(2, 1)], point.adjacent_in_directions(Tile::from('L').directions()));
        assert_eq!(vec![Point::new(1, 0), Point::new(0, 1)], point.adjacent_in_directions(Tile::from('J').directions()));
        assert_eq!(vec![Point::new(0, 1), Point::new(1, 2)], point.adjacent_in_directions(Tile::from('7').directions()));
        assert_eq!(vec![Point::new(2, 1), Point::new(1, 2)], point.adjacent_in_directions(Tile::from('F').directions()));
    }
}
