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
        let grid_clone = grid.clone();
        let grid_clone_2 = grid.clone();

        let adjacency = move |node: Node| -> Vec<Node> {
            let vec: Vec<Node> = if node.forward_count < 3 {
                vec![node.left(), node.right(), node.forward()]
            } else {
                vec![node.left(), node.right()]
            };

            Self::filter_out_outside_grid(vec, &grid_clone)
        };

        let is_end = move |node: Node| -> bool { Self::is_end_node(node, &grid_clone_2) };

        Self::solve(&grid, Box::new(adjacency), Box::new(is_end))
    }

    fn part_two(&self, input: &str) -> String {
        let grid: Grid<u8> = Self::parse(input);
        let grid_clone = grid.clone();
        let grid_clone_2 = grid.clone();

        let adjacency = move |node: Node| -> Vec<Node> {
            let vec: Vec<Node> = if node.forward_count < 4 {
                vec![node.forward()]
            } else if node.forward_count >= 4 && node.forward_count < 10 {
                vec![node.forward(), node.left(), node.right()]
            } else {
                vec![node.left(), node.right()]
            };

            Self::filter_out_outside_grid(vec, &grid_clone)
        };

        let is_end = move |node: Node| -> bool {
            node.forward_count >= 4 && Self::is_end_node(node, &grid_clone_2)
        };

        Self::solve(&grid, Box::new(adjacency), Box::new(is_end))
    }
}

impl Day17 {
    fn parse(input: &str) -> Grid<u8> {
        Grid::from_custom(input, |c| c.to_digit(10).unwrap() as u8)
    }

    fn solve(
        grid: &Grid<u8>,
        adjacency: Box<dyn Fn(Node) -> Vec<Node>>,
        is_end: Box<dyn Fn(Node) -> bool>,
    ) -> String {
        let start_point = grid.surface().top_left_corner();
        let grid_clone = grid.clone();
        let cost = move |_, next: Node| {
            *grid_clone.get_for_point(&next.vector.position()).unwrap() as usize
        };
        let dijkstra: Dijkstra<Node> = Dijkstra::new(adjacency, Box::new(cost));

        let starts = vec![
            Node::new(start_point, Direction::East),
            Node::new(start_point, Direction::South),
        ];

        dijkstra.cost(starts, &is_end).unwrap().to_string()
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
