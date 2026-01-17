use crate::utils::graphs::state_utils::State;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;
use std::hash::Hash;

/// A* pathfinding algorithm with iterator-based neighbour function
pub struct AStar<'a, T, I>
where
    I: IntoIterator<Item = T>,
{
    neighbours: &'a dyn Fn(T) -> I,
    distance: &'a dyn Fn(T, T) -> usize,
    memory_size: usize,
}

impl<'a, T, I> AStar<'a, T, I>
where
    I: IntoIterator<Item = T>,
{
    pub fn new(
        neighbours: &'a dyn Fn(T) -> I,
        distance: &'a dyn Fn(T, T) -> usize,
        memory_size: usize,
    ) -> Self {
        Self {
            neighbours,
            distance,
            memory_size,
        }
    }

    /// Finds the shortest path from start to end using A* algorithm
    pub fn path(&self, start: T, end: T) -> Option<Vec<T>>
    where
        T: Hash + Eq + PartialEq + Ord + Copy + Debug,
    {
        let mut open_set = BinaryHeap::with_capacity(self.memory_size);
        let mut came_from: HashMap<T, T> = HashMap::with_capacity(self.memory_size);
        let mut g_score: HashMap<T, usize> = HashMap::with_capacity(self.memory_size);

        let initial_distance = (self.distance)(start, end);

        g_score.insert(start, 0);
        open_set.push(State::new(start, initial_distance));

        while let Some(State { node: current, .. }) = open_set.pop() {
            if current == end {
                return Some(Self::build_path(&came_from, current));
            }

            let current_g_score = *g_score.get(&current)?;

            // Process neighbours using iterator
            for neighbour in (self.neighbours)(current) {
                let tentative_g_score = current_g_score + 1;

                // Only update if this path is better
                if tentative_g_score < *g_score.get(&neighbour).unwrap_or(&usize::MAX) {
                    came_from.insert(neighbour, current);
                    g_score.insert(neighbour, tentative_g_score);

                    let f_score = tentative_g_score + (self.distance)(neighbour, end);
                    open_set.push(State::new(neighbour, f_score));
                }
            }
        }

        None
    }

    /// Reconstructs the path from start to end by following came_from links
    fn build_path(came_from: &HashMap<T, T>, mut current: T) -> Vec<T>
    where
        T: Hash + Eq + Copy,
    {
        let mut path = vec![current];

        while let Some(&previous) = came_from.get(&current) {
            path.push(previous);
            current = previous;
        }

        path.reverse();
        path
    }
}

/// Builder for AStar with sensible defaults
pub struct AStarBuilder<'a, T, I>
where
    I: IntoIterator<Item = T>,
{
    neighbours: &'a dyn Fn(T) -> I,
    distance: &'a dyn Fn(T, T) -> usize,
    memory_size: Option<usize>,
}

impl<'a, T, I> AStarBuilder<'a, T, I>
where
    I: IntoIterator<Item = T>,
{
    /// Initialize the builder with required functions
    pub fn init(neighbours: &'a dyn Fn(T) -> I, distance: &'a dyn Fn(T, T) -> usize) -> Self {
        Self {
            neighbours,
            distance,
            memory_size: None,
        }
    }

    /// Set the initial capacity for internal data structures
    pub fn memory_size(mut self, memory_size: usize) -> Self {
        self.memory_size = Some(memory_size);
        self
    }

    /// Build the AStar instance
    pub fn build(self) -> AStar<'a, T, I> {
        AStar::new(
            self.neighbours,
            self.distance,
            self.memory_size.unwrap_or(1000),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::point::Point;

    #[test]
    fn test_astar_with_iterator() {
        let start = Point::new(0, 0);
        let end = Point::new(2, 2);

        let neighbours = |point: Point| {
            point
                .adjacent()
                .into_iter()
                .filter(|p| p.x >= 0 && p.y >= 0 && p.x <= 2 && p.y <= 2)
        };

        let distance = |from: Point, to: Point| from.manhattan_distance(&to) as usize;

        let astar = AStarBuilder::init(&neighbours, &distance).build();

        let path = astar.path(start, end);
        assert!(path.is_some());
        assert_eq!(path.unwrap().len(), 5); // 0,0 -> 0,1 -> 0,2 -> 1,2 -> 2,2 or similar
    }
}
