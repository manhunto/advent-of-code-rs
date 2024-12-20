use crate::utils::graphs::state_utils::State;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;
use std::hash::Hash;

pub struct AStar<'a, T> {
    neighbours: &'a dyn Fn(T) -> Vec<T>,
    distance: &'a dyn Fn(T, T) -> usize,
}

impl<'a, T> AStar<'a, T> {
    pub fn new(neighbours: &'a dyn Fn(T) -> Vec<T>, distance: &'a dyn Fn(T, T) -> usize) -> Self {
        Self {
            neighbours,
            distance,
        }
    }

    pub fn path(&self, start: T, end: T) -> Option<Vec<T>>
    where
        T: Hash + Eq + PartialEq + Ord + Copy + Debug,
    {
        let mut open_set = BinaryHeap::new();
        let mut came_from: HashMap<T, T> = HashMap::new();
        let mut g_score: HashMap<T, usize> = HashMap::new();
        let mut f_score: HashMap<T, usize> = HashMap::new();

        let distance = (self.distance)(start, end);

        g_score.insert(start, 0);
        f_score.insert(start, distance);

        open_set.push(State::new(start, distance));

        while let Some(State { node: current, .. }) = open_set.pop() {
            if current == end {
                let path = Self::build_path(&mut came_from, current);

                return Some(path);
            }

            let neighbours = (self.neighbours)(current);
            for neighbour in neighbours {
                let tentative_g_score = g_score.get(&current).unwrap() + 1;

                if !g_score.contains_key(&neighbour)
                    || tentative_g_score < *g_score.get(&neighbour).unwrap()
                {
                    came_from.insert(neighbour, current);
                    g_score.insert(neighbour, tentative_g_score);

                    let f = tentative_g_score + (self.distance)(neighbour, end);

                    f_score.insert(neighbour, f);
                    open_set.push(State::new(neighbour, f));
                }
            }
        }

        None
    }

    fn build_path(came_from: &mut HashMap<T, T>, current: T) -> Vec<T>
    where
        T: Hash + Eq + Copy,
    {
        let mut path = vec![current];
        let mut current_point = current;

        while let Some(&previous) = came_from.get(&current_point) {
            path.push(previous);
            current_point = previous;
        }

        path.reverse();
        path
    }
}
