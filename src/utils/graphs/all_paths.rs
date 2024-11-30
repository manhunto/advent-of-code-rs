use std::collections::VecDeque;

pub struct AllPaths<'a, T> {
    adjacency: &'a dyn Fn(T) -> Vec<T>,
}

impl<'a, T> AllPaths<'a, T>
where
    T: PartialEq + Clone,
{
    pub fn new(adjacency: &'a dyn Fn(T) -> Vec<T>) -> Self {
        Self { adjacency }
    }

    pub fn generate(&self, start: T, end: T) -> Vec<VecDeque<T>> {
        let mut paths: Vec<VecDeque<T>> = Vec::new();

        self.visit(start, end, Vec::new(), VecDeque::new(), &mut paths);

        paths
    }

    fn visit(
        &self,
        from: T,
        end: T,
        mut visited: Vec<T>,
        mut path: VecDeque<T>,
        paths: &mut Vec<VecDeque<T>>,
    ) {
        visited.push(from.clone());
        path.push_back(from.clone());

        if from == end {
            paths.push(path.clone());

            return;
        }

        for p in (self.adjacency)(from) {
            if !visited.contains(&p) {
                self.visit(p, end.clone(), visited.clone(), path.clone(), paths);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::point::Point;
    use crate::utils::graphs::all_paths::AllPaths;
    use std::collections::{HashMap, VecDeque};

    #[test]
    fn generate() {
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
        let adjacency = |p: Point| graph.get(&p).unwrap().to_vec();

        let all_paths: AllPaths<Point> = AllPaths::new(&adjacency);

        let paths = all_paths.generate(p2, p3);

        assert_eq!(3, paths.len());
        assert!(paths.contains(&VecDeque::from(vec![p2, p1, p3])));
        assert!(paths.contains(&VecDeque::from(vec![p2, p0, p3])));
        assert!(paths.contains(&VecDeque::from(vec![p2, p0, p1, p3])));
    }
}
