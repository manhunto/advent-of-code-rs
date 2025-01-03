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

    fn edges(&self) -> &HashSet<(T, T)> {
        &self.edges
    }

    fn neighbours(&self, node: &T) -> Vec<T>
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

    pub fn cycles_3_elements(&self) -> HashSet<[T; 3]>
    where
        T: Eq + Hash + Copy + Ord,
    {
        self.edges()
            .iter()
            .flat_map(|(a, b)| {
                let a_neighbours = self.neighbours(a);
                let b_neighbours = self.neighbours(b);

                a_neighbours
                    .iter()
                    .filter(|x| b_neighbours.contains(x))
                    .map(|c| {
                        let mut set = [*a, *b, *c];
                        set.sort();
                        set
                    })
                    .collect::<Vec<[T; 3]>>()
            })
            .collect()
    }

    /// Clique - every node is connected to every other node
    /// A maximal clique is a clique that can't get any bigger (you can't add any more vertices while keeping it a clique)
    pub fn maximal_cliques(&self) -> HashSet<Vec<T>>
    where
        T: Eq + Hash + Copy + Ord,
    {
        let mut cliques = HashSet::new();
        let mut r = Vec::new();
        let mut p: HashSet<T> = self.nodes.clone();
        let mut x = HashSet::new();

        self.bron_kerbosch(&mut r, &mut p, &mut x, &mut cliques);

        cliques
    }

    fn bron_kerbosch(
        &self,
        r: &mut Vec<T>,
        p: &mut HashSet<T>,
        x: &mut HashSet<T>,
        cliques: &mut HashSet<Vec<T>>,
    ) where
        T: Eq + Hash + Copy + Ord,
    {
        if p.is_empty() && x.is_empty() {
            let mut clique = r.clone();
            clique.sort();
            cliques.insert(clique);
            return;
        }

        let pivot = p.union(x).next().unwrap();
        let p_without_neighbors = p
            .difference(&self.neighbours(pivot).into_iter().collect())
            .cloned()
            .collect::<HashSet<_>>();

        for v in p_without_neighbors.iter() {
            r.push(*v);
            let mut new_p = p
                .intersection(&self.neighbours(v).into_iter().collect())
                .cloned()
                .collect();
            let mut new_x = x
                .intersection(&self.neighbours(v).into_iter().collect())
                .cloned()
                .collect();

            self.bron_kerbosch(r, &mut new_p, &mut new_x, cliques);

            r.pop();
            p.remove(v);
            x.insert(*v);
        }
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
    fn test_maximal_cliques() {
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

        let cliques = graph.maximal_cliques();

        assert_eq!(cliques.len(), 3);
        assert!(cliques.contains(&vec![1, 2, 3]));
        assert!(cliques.contains(&vec![4, 5, 6]));
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
    fn test_maximal_cliques_2() {
        let mut graph = Graph::undirected();
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 1);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);
        graph.add_edge(5, 6);
        graph.add_edge(6, 4);
        graph.add_edge(1, 5);

        let cliques = graph.maximal_cliques();

        assert_eq!(cliques.len(), 4);
        assert!(cliques.contains(&vec![1, 5]));
        assert!(cliques.contains(&vec![3, 4]));
        assert!(cliques.contains(&vec![1, 2, 3]));
        assert!(cliques.contains(&vec![4, 5, 6]));
    }
}
