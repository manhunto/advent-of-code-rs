use std::collections::{HashMap, HashSet};
use std::hash::Hash;

enum GraphType {
    #[allow(dead_code)]
    Directed,
    Undirected,
}

pub struct Graph<T> {
    nodes: HashSet<T>,
    edges: HashSet<(T, T)>,
    neighbours: HashMap<T, Vec<T>>,
    graph_type: GraphType,
}

impl<T> Graph<T> {
    pub fn undirected() -> Self {
        Self {
            nodes: HashSet::new(),
            edges: HashSet::new(),
            neighbours: HashMap::new(),
            graph_type: GraphType::Undirected,
        }
    }

    pub fn add_edge(&mut self, a: T, b: T)
    where
        T: Eq + Hash + Copy,
    {
        match self.graph_type {
            GraphType::Directed => self.add_directed_edge(a, b),
            GraphType::Undirected => self.add_undirected_edge(a, b),
        }
    }

    fn add_directed_edge(&mut self, a: T, b: T)
    where
        T: Eq + Hash + Copy,
    {
        self.nodes.insert(a);
        self.nodes.insert(b);
        self.edges.insert((a, b));
        self.neighbours.entry(a).or_default().push(b);
    }

    fn add_undirected_edge(&mut self, a: T, b: T)
    where
        T: Eq + Hash + Copy,
    {
        self.nodes.insert(a);
        self.nodes.insert(b);
        self.edges.insert((a, b));
        self.neighbours.entry(a).or_default().push(b);
        self.neighbours.entry(b).or_default().push(a);
    }

    pub fn edges(&self) -> &HashSet<(T, T)> {
        &self.edges
    }

    pub fn neighbours(&self, node: &T) -> Vec<T>
    where
        T: Eq + Hash + Copy,
    {
        self.neighbours.get(node).unwrap_or(&Vec::new()).to_vec()
    }

    #[cfg(test)]
    pub fn cycles(&self) -> HashSet<Vec<T>>
    where
        T: Eq + Hash + Copy + Ord,
    {
        let mut cycles = HashSet::new();

        for node in &self.nodes {
            let mut stack = vec![(vec![*node], *node)];

            while let Some((path, current)) = stack.pop() {
                if path.len() > 2 && self.neighbours(&current).contains(&path[0]) {
                    let mut path_cloned = path.clone();
                    path_cloned.sort();

                    cycles.insert(path_cloned);

                    continue;
                }

                for neighbor in self.neighbours(&current) {
                    if !path.contains(&neighbor) {
                        let mut new_path = path.clone();
                        new_path.push(neighbor);
                        stack.push((new_path, neighbor));
                    }
                }
            }
        }

        cycles
    }

    pub fn cliques(&self) -> HashSet<Vec<T>>
    where
        T: Eq + Hash + Copy + Ord,
    {
        let mut cliques = HashSet::new();

        for node in &self.nodes {
            let mut stack = vec![vec![*node]];

            while let Some(clique) = stack.pop() {
                let last = *clique.last().unwrap();

                if clique.len() > 2 {
                    let mut found_clique = clique.clone();
                    found_clique.sort();
                    cliques.insert(found_clique);
                }

                for neighbor in self.neighbours(&last) {
                    if !clique.contains(&neighbor)
                        && clique
                            .iter()
                            .all(|n| self.neighbours(n).contains(&neighbor))
                    {
                        let mut new_clique = clique.clone();
                        new_clique.push(neighbor);
                        stack.push(new_clique);
                    }
                }
            }
        }

        cliques
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //     1
    //    / \
    //   2---3
    //       |
    //       4
    fn test_cycles() {
        let mut graph = Graph::undirected();
        graph.add_edge(2, 3);
        graph.add_edge(1, 2);
        graph.add_edge(3, 1);
        graph.add_edge(3, 4);

        let cycles = graph.cycles();

        assert_eq!(cycles.len(), 1);
        assert!(cycles.contains(&vec![1, 2, 3]));
    }

    #[test]
    //     2
    //    / \
    //   1---3
    //   |   |
    //   5---4
    //    \ /
    //     6
    fn test_multiple_cycles() {
        let mut graph = Graph::undirected();
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 1);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);
        graph.add_edge(5, 6);
        graph.add_edge(6, 4);
        graph.add_edge(1, 5);

        let cycles = graph.cycles();

        assert_eq!(cycles.len(), 6);
        assert!(cycles.contains(&vec![1, 2, 3]));
        assert!(cycles.contains(&vec![4, 5, 6]));
        assert!(cycles.contains(&vec![1, 3, 4, 5]));
        assert!(cycles.contains(&vec![1, 3, 4, 5, 6]));
        assert!(cycles.contains(&vec![1, 2, 3, 4, 5]));
        assert!(cycles.contains(&vec![1, 2, 3, 4, 5, 6]));
    }

    #[test]
    //     2
    //    / \
    //   1---3
    //   | x |
    //   5---4
    //    \ /
    //     6
    fn test_clique() {
        let mut graph = Graph::undirected();
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 1);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);
        graph.add_edge(5, 6);
        graph.add_edge(6, 4);
        graph.add_edge(1, 5);
        graph.add_edge(1, 4);
        graph.add_edge(3, 5);

        let cliques = graph.cliques();

        assert_eq!(cliques.len(), 7);
        assert!(cliques.contains(&vec![1, 2, 3]));
        assert!(cliques.contains(&vec![4, 5, 6]));
        assert!(cliques.contains(&vec![1, 3, 5]));
        assert!(cliques.contains(&vec![1, 4, 5]));
        assert!(cliques.contains(&vec![3, 4, 5]));
        assert!(cliques.contains(&vec![1, 3, 4]));
        assert!(cliques.contains(&vec![1, 3, 4, 5]));
    }

    #[test]
    //     2
    //    / \
    //   1---3
    //   |   |
    //   5---4
    //    \ /
    //     6
    fn test_clique_2() {
        let mut graph = Graph::undirected();
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 1);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);
        graph.add_edge(5, 6);
        graph.add_edge(6, 4);
        graph.add_edge(1, 5);

        let cliques = graph.cliques();

        assert_eq!(cliques.len(), 2);
        assert!(cliques.contains(&vec![1, 2, 3]));
        assert!(cliques.contains(&vec![4, 5, 6]));
    }
}
