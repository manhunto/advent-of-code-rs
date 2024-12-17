use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

#[derive(PartialEq, Eq)]
struct State<T> {
    node: T,
    cost: usize,
}

impl<T> State<T> {
    fn new(node: T, cost: usize) -> Self {
        Self { node, cost }
    }
}

impl<T> PartialOrd for State<T>
where
    T: PartialEq + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for State<T>
where
    T: Eq + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

pub struct Dijkstra<'a, T> {
    adjacency: &'a dyn Fn(T) -> Vec<T>,
    cost: &'a dyn Fn(T, T) -> usize,
    is_end: &'a dyn Fn(T) -> bool,
}

impl<'a, T> Dijkstra<'a, T> {
    pub fn new(
        adjacency: &'a dyn Fn(T) -> Vec<T>,
        cost: &'a dyn Fn(T, T) -> usize,
        is_end: &'a dyn Fn(T) -> bool,
    ) -> Self {
        Self {
            adjacency,
            cost,
            is_end,
        }
    }

    pub fn cost(&self, starts: Vec<T>) -> Option<usize>
    where
        T: Hash + Eq + PartialEq + Ord + Clone + Copy,
    {
        let mut dist_map: HashMap<T, usize> = HashMap::new();
        let mut heap = BinaryHeap::new();

        for start in starts {
            dist_map.insert(start, 0);
            heap.push(State::new(start, 0));
        }

        while let Some(State { cost, node }) = heap.pop() {
            if (self.is_end)(node) {
                return Some(cost);
            }

            let dist = *dist_map.get(&node).unwrap_or(&usize::MAX);
            if cost > dist {
                continue;
            }

            for neighbour in (self.adjacency)(node) {
                let neighbour_cost = (self.cost)(node, neighbour);
                let next = State::new(neighbour, cost + neighbour_cost);

                let dist_to_next = dist_map.get(&next.node).unwrap_or(&usize::MAX);
                if next.cost < *dist_to_next {
                    *dist_map.entry(next.node).or_insert(usize::MAX) = next.cost;

                    heap.push(next);
                }
            }
        }

        None
    }

    pub fn all_possible_paths(&self, starts: Vec<T>) -> Vec<(usize, T)>
    where
        T: Hash + Eq + PartialEq + Ord + Clone,
    {
        let mut dist_map: HashMap<T, usize> = HashMap::new();
        let mut heap = BinaryHeap::new();

        for start in starts {
            dist_map.insert(start.clone(), 0);
            heap.push(State::new(start.clone(), 0));
        }

        let mut node_which_ends: Vec<(usize, T)> = vec![];

        while let Some(State { cost, node }) = heap.pop() {
            let dist = *dist_map.get(&node.clone()).unwrap_or(&usize::MAX);
            if (self.is_end)(node.clone()) {
                node_which_ends.push((cost, node.clone()));
                continue;
            }

            if cost > dist {
                continue;
            }

            for neighbour in (self.adjacency)(node.clone()) {
                let neighbour_cost = (self.cost)(node.clone(), neighbour.clone());
                let next = State::new(neighbour.clone(), cost + neighbour_cost);

                let dist_to_next = dist_map.get(&next.node).unwrap_or(&usize::MAX);
                if next.cost < *dist_to_next {
                    *dist_map.entry(next.node.clone()).or_insert(usize::MAX) = next.cost;

                    heap.push(next);
                }
            }
        }

        node_which_ends
    }
}
