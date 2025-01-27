use crate::utils::graphs::state_utils::State;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

pub struct Dijkstra<T> {
    neighbours: Box<dyn Fn(T) -> Vec<T>>,
    cost: Box<dyn Fn(T, T) -> usize>,
}

impl<T> Dijkstra<T> {
    pub fn new(neighbours: Box<dyn Fn(T) -> Vec<T>>, cost: Box<dyn Fn(T, T) -> usize>) -> Self {
        Self { neighbours, cost }
    }

    pub fn cost(&self, starts: Vec<T>, is_end: &dyn Fn(T) -> bool) -> Option<usize>
    where
        T: Hash + Eq + PartialEq + Ord + Copy + Debug,
    {
        let mut current_costs: HashMap<T, usize> = HashMap::new();
        let mut heap = BinaryHeap::new();

        for start in starts {
            current_costs.insert(start, 0);
            heap.push(State::new(start, 0));
        }

        while let Some(State { cost, node }) = heap.pop() {
            if is_end(node) {
                return Some(cost);
            }

            for neighbour in (self.neighbours)(node) {
                let neighbour_cost = cost + (self.cost)(node, neighbour);
                let current_neighbour_cost = current_costs.get(&neighbour).unwrap_or(&usize::MAX);

                if neighbour_cost < *current_neighbour_cost {
                    *current_costs.entry(neighbour).or_insert(usize::MAX) = neighbour_cost;
                    heap.push(State::new(neighbour, neighbour_cost));
                }
            }
        }

        None
    }

    /// It returns every possible visited node
    /// Even if there is a many possible ways to reach end
    pub fn all_paths(&self, starts: Vec<T>, is_end: &dyn Fn(T) -> bool) -> Vec<VecDeque<T>>
    where
        T: Hash + Eq + PartialEq + Ord + Debug + Copy,
    {
        let mut current_costs: HashMap<T, usize> = HashMap::new();
        let mut heap = BinaryHeap::new();
        let mut come_from: HashMap<T, Vec<T>> = HashMap::new();

        for start in starts.clone() {
            current_costs.insert(start, 0);
            heap.push(State::new(start, 0));
            come_from.insert(start, Vec::new());
        }

        let mut lowest: Option<usize> = None;
        let mut end_nodes: Vec<T> = Vec::new();

        while let Some(State { cost, node }) = heap.pop() {
            if is_end(node) {
                lowest = Some(cost);
                end_nodes.push(node);

                continue;
            }

            if lowest.is_some_and(|v| v < cost) {
                break;
            }

            for neighbour in (self.neighbours)(node) {
                let neighbour_cost = cost + (self.cost)(node, neighbour);
                let current_neighbour_cost = current_costs.get(&neighbour).unwrap_or(&usize::MAX);

                match neighbour_cost.cmp(current_neighbour_cost) {
                    Ordering::Less => {
                        *current_costs.entry(neighbour).or_insert(usize::MAX) = neighbour_cost;
                        come_from.entry(neighbour).or_default().push(node);
                        heap.push(State::new(neighbour, neighbour_cost));
                    }
                    Ordering::Equal => {
                        come_from.entry(neighbour).or_default().push(node);
                    }
                    _ => {}
                }
            }
        }

        self.build_paths(&come_from, starts, end_nodes)
    }

    fn build_paths(
        &self,
        come_from: &HashMap<T, Vec<T>>,
        start_nodes: Vec<T>,
        end_nodes: Vec<T>,
    ) -> Vec<VecDeque<T>>
    where
        T: Hash + Eq + PartialEq + Ord + Debug + Copy,
    {
        let mut paths: Vec<VecDeque<T>> = Vec::new();

        for start in start_nodes {
            for end in &end_nodes {
                Self::visit(start, *end, &mut VecDeque::new(), &mut paths, come_from);
            }
        }

        paths
    }

    fn visit(
        end: T,
        current: T,
        current_path: &mut VecDeque<T>,
        paths: &mut Vec<VecDeque<T>>,
        come_from: &HashMap<T, Vec<T>>,
    ) where
        T: Hash + Eq + PartialEq + Ord + Debug + Copy,
    {
        current_path.push_front(current);

        if end == current {
            paths.push(current_path.clone());
            current_path.pop_front();

            return;
        }

        for p in come_from.get(&current).unwrap_or(&Vec::new()) {
            if !current_path.contains(p) {
                Self::visit(end, *p, current_path, paths, come_from);
            }
        }

        current_path.pop_front();
    }
}

#[cfg(test)]
mod test {
    use crate::utils::graphs::dijkstra::Dijkstra;
    use std::collections::VecDeque;

    #[test]
    fn all_paths() {
        let dijkstra = Dijkstra::new(
            Box::new(|node: char| match node {
                'A' => vec!['^', '>'],
                '^' => vec!['A', 'v'],
                '>' => vec!['A', 'v'],
                'v' => vec!['^', '>', '<'],
                '<' => vec!['v'],
                _ => unreachable!("Invalid node"),
            }),
            Box::new(|_: char, _: char| 1),
        );
        let is_end = |node: char| node == '<';

        let paths = dijkstra.all_paths(vec!['A'], &is_end);

        assert_eq!(paths.len(), 2);
        assert!(paths.contains(&VecDeque::from(vec!['A', '^', 'v', '<'])));
        assert!(paths.contains(&VecDeque::from(vec!['A', '>', 'v', '<'])));
    }
}
