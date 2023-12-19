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

        println!("{:?} {:?}", y_range, x_range);

        loop {
            let next_moves: Vec<&Pipe> = current
                .position
                .adjacent()
                .iter()
                .filter(|p| {
                    if !p.in_ranges(x_range, y_range) {
                        return false;
                    }

                    if visited.contains(&&p) {
                        return false;
                    }

                    return true;
                })
                .into_iter()
                .map(|p| &pipes[p.y as usize][p.x as usize])
                .filter(|adjacent| !adjacent.tile.eq(&Tile::Ground))
                .collect();

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
    use crate::solutions::day10::Day10;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("10");

        assert_eq!("4", Day10.part_one(&input.as_str()));
    }

    #[test]
    fn part_one_example_3_test() {
        let input = read_example("10_3");

        assert_eq!("8", Day10.part_one(&input.as_str()));
    }
}
