use crate::utils::graphs::graph::Graph;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

pub struct TravellingSalesman<T> {
    graph: Graph<T>,
    weights: HashMap<(T, T), usize>,
}

impl<T: PartialEq + Eq + Hash> TravellingSalesman<T> {
    pub fn add(&mut self, from: T, to: T, weight: usize)
    where
        T: Eq + Hash + Copy,
    {
        self.weights.insert((from, to), weight);
        self.weights.insert((to, from), weight);
        self.graph.add_edge(from, to);
    }

    pub fn find_shortest_path_cost(&self) -> Option<usize>
    where
        T: PartialEq + Clone,
    {
        self.all_paths().iter().map(|path| path.cost).min()
    }

    pub fn find_longest_path_cost(&self) -> Option<usize>
    where
        T: PartialEq + Clone,
    {
        self.all_paths().iter().map(|path| path.cost).max()
    }

    fn all_paths(&self) -> Vec<Path<T>>
    where
        T: PartialEq + Clone,
    {
        self.graph
            .nodes()
            .iter()
            .tuple_combinations()
            .flat_map(|(from, to)| self.all_paths_between(from, to))
            .collect()
    }

    fn all_paths_between(&self, from: &T, to: &T) -> Vec<Path<T>>
    where
        T: PartialEq + Clone,
    {
        let path_length = self.graph.nodes().len();

        let mut queue: VecDeque<Path<T>> = VecDeque::new();
        queue.push_back(Path::new(from, to, path_length));

        let mut finished_paths: Vec<Path<T>> = Vec::with_capacity(path_length); // todo calculate length of all paths

        while let Some(path) = queue.pop_front() {
            if path.is_closed() {
                finished_paths.push(path);
                continue;
            }

            for node in self.graph.nodes() {
                let mut new_path = path.clone();
                let last = new_path.last().clone();
                let next = node.clone();

                if last == next {
                    continue;
                }

                let cost = self.weights[&(last, next)];

                if new_path.add(node, cost).is_ok() {
                    queue.push_back(new_path);
                }
            }
        }

        finished_paths
    }
}

impl<T> Default for TravellingSalesman<T> {
    fn default() -> Self {
        Self {
            graph: Graph::undirected(),
            weights: HashMap::new(),
        }
    }
}

#[derive(Clone)]
struct Path<T> {
    visited: Vec<T>,
    to: T,
    expected_path_length: usize,
    cost: usize,
}

impl<T> Path<T> {
    fn new(from: &T, to: &T, expected_path_length: usize) -> Self
    where
        T: Clone,
    {
        let mut visited = Vec::with_capacity(expected_path_length);
        visited.push(from.clone());

        Self {
            visited,
            to: to.clone(),
            expected_path_length,
            cost: 0,
        }
    }

    fn add(&mut self, node: &T, cost: usize) -> Result<(), &'static str>
    where
        T: Clone + PartialEq,
    {
        if *node == self.to && self.visited.len() < self.expected_path_length - 1 {
            return Err("Cannot add end node for path that haven't visited all other nodes");
        }

        if self.visited.contains(node) {
            return Err("Cannot visit node already visited");
        }

        self.visited.push(node.clone());
        self.cost += cost;

        Ok(())
    }

    fn last(&self) -> &T {
        self.visited
            .iter()
            .last()
            .expect("There must be a visited node")
    }

    fn is_closed(&self) -> bool {
        self.expected_path_length == self.visited.len()
    }
}
