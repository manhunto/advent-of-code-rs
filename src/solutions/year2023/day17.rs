use crate::solutions::Solution;
use crate::utils::direction::Direction;
use crate::utils::graphs::dijkstra::Dijkstra;
use crate::utils::grid::Grid;
use crate::utils::point::Point;
use crate::utils::vector::Vector;

pub struct Day17;

impl Solution for Day17 {
    fn part_one(&self, input: &str) -> String {
        let grid: Grid<u8> = Self::parse(input);

        let adjacency = |node: Node| -> Vec<Node> {
            let vec: Vec<Node> = if node.forward_count < 3 {
                vec![node.left(), node.right(), node.forward()]
            } else {
                vec![node.left(), node.right()]
            };

            Self::filter_out_outside_grid(vec, &grid)
        };

        let is_end = |node: Node| -> bool { Self::is_end_node(node, &grid) };

        Self::solve(&grid, &adjacency, &is_end)
    }

    fn part_two(&self, input: &str) -> String {
        let grid: Grid<u8> = Self::parse(input);

        let adjacency = |node: Node| -> Vec<Node> {
            let vec: Vec<Node> = if node.forward_count < 4 {
                vec![node.forward()]
            } else if node.forward_count >= 4 && node.forward_count < 10 {
                vec![node.forward(), node.left(), node.right()]
            } else {
                vec![node.left(), node.right()]
            };

            Self::filter_out_outside_grid(vec, &grid)
        };

        let is_end =
            |node: Node| -> bool { node.forward_count >= 4 && Self::is_end_node(node, &grid) };

        Self::solve(&grid, &adjacency, &is_end)
    }
}

impl Day17 {
    fn parse(input: &str) -> Grid<u8> {
        Grid::from_custom(input, |c| c.to_digit(10).unwrap() as u8)
    }

    fn solve(
        grid: &Grid<u8>,
        adjacency: &dyn Fn(Node) -> Vec<Node>,
        is_end: &dyn Fn(Node) -> bool,
    ) -> String {
        let start_point = grid.surface().top_left_corner();
        let cost = |_, next: Node| *grid.get_for_point(&next.vector.position()).unwrap() as usize;
        let dijkstra: Dijkstra<Node> = Dijkstra::new(adjacency, &cost, is_end);

        let starts = vec![
            Node::new(start_point, Direction::East),
            Node::new(start_point, Direction::South),
        ];

        dijkstra.cost(starts).unwrap().to_string()
    }

    fn filter_out_outside_grid(vec: Vec<Node>, grid: &Grid<u8>) -> Vec<Node> {
        vec.into_iter()
            .filter(|n| grid.surface().contains(n.vector.position()))
            .collect()
    }

    fn is_end_node(node: Node, grid: &Grid<u8>) -> bool {
        node.vector.position() == grid.surface().bottom_right_corner()
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
            vector: Vector::new(position, direction),
            forward_count: 0,
        }
    }
    fn forward(&self) -> Self {
        Self {
            vector: self.vector.forward(),
            forward_count: self.forward_count + 1,
        }
    }

    fn left(&self) -> Self {
        Self {
            vector: self.vector.rotate_ccw().forward(),
            forward_count: 1,
        }
    }

    fn right(&self) -> Self {
        Self {
            vector: self.vector.rotate_cw().forward(),
            forward_count: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2023::day17::Day17;
    use crate::solutions::year2023::read_2023_example;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_2023_example("17");

        assert_eq!("102", Day17.part_one(input.as_str()));
    }

    #[test]
    fn part_two_example_test() {
        let input = read_2023_example("17");

        assert_eq!("94", Day17.part_two(input.as_str()));
    }

    #[test]
    fn part_two_example_2_test() {
        let input = read_2023_example("17_2");

        assert_eq!("71", Day17.part_two(input.as_str()));
    }
}
