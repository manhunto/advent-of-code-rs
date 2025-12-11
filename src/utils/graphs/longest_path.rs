use crate::utils::graphs::all_paths::AllPaths;
use hash::Hash;
use itertools::Itertools;
use std::collections::VecDeque;
use std::hash;

pub struct LongestPath<'a, T> {
    adjacency: &'a dyn Fn(T) -> Vec<T>,
    cost: &'a dyn Fn(T, T) -> usize,
}

impl<'a, T> LongestPath<'a, T>
where
    T: PartialEq + Clone + Copy + Eq + Hash,
{
    pub fn new(adjacency: &'a dyn Fn(T) -> Vec<T>, cost: &'a dyn Fn(T, T) -> usize) -> Self {
        Self { adjacency, cost }
    }

    pub fn cost(&self, start: T, end: T) -> usize {
        let all_paths: AllPaths<T> = AllPaths::new(self.adjacency);
        let paths = all_paths.paths(start, end);

        paths
            .into_iter()
            .map(|path| self.calculate_cost(path))
            .max()
            .unwrap()
    }

    fn calculate_cost(&self, path: VecDeque<T>) -> usize {
        path.iter()
            .tuple_windows::<(_, _)>()
            .map(|(from, to)| (self.cost)(*from, *to))
            .sum::<usize>()
    }
}
