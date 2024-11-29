use crate::solutions::Solution;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Day25;

impl Solution for Day25 {
    fn part_one(&self, input: &str) -> String {
        let graph = Self::parse_input(input);
        let connections = graph.connections();

        for (to, connection) in graph.connections {
            println!("{} {:?}", to, connection);
        }

        let mut count = 0;
        //
        for a in 0..connections.len() {
            for b in a..connections.len() {
                for c in b..connections.len() {
                    let (_af, _at) = connections.get(a).unwrap();
                    let (_bf, _bt) = connections.get(b).unwrap();
                    let (_cf, _ct) = connections.get(c).unwrap();

                    count += 1;
                    //
                    //             let mut tmp_graph = graph.clone();
                    //             tmp_graph.remove_unidirectional_connection(af, at);
                    //             tmp_graph.remove_unidirectional_connection(bf, bt);
                    //             tmp_graph.remove_unidirectional_connection(cf, ct);
                    // println!("{:?} {:?} {:?}", first, second, third);
                }
            }
        }
        //
        println!("{}", count);

        String::from('0')
    }

    fn part_two(&self, _input: &str) -> String {
        String::from('0')
    }
}

impl Day25 {
    fn parse_input(input: &str) -> Graph {
        let mut graph: Graph = Graph::new();

        for line in input.lines() {
            let (from, destinations) = line.split_terminator(": ").collect_tuple().unwrap();

            for to in destinations.split_whitespace() {
                graph.add_unidirectional(from, to);
            }
        }

        graph
    }
}

#[derive(Clone)]
struct Graph {
    connections: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }

    fn add_unidirectional(&mut self, from: &str, to: &str) {
        self.add_directional(from, to);
        self.add_directional(to, from);
    }

    fn add_directional(&mut self, from: &str, to: &str) {
        debug_assert!(!self
            .connections
            .entry(from.to_string())
            .or_default()
            .contains(&to.to_string()));
        self.connections
            .entry(from.to_string())
            .or_default()
            .push(to.to_string());
    }

    #[allow(dead_code)]
    fn remove_unidirectional_connection(&mut self, from: &String, to: &String) {
        self.remove_directional_connection(from, to);
        self.remove_directional_connection(to, from);
    }

    #[allow(dead_code)]
    fn remove_directional_connection(&mut self, from: &String, to: &String) {
        let from_connections = self.connections.entry(from.to_string()).or_default();
        if let Some(index) = from_connections.iter().position(|f| f == to) {
            from_connections.remove(index);
        }
    }

    fn connections(&self) -> Vec<(String, String)> {
        let mut connections: HashSet<(String, String)> = HashSet::new();

        for (from, conn) in &self.connections {
            for to in conn {
                connections.insert((from.clone(), to.clone()));
            }
        }

        connections.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2023::read_2023_example;
    use crate::solutions::year2023::day25::Day25;
    use crate::solutions::Solution;

    #[ignore]
    #[test]
    fn part_one_example_test() {
        let input = read_2023_example("25");

        assert_eq!("54", Day25.part_one(input.as_str()));
    }
}
