use crate::solutions::year2016::day08::Instruction::{Rect, RotateColumn, RotateRow};
use crate::solutions::Solution;
use crate::utils::direction::Direction;
use crate::utils::grid::Grid;
use crate::utils::point::Point;
use crate::utils::surface_range::SurfaceRange;
use std::str::FromStr;

pub struct Day08 {
    width: isize,
    height: isize,
}

impl Solution for Day08 {
    fn part_one(&self, input: &str) -> String {
        self.apply_on_screen(input).pixels_lit().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let screen = self.apply_on_screen(input);
        let grid = Grid::from(screen);

        println!("{}", grid);

        String::from("0")
    }
}

impl Day08 {
    fn parse<'a>(&self, input: &'a str) -> impl Iterator<Item = Instruction> + 'a {
        input.lines().map(|map| map.parse().unwrap())
    }

    fn apply_on_screen(&self, input: &str) -> Screen {
        let mut screen = Screen::new(self.width, self.height);

        self.parse(input)
            .for_each(|instruction| screen.apply(instruction));

        screen
    }
}

impl Default for Day08 {
    fn default() -> Self {
        Self {
            width: 50,
            height: 6,
        }
    }
}

#[derive(Clone)]
struct Screen {
    pixels: Vec<Point>,
    width: isize,
    height: isize,
}

impl Screen {
    fn new(width: isize, height: isize) -> Self {
        Self {
            pixels: Vec::new(),
            width,
            height,
        }
    }

    fn apply(&mut self, instruction: Instruction) {
        match instruction {
            Rect { width, height } => {
                for y in 0..height {
                    for x in 0..width {
                        self.pixels.push(Point::new(x, y));
                    }
                }
            }
            RotateRow { row, by } => self.pixels.iter_mut().filter(|p| p.y == row).for_each(|p| {
                let new = (p.x + by) % self.width;
                let diff = new - p.x;

                *p = p.move_in_with_length(Direction::East, diff);
            }),
            RotateColumn { col, by } => {
                self.pixels.iter_mut().filter(|p| p.x == col).for_each(|p| {
                    let new = (p.y + by) % self.height;
                    let diff = new - p.y;

                    *p = p.move_in_with_length(Direction::South, diff);
                })
            }
        }
    }

    fn pixels_lit(&self) -> usize {
        self.pixels.len()
    }
}

#[derive(Debug)]
enum Instruction {
    Rect { width: isize, height: isize },
    RotateRow { row: isize, by: isize },
    RotateColumn { col: isize, by: isize },
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();

        if parts[0] == "rect" {
            // rect 3x2
            let (width, height) = parts[1].split_once('x').unwrap();

            Ok(Rect {
                width: width.parse().unwrap(),
                height: height.parse().unwrap(),
            })
        } else if parts[0] == "rotate" && parts[1] == "row" {
            // rotate row y=0 by 4
            let row = parts[2][2..].parse().unwrap();
            let by = parts[4].parse().unwrap();

            Ok(RotateRow { row, by })
        } else if parts[0] == "rotate" && parts[1] == "column" {
            // rotate column x=1 by 1
            let col = parts[2][2..].parse().unwrap();
            let by = parts[4].parse().unwrap();

            Ok(RotateColumn { col, by })
        } else {
            Err(())
        }
    }
}

impl From<Screen> for Grid<char> {
    fn from(value: Screen) -> Self {
        let mut grid = Grid::filled(SurfaceRange::rectangle(value.width, value.height), '.');

        for pixel in value.pixels {
            grid.modify(pixel, '#');
        }

        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::grid::Grid;

    #[test]
    fn part_one_example() {
        let mut screen = Screen::new(7, 3);

        // First instruction
        let instruction = Instruction::from_str("rect 3x2").unwrap();
        screen.apply(instruction);

        let expected = r#"###....
###....
.......
"#;

        assert_eq!(expected, Grid::from(screen.clone()).to_string());

        // Second instruction
        let instruction = Instruction::from_str("rotate column x=1 by 1").unwrap();
        screen.apply(instruction);

        let expected = r#"#.#....
###....
.#.....
"#;

        assert_eq!(expected, Grid::from(screen.clone()).to_string());

        // Third instruction
        let instruction = Instruction::from_str("rotate row y=0 by 4").unwrap();
        screen.apply(instruction);

        let expected = r#"....#.#
###....
.#.....
"#;

        assert_eq!(expected, Grid::from(screen.clone()).to_string());

        // Fourth instruction
        let instruction = Instruction::from_str("rotate column x=1 by 1").unwrap();
        screen.apply(instruction);

        let expected = r#".#..#.#
#.#....
.#.....
"#;

        assert_eq!(expected, Grid::from(screen.clone()).to_string());
    }
}
