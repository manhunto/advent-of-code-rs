use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use crate::direction::Direction;
use crate::grid::Grid;
use crate::solutions::Solution;
use crate::utils::vector::Vector;

pub struct Day17;

impl Solution for Day17 {
    fn part_one(&self, input: &str) -> String {
        let grid: Grid<u8> = Grid::from_custom(input, |c| c.to_digit(10).unwrap() as u8);
        let surface = grid.surface_range();
        let start_point = surface.top_left_corner();
        let logic = Logic { grid };

        let start_east = Node {
            vector: Vector::new(start_point.clone(), Direction::East),
            forward_count: 1,
        };
        let result_east = dijkstra(&logic, &logic, &logic, start_east);

        let start_south = Node {
            vector: Vector::new(start_point.clone(), Direction::South),
            forward_count: 1,
        };
        let result_south = dijkstra(&logic, &logic, &logic, start_south);

        result_east.min(result_south).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        String::from('0')
    }
}

#[derive(Clone, Copy, Hash, Ord, PartialOrd, Debug)]
struct Node {
    vector: Vector,
    forward_count: u8,
}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.vector.position() == other.vector.position()
    }
}

impl Eq for Node {}

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

#[derive(PartialEq, Eq)]
struct State {
    node: Node,
    cost: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

trait Neighbours {
    fn neighbours(&self, node: Node) -> Vec<Node>;
}

trait IsAtEnd {
    fn is_end(&self, node: Node) -> bool;
}

trait CostProvider {
    fn cost(&self, node: Node) -> usize;
}

struct Logic {
    grid: Grid<u8>,
}

impl Neighbours for Logic {
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

impl CostProvider for Logic {
    fn cost(&self, node: Node) -> usize {
        *self.grid.get_for_point(&node.vector.position()).unwrap() as usize
    }
}

impl IsAtEnd for Logic {
    fn is_end(&self, node: Node) -> bool {
        node.vector.position() == self.grid.surface_range().bottom_right_corner()
    }
}

fn dijkstra(neighbours: &impl Neighbours, cost_provider: &impl CostProvider, is_at_end: &impl IsAtEnd, start: Node) -> usize {
    let mut dist_map: HashMap<Node, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist_map.insert(start, 0);
    heap.push(State { cost: 0, node: start });

    while let Some(State { cost, node }) = heap.pop() {
        if is_at_end.is_end(node) {
            return cost;
        }

        let dist = *dist_map.get(&node).unwrap_or(&usize::MAX);
        if cost > dist {
            continue;
        }

        for neighbour in neighbours.neighbours(node) {
            let neighbour_cost = cost_provider.cost(neighbour);
            let next = State { cost: cost + neighbour_cost, node: neighbour };

            let dist_to_next = dist_map.get(&next.node).unwrap_or(&usize::MAX);
            if next.cost < *dist_to_next {
                *dist_map.entry(next.node).or_insert(usize::MAX) = next.cost;

                heap.push(next);
            }
        }
    }

    unreachable!()
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
