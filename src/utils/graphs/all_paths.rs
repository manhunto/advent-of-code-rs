use crate::utils::graphs::graph::Graph;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

pub trait IsEnd<T> {
    fn is_end(&self, value: &T) -> bool;
}

impl<T: PartialEq> IsEnd<T> for T {
    fn is_end(&self, value: &T) -> bool {
        self == value
    }
}

impl<T: PartialEq> IsEnd<T> for Vec<T> {
    fn is_end(&self, value: &T) -> bool {
        self.contains(value)
    }
}

pub struct AllPaths<'a, T: 'a> {
    adjacency: Box<dyn Fn(T) -> Vec<T> + 'a>,
}

impl<'a, T> From<&'a Graph<T>> for AllPaths<'a, T>
where
    T: Eq + Hash + Copy + PartialEq,
{
    fn from(graph: &'a Graph<T>) -> Self {
        Self::new(move |p: T| graph.neighbours(&p))
    }
}

impl<'a, T> From<&'a HashMap<T, Vec<T>>> for AllPaths<'a, T>
where
    T: Eq + Hash + Copy + PartialEq,
{
    fn from(value: &'a HashMap<T, Vec<T>>) -> Self {
        Self::new(move |p: T| value.get(&p).unwrap().to_vec())
    }
}
impl<'a, T> AllPaths<'a, T>
where
    T: Eq + Hash + Copy,
{
    pub fn new(adjacency: impl Fn(T) -> Vec<T> + 'a) -> Self {
        Self {
            adjacency: Box::new(adjacency),
        }
    }

    pub fn paths<E>(&self, start: T, end: E) -> Vec<VecDeque<T>>
    where
        E: IsEnd<T>,
    {
        let mut paths: Vec<VecDeque<T>> = Vec::new();
        let mut visited = HashSet::new();
        let mut path = VecDeque::new();

        self.visit(start, &end, &mut visited, &mut path, &mut paths);

        paths
    }

    fn visit<E>(
        &self,
        from: T,
        end: &E,
        visited: &mut HashSet<T>,
        path: &mut VecDeque<T>,
        paths: &mut Vec<VecDeque<T>>,
    ) where
        E: IsEnd<T>,
    {
        visited.insert(from);
        path.push_back(from);

        if end.is_end(&from) {
            paths.push(path.clone());
        } else {
            for p in (self.adjacency)(from) {
                if !visited.contains(&p) {
                    self.visit(p, end, visited, path, paths);
                }
            }
        }

        path.pop_back();
        visited.remove(&from);
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::graphs::all_paths::AllPaths;
    use crate::utils::graphs::graph::Graph;
    use crate::utils::point::Point;
    use std::collections::{HashMap, VecDeque};

    #[test]
    fn paths() {
        let p0 = Point::new(0, 0);
        let p1 = Point::new(1, 1);
        let p2 = Point::new(2, 2);
        let p3 = Point::new(3, 3);

        let graph: HashMap<Point, Vec<Point>> = HashMap::from([
            (p0, vec![p1, p2, p3]),
            (p1, vec![p3]),
            (p2, vec![p0, p1]),
            (p3, vec![]),
        ]);

        let all_paths = AllPaths::from(&graph);

        let paths = all_paths.paths(p2, p3);

        assert_eq!(3, paths.len());
        assert!(paths.contains(&VecDeque::from(vec![p2, p1, p3])));
        assert!(paths.contains(&VecDeque::from(vec![p2, p0, p3])));
        assert!(paths.contains(&VecDeque::from(vec![p2, p0, p1, p3])));
    }

    #[test]
    fn paths_vec_end() {
        let p0 = Point::new(0, 0);
        let p1 = Point::new(1, 1);
        let p2 = Point::new(2, 2);
        let p3 = Point::new(3, 3);
        let p4 = Point::new(4, 4);

        let graph: HashMap<Point, Vec<Point>> = HashMap::from([
            (p0, vec![p1, p2]),
            (p1, vec![p3]),
            (p2, vec![p4]),
            (p3, vec![]),
            (p4, vec![]),
        ]);
        let adjacency = |p: Point| graph.get(&p).unwrap().to_vec();

        let all_paths: AllPaths<Point> = AllPaths::new(adjacency);

        let paths = all_paths.paths(p0, vec![p3, p4]);

        assert_eq!(2, paths.len());
        assert!(paths.contains(&VecDeque::from(vec![p0, p1, p3])));
        assert!(paths.contains(&VecDeque::from(vec![p0, p2, p4])));
    }

    #[test]
    fn paths_from_graph() {
        let p0 = Point::new(0, 0);
        let p1 = Point::new(1, 1);
        let p2 = Point::new(2, 2);
        let p3 = Point::new(3, 3);

        let mut graph = Graph::undirected();
        graph.add_edge(p0, p1);
        graph.add_edge(p0, p2);
        graph.add_edge(p0, p3);
        graph.add_edge(p1, p3);
        graph.add_edge(p2, p1);

        let all_paths = AllPaths::from(&graph);
        let paths = all_paths.paths(p2, p3);

        assert_eq!(4, paths.len());
        assert!(paths.contains(&VecDeque::from(vec![p2, p1, p3])));
        assert!(paths.contains(&VecDeque::from(vec![p2, p0, p3])));
        assert!(paths.contains(&VecDeque::from(vec![p2, p0, p1, p3])));
        assert!(paths.contains(&VecDeque::from(vec![p2, p1, p0, p3])));
    }
}
