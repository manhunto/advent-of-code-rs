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
    where T: PartialEq + Ord
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for State<T>
    where T: Eq + Ord
{
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

pub struct Dijkstra<'a, T> {
    neighbours: &'a dyn Fn(T) -> Vec<T>,
    cost: &'a dyn Fn(T) -> usize,
    is_end: &'a dyn Fn(T) -> bool,
}

impl<'a, T> Dijkstra<'a, T> {
    pub fn new(
        neighbours: &'a dyn Fn(T) -> Vec<T>,
        cost: &'a dyn Fn(T) -> usize,
        is_end: &'a dyn Fn(T) -> bool,
    ) -> Self {
        Self { neighbours, cost, is_end }
    }

    pub fn cost(&self, start: T) -> usize
        where T: Hash + Eq + PartialEq + Ord + Clone + Copy
    {
        let mut dist_map: HashMap<T, usize> = HashMap::new();
        let mut heap = BinaryHeap::new();

        dist_map.insert(start.clone(), 0);
        heap.push(State::new(start.clone(), 0));

        while let Some(State { cost, node }) = heap.pop() {
            if (self.is_end)(node.clone()) {
                return cost;
            }

            let dist = *dist_map.get(&node).unwrap_or(&usize::MAX);
            if cost > dist {
                continue;
            }

            for neighbour in (self.neighbours)(node) {
                let neighbour_cost = (self.cost)(neighbour.clone());
                let next = State::new(neighbour, cost + neighbour_cost);

                let dist_to_next = dist_map.get(&next.node).unwrap_or(&usize::MAX);
                if next.cost < *dist_to_next {
                    *dist_map.entry(next.node).or_insert(usize::MAX) = next.cost;

                    heap.push(next);
                }
            }
        }

        unreachable!()
    }
}