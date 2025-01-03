use crate::solutions::Solution;
use crate::utils::graphs::graph::Graph;
use itertools::Itertools;

pub struct Day23;

impl Solution for Day23 {
    fn part_one(&self, input: &str) -> String {
        let graph = self.parse(input);

        graph
            .edges()
            .iter()
            .flat_map(|(a, b)| {
                let a_neighbours = graph.neighbours(a);
                let b_neighbours = graph.neighbours(b);

                a_neighbours
                    .iter()
                    .filter(|x| b_neighbours.contains(x))
                    .map(|c| {
                        let mut set = [*a, *b, *c];
                        set.sort();
                        set
                    })
                    .collect::<Vec<[&str; 3]>>()
            })
            .unique()
            .filter(|set| set.iter().any(|c| c.starts_with("t")))
            .count()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let cliques = self.parse(input).cliques();
        let cliques = cliques.iter().sorted_by_key(|cycle| cycle.len());

        // println!("{:?}", &cliques);

        cliques.last().unwrap().join(",")
    }
}

impl Day23 {
    fn parse<'a>(&self, input: &'a str) -> Graph<&'a str> {
        let mut graph: Graph<&str> = Graph::undirected();

        input.lines().for_each(|line| {
            let (a, b) = line.split_once('-').unwrap();

            graph.add_edge(a, b);
        });

        graph
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day23::Day23;
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#;

    #[test]
    fn part_one_example() {
        assert_eq!("7", Day23.part_one(EXAMPLE));
    }

    #[test]
    fn part_two_example() {
        assert_eq!("co,de,ka,ta", Day23.part_two(EXAMPLE));
    }
}
