use crate::utils::graphs::state_utils::State;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;
use std::hash::Hash;

pub struct Dijkstra<'a, T> {
    neighbours: &'a dyn Fn(T) -> Vec<T>,
    cost: &'a dyn Fn(T, T) -> usize,
    is_end: &'a dyn Fn(T) -> bool,
}

impl<'a, T> Dijkstra<'a, T> {
    pub fn new(
        neighbours: &'a dyn Fn(T) -> Vec<T>,
        cost: &'a dyn Fn(T, T) -> usize,
        is_end: &'a dyn Fn(T) -> bool,
    ) -> Self {
        Self {
            neighbours,
            cost,
            is_end,
        }
    }

    pub fn cost(&self, starts: Vec<T>) -> Option<usize>
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
            if (self.is_end)(node) {
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
    /// FIXME: is not working in valid way
    pub fn all_path(&self, starts: Vec<T>) -> HashMap<T, Option<T>>
    where
        T: Hash + Eq + PartialEq + Ord + Debug + Copy,
    {
        let mut current_costs: HashMap<T, usize> = HashMap::new();
        let mut heap = BinaryHeap::new();
        let mut come_from = HashMap::new();

        for start in starts {
            current_costs.insert(start, 0);
            heap.push(State::new(start, 0));
            come_from.insert(start, None);
        }

        let mut lowest: Option<usize> = None;

        while let Some(State { cost, node }) = heap.pop() {
            if (self.is_end)(node) {
                lowest = Some(cost);

                continue;
            }

            if lowest.is_some_and(|v| v < cost) {
                break;
            }

            for neighbour in (self.neighbours)(node) {
                let neighbour_cost = cost + (self.cost)(node, neighbour);
                let current_neighbour_cost = current_costs.get(&neighbour).unwrap_or(&usize::MAX);

                if neighbour_cost < *current_neighbour_cost {
                    *current_costs.entry(neighbour).or_insert(usize::MAX) = neighbour_cost;
                    come_from.insert(neighbour, Some(node));
                    heap.push(State::new(neighbour, neighbour_cost));
                }
            }
        }

        come_from
    }
}
