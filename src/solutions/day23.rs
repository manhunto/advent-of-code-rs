use crate::direction::Direction;
use crate::grid::Grid;
use crate::point::Point;
use crate::solutions::Solution;
use std::collections::VecDeque;

pub struct Day23;

impl Solution for Day23 {
    fn part_one(&self, input: &str) -> String {
        let grid: Grid<char> = Grid::from(input);
        let start = Point::new(1, 0);
        let surface = grid.surface_range();
        let end = surface.bottom_right_corner().west();

        let mut finished_elves: Vec<Elf> = Vec::new();

        let mut elves: VecDeque<Elf> = VecDeque::new();
        elves.push_back(Elf::new(start));

        while let Some(elf) = elves.pop_front() {
            if elf.position == end {
                finished_elves.push(elf);
                continue;
            }

            let adjacent = elf.position.adjacent_vectors();
            for next in adjacent {
                if !surface.contains(next.position()) {
                    continue;
                }

                let tile = grid.get_for_point(&next.position()).unwrap();
                if tile == &'#' {
                    continue;
                }

                if elf.visited(&next.position()) {
                    continue;
                }

                match tile {
                    '.' | 'F' => elves.push_back(elf.step(next.position())),
                    '^' => {
                        if next.facing() == Direction::North {
                            elves.push_back(elf.step(next.position()));
                        }
                    }
                    '<' => {
                        if next.facing() == Direction::West {
                            elves.push_back(elf.step(next.position()));
                        }
                    }
                    '>' => {
                        if next.facing() == Direction::East {
                            elves.push_back(elf.step(next.position()));
                        }
                    }
                    'v' => {
                        if next.facing() == Direction::South {
                            elves.push_back(elf.step(next.position()));
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }

        finished_elves
            .iter()
            .map(|e| e.steps())
            .max()
            .unwrap()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from('0')
    }
}

struct Elf {
    position: Point,
    path: Vec<Point>,
}

impl Elf {
    fn new(position: Point) -> Self {
        Self {
            position,
            path: vec![position],
        }
    }

    fn visited(&self, position: &Point) -> bool {
        self.path.contains(position)
    }

    fn step(&self, position: Point) -> Self {
        let mut path = self.path.clone();
        path.push(position);

        Self { position, path }
    }

    fn steps(&self) -> usize {
        self.path.len() - 1
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day23::Day23;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("23");

        assert_eq!("94", Day23.part_one(input.as_str()));
    }
}
