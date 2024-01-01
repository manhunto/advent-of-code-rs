use crate::direction::Direction;
use crate::grid::Grid;
use crate::solutions::Solution;
use crate::utils::pathfinding::dijkstra::{CostProvider, Dijkstra, IsAtEnd, Neighbours};
use crate::utils::vector::Vector;

pub struct Day17;

impl Solution for Day17 {
    fn part_one(&self, input: &str) -> String {
        let grid: Grid<u8> = Grid::from_custom(input, |c| c.to_digit(10).unwrap() as u8);
        let start_point = grid.surface_range().top_left_corner();
        let logic: Logic = Logic { grid };

        let start_east = Node {
            vector: Vector::new(start_point.clone(), Direction::East),
            forward_count: 1,
        };
        let dijkstra: Dijkstra<Node> = Dijkstra::new(Box::new(logic.clone()), Box::new(logic.clone()), Box::new(logic.clone()));
        let result_east = dijkstra.cost(start_east);

        let start_south = Node {
            vector: Vector::new(start_point.clone(), Direction::South),
            forward_count: 1,
        };
        let result_south = dijkstra.cost(start_south);

        result_east.min(result_south).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        String::from('0')
    }
}

#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Node {
    vector: Vector,
    forward_count: u8,
}

impl Node {
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
struct Logic {
    grid: Grid<u8>,
}

impl Neighbours<Node> for Logic {
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

impl CostProvider<Node> for Logic {
    fn cost(&self, node: Node) -> usize {
        *self.grid.get_for_point(&node.vector.position()).unwrap() as usize
    }
}

impl IsAtEnd<Node> for Logic {
    fn is_end(&self, node: Node) -> bool {
        node.vector.position() == self.grid.surface_range().bottom_right_corner()
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
}
