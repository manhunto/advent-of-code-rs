use crate::direction::Direction;
use crate::grid::Grid;
use crate::point::Point;
use crate::solutions::Solution;
use crate::utils::vector::Vector;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Day23;

impl Solution for Day23 {
    fn part_one(&self, input: &str) -> String {
        let can_move = |tile: char, next: Vector| -> bool {
            match tile {
                '.' => true,
                '^' => next.facing() == Direction::North,
                '<' => next.facing() == Direction::West,
                '>' => next.facing() == Direction::East,
                'v' => next.facing() == Direction::South,
                _ => unreachable!(),
            }
        };

        Self::solve(input, can_move)
    }

    fn part_two(&self, input: &str) -> String {
        let can_move = |tile: char, _| -> bool {
            match tile {
                '.' | '^' | '<' | '>' | 'v' => true,
                _ => unreachable!(),
            }
        };

        Self::solve(input, can_move)
    }
}

impl Day23 {
    fn solve(input: &str, can_move: fn(tile: char, next: Vector) -> bool) -> String {
        let grid: Grid<char> = Grid::from(input);
        let surface = grid.surface_range();
        let start = surface.top_left_corner().east();
        let end = surface.bottom_right_corner().west();

        let mut finished_elves: Vec<Elf> = Vec::new();

        let mut elves: VecDeque<Elf> = VecDeque::new();
        elves.push_back(Elf::new(start, Direction::South));

        let mut crossroads: HashSet<Vector> = HashSet::new();
        crossroads.insert(Vector::new(start, Direction::South));
        crossroads.insert(Vector::new(end, Direction::South));

        let mut between_crossroads: HashMap<Vector, VecDeque<Vector>> = HashMap::new();

        while let Some(mut current_elf) = elves.pop_front() {
            let mut current_elf_has_moves = true;

            while current_elf_has_moves {
                if current_elf.position == end {
                    finished_elves.push(current_elf.clone());
                    break;
                }

                let available_moves: Vec<Vector> = current_elf
                    .position
                    .adjacent_vectors()
                    .into_iter()
                    .filter(|a| {
                        if let Some(tile) = grid.get_for_point(&a.position()) {
                            return tile != &'#'
                                && !current_elf.visited(&a.position())
                                && can_move(*tile, *a);
                        }

                        false
                    })
                    .collect();

                if available_moves.len() > 1 {
                    for available_move in available_moves.clone() {
                        crossroads.insert(available_move);
                    }
                }

                current_elf_has_moves = !available_moves.is_empty();

                if current_elf_has_moves {
                    let mut iter = available_moves.iter();

                    let first_move = iter.next().unwrap();
                    let new_current = match between_crossroads.get(first_move) {
                        None => current_elf.step(*first_move),
                        Some(path) => current_elf.step_forward(path),
                    };

                    for next in iter {
                        let new_elv = match between_crossroads.get(next) {
                            None => current_elf.step(*next),
                            Some(path) => current_elf.step_forward(path),
                        };

                        elves.push_back(new_elv);
                    }

                    current_elf = new_current;
                } else {
                    Self::snapshot(current_elf.clone(), &crossroads, &mut between_crossroads);
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

    fn snapshot(
        elf: Elf,
        crossroads: &HashSet<Vector>,
        between_crossroads: &mut HashMap<Vector, VecDeque<Vector>>,
    ) {
        let sorted_vec = elf.path.clone();

        let all_visited_crossroads: Vec<usize> = sorted_vec
            .clone()
            .into_iter()
            .enumerate()
            .filter_map(|(key, c)| {
                if !crossroads.contains(&c) {
                    return None;
                }

                Some(key)
            })
            .collect();

        for range in all_visited_crossroads.windows(2) {
            let from = range[0];
            let to = range[1];
            if between_crossroads.contains_key(sorted_vec.get(from).unwrap()) {
                continue;
            }

            let mut queue: VecDeque<Vector> = VecDeque::new();
            for i in from..to {
                queue.push_back(*sorted_vec.get(i).unwrap());
            }

            between_crossroads.insert(*queue.front().unwrap(), queue);
        }
    }
}

#[derive(Clone)]
struct Elf {
    position: Point,
    visited: Vec<Point>,
    path: VecDeque<Vector>,
}

impl Elf {
    fn new(position: Point, direction: Direction) -> Self {
        Self {
            position,
            visited: vec![position],
            path: VecDeque::from([Vector::new(position, direction)]),
        }
    }

    fn visited(&self, position: &Point) -> bool {
        self.visited.iter().contains(position)
    }

    fn step(&self, vector: Vector) -> Self {
        let mut path = self.path.clone();
        path.push_back(vector);

        let mut visited = self.visited.clone();
        visited.push(vector.position());

        Self {
            position: vector.position(),
            visited,
            path,
        }
    }

    fn step_forward(&self, new_path: &VecDeque<Vector>) -> Self {
        let mut visited = self.visited.clone();
        let mut path = self.path.clone();

        let mut position = self.position;

        for step in new_path {
            position = step.position();
            visited.push(position);
            path.push_back(*step);
        }

        Self {
            position,
            visited,
            path,
        }
    }

    fn steps(&self) -> usize {
        self.visited.len() - 1
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
