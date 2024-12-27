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
        T: Eq + Hash + Clone,
    {
        self.neighbours.get(node).unwrap_or(&Vec::new()).to_vec()
    }
}
