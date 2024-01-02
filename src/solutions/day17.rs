use crate::point::Point;
use crate::direction::Direction;
use crate::grid::Grid;
use crate::solutions::Solution;
use crate::utils::pathfinding::dijkstra::{CostProvider, Dijkstra, IsAtEnd, Neighbours};
use crate::utils::vector::Vector;

pub struct Day17;

impl Solution for Day17 {
    fn part_one(&self, input: &str) -> String {
        let grid: Grid<u8> = Self::parse(input);

        let logic = Box::new(PartOneNeighbours { grid: grid.clone() });

        Self::solve(&grid, logic.clone(), logic.clone())
    }

    fn part_two(&self, input: &str) -> String {
        let grid: Grid<u8> = Self::parse(input);

        let logic = Box::new(PartTwoNeighbours { grid: grid.clone() });

        Self::solve(&grid, logic.clone(), logic.clone())
    }
}

impl Day17 {
    fn parse(input: &str) -> Grid<u8> {
        Grid::from_custom(input, |c| c.to_digit(10).unwrap() as u8)
    }

    fn solve(grid: &Grid<u8>, neighbours_provider: Box<dyn Neighbours<Node>>, is_at_end: Box<dyn IsAtEnd<Node>>) -> String {
        let start_point = grid.surface_range().top_left_corner();
        let cost_from_grid = Box::new(CostFromGrid { grid: grid.clone() });
        let dijkstra: Dijkstra<Node> = Dijkstra::new(neighbours_provider, cost_from_grid, is_at_end);

        dijkstra.cost(Node::new(start_point.clone(), Direction::East)).to_string()
    }
}

#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Node {
    vector: Vector,
    forward_count: u8,
}

impl Node {
    fn new(position: Point, direction: Direction) -> Self {
        Self {
            vector: Vector::new(position.clone(), direction),
            forward_count: 0,
        }
    }
    fn forward(&self) -> Self {
        Self { vector: self.vector.step(), forward_count: self.forward_count + 1 }
    }

    fn left(&self) -> Self {
        Self { vector: self.vector.rotate_ccw().step(), forward_count: 1 }
    }

    fn right(&self) -> Self {
        Self { vector: self.vector.rotate_cw().step(), forward_count: 1 }
    }
}

#[derive(Clone)]
struct CostFromGrid {
    grid: Grid<u8>,
}

impl CostProvider<Node> for CostFromGrid {
    fn cost(&self, node: Node) -> usize {
        *self.grid.get_for_point(&node.vector.position()).unwrap() as usize
    }
}

#[derive(Clone)]
struct PartOneNeighbours {
    grid: Grid<u8>,
}

impl Neighbours<Node> for PartOneNeighbours {
    fn neighbours(&self, node: Node) -> Vec<Node> {
        let mut vec: Vec<Node> = vec![node.left(), node.right()];
        if node.forward_count < 3 {
            vec.push(node.forward());
        }

        vec.into_iter()
            .filter(|n| self.grid.surface_range().contains(n.vector.position()))
            .collect()
    }
}

impl IsAtEnd<Node> for PartOneNeighbours {
    fn is_end(&self, node: Node) -> bool {
        node.vector.position() == self.grid.surface_range().bottom_right_corner()
    }
}

#[derive(Clone)]
struct PartTwoNeighbours {
    grid: Grid<u8>,
}

impl Neighbours<Node> for PartTwoNeighbours {
    fn neighbours(&self, node: Node) -> Vec<Node> {
        let mut vec: Vec<Node> = vec![];
        if node.vector.position() == self.grid.surface_range().top_left_corner() {
            vec.push(node.forward());
            vec.push(node.left());
            vec.push(node.right());
        } else if node.forward_count < 4 {
            vec.push(node.forward());
        } else if node.forward_count >= 4 && node.forward_count < 10 {
            vec.push(node.forward());
            vec.push(node.left());
            vec.push(node.right());
        } else {
            vec.push(node.left());
            vec.push(node.right());
        }

        vec.into_iter()
            .filter(|n| self.grid.surface_range().contains(n.vector.position()))
            .collect()
    }
}

impl IsAtEnd<Node> for PartTwoNeighbours {
    fn is_end(&self, node: Node) -> bool {
        node.forward_count >= 4 && node.vector.position() == self.grid.surface_range().bottom_right_corner()
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day17::Day17;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("17");

        assert_eq!("102", Day17.part_one(&input.as_str()));
    }

    #[test]
    fn part_two_example_test() {
        let input = read_example("17");

        assert_eq!("94", Day17.part_two(&input.as_str()));
    }

    #[test]
    fn part_two_example_2_test() {
        let input = read_example("17_2");

        assert_eq!("71", Day17.part_two(&input.as_str()));
    }
}
