use crate::direction::Direction;
use crate::grid::Grid;
use crate::point::Point;
use crate::solutions::Solution;
use crate::utils::vector::Vector;
use std::collections::VecDeque;

pub struct Day23;

impl Solution for Day23 {
    fn part_one(&self, input: &str) -> String {
        let slopes = |tile: char, next: Vector| -> bool {
            match tile {
                '.' | 'F' => true,
                '^' => next.facing() == Direction::North,
                '<' => next.facing() == Direction::West,
                '>' => next.facing() == Direction::East,
                'v' => next.facing() == Direction::South,
                _ => unreachable!(),
            }
        };

        Self::solve(input, slopes)
    }

    fn part_two(&self, input: &str) -> String {
        let slopes = |tile: char, _| -> bool {
            match tile {
                '.' | 'F' | '^' | '<' | '>' | 'v' => true,
                _ => unreachable!(),
            }
        };

        Self::solve(input, slopes)
    }
}

impl Day23 {
    fn solve(input: &str, slopes: fn(tile: char, next: Vector) -> bool) -> String {
        let grid: Grid<char> = Grid::from(input);
        let surface = grid.surface_range();
        let start = surface.top_left_corner().east();
        let end = surface.bottom_right_corner().west();

        let mut finished_elves: Vec<Elf> = Vec::new();

        let mut elves: VecDeque<Elf> = VecDeque::new();
        elves.push_back(Elf::new(start));

        while let Some(mut current_elf) = elves.pop_front() {
            let mut current_elf_has_moves = true;

            while current_elf_has_moves {
                if current_elf.position == end {
                    finished_elves.push(current_elf);
                    break;
                }

                let mut available_moves: Vec<Vector> = current_elf
                    .position
                    .adjacent_vectors()
                    .into_iter()
                    .filter(|a| {
                        if let Some(tile) = grid.get_for_point(&a.position()) {
                            return tile != &'#'
                                && !current_elf.visited(&a.position())
                                && slopes(*tile, *a);
                        }

                        false
                    })
                    .collect();

                current_elf_has_moves = !available_moves.is_empty();

                if current_elf_has_moves {
                    let new_current = current_elf.step(available_moves.pop().unwrap().position());

                    if !available_moves.is_empty() {
                        for next in available_moves {
                            elves.push_back(current_elf.step(next.position()));
                        }
                    }

                    current_elf = new_current;
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
}

#[derive(Clone)]
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

    #[test]
    fn part_two_example_test() {
        let input = read_example("23");

        assert_eq!("154", Day23.part_two(input.as_str()));
    }
}
