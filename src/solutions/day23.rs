use crate::direction::Direction;
use crate::grid::Grid;
use crate::point::Point;
use crate::solutions::Solution;
use crate::utils::vector::Vector;
use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

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
        elves.push_back(Elf::new(start, Direction::South));

        let mut crossroads: HashSet<Vector> = HashSet::new();
        crossroads.insert(Vector::new(start, Direction::South));
        crossroads.insert(Vector::new(end, Direction::South));

        let mut between_crossroads: HashMap<Vector, BinaryHeap<Vector>> = HashMap::new();

        while let Some(mut current_elf) = elves.pop_front() {
            let mut current_elf_has_moves = true;

            while current_elf_has_moves {
                if current_elf.position == end {
                    finished_elves.push(current_elf.clone());

                    let sorted_vec = current_elf.path.clone().into_sorted_vec();

                    let all_visited_crossroads: Vec<usize> = sorted_vec
                        .clone()
                        .into_iter()
                        .filter(|c| crossroads.contains(c))
                        .map(|c| sorted_vec.binary_search(&c).unwrap())
                        .collect();

                    for from_to in all_visited_crossroads.windows(2) {
                        let slice = &sorted_vec[from_to[0]..from_to[1]];

                        between_crossroads
                            .insert(*slice.first().unwrap(), BinaryHeap::from(slice.to_vec()));
                    }

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

                if available_moves.len() > 1 {
                    for available_move in &available_moves {
                        crossroads
                            .insert(Vector::new(current_elf.position, available_move.facing()));
                    }
                }

                if current_elf_has_moves {
                    let next_move = available_moves.pop().unwrap();
                    let new_current: Elf;

                    if let Some(path) = between_crossroads.get(&next_move) {
                        new_current = current_elf.step_forward(path.clone());
                    } else {
                        new_current = current_elf.step(next_move);
                    }

                    if !available_moves.is_empty() {
                        for next in available_moves {
                            elves.push_back(current_elf.step(next));
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
    visited: Vec<Point>,
    path: BinaryHeap<Vector>,
}

impl Elf {
    fn new(position: Point, direction: Direction) -> Self {
        Self {
            position,
            visited: vec![position],
            path: BinaryHeap::from(vec![Vector::new(position, direction)]),
        }
    }

    fn visited(&self, position: &Point) -> bool {
        self.visited.iter().contains(position)
    }

    fn step(&self, vector: Vector) -> Self {
        let mut visited = self.visited.clone();
        visited.push(vector.position());

        let mut path = self.path.clone();
        path.push(vector);

        Self {
            position: vector.position(),
            visited,
            path,
        }
    }

    fn step_forward(&self, path: BinaryHeap<Vector>) -> Self {
        let mut new: Elf = self.clone();

        for step in path.into_sorted_vec() {
            new = new.step(step);
        }

        new
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
