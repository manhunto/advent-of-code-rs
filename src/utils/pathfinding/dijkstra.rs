use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

pub trait Neighbours<T> {
    fn neighbours(&self, node: T) -> Vec<T>;
}

pub trait IsAtEnd<T> {
    fn is_end(&self, node: T) -> bool;
}

pub trait CostProvider<T> {
    fn cost(&self, node: T) -> usize;
}

#[derive(PartialEq, Eq)]
struct State<T> {
    node: T,
    cost: usize,
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

pub struct Dijkstra<T> {
    neighbours_provider: Box<dyn Neighbours<T>>,
    cost_provider: Box<dyn CostProvider<T>>,
    is_at_end: Box<dyn IsAtEnd<T>>,
}

impl<T> Dijkstra<T> {
    pub fn new(neighbours_provider: Box<dyn Neighbours<T>>, cost_provider: Box<dyn CostProvider<T>>, is_at_end: Box<dyn IsAtEnd<T>>) -> Self {
        Self { neighbours_provider, cost_provider, is_at_end }
    }

    pub fn cost(&self, start: T) -> usize
        where T: Hash + Eq + PartialEq + Ord + Clone + Copy
    {
        let mut dist_map: HashMap<T, usize> = HashMap::new();
        let mut heap = BinaryHeap::new();

        dist_map.insert(start.clone(), 0);
        heap.push(State { cost: 0, node: start.clone() });

        while let Some(State { cost, node }) = heap.pop() {
            if self.is_at_end.is_end(node.clone()) {
                return cost;
            }

            let dist = *dist_map.get(&node).unwrap_or(&usize::MAX);
            if cost > dist {
                continue;
            }

            for neighbour in self.neighbours_provider.neighbours(node) {
                let neighbour_cost = self.cost_provider.cost(neighbour.clone());
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
}