use crate::solutions::Solution;
use std::collections::{HashMap, HashSet};

pub struct Day23;

impl Solution for Day23 {
    fn part_one(&self, input: &str) -> String {
        let mut neighbours: HashMap<&str, Vec<&str>> = HashMap::new();

        let connections: Vec<(&str, &str)> = input
            .lines()
            .map(|line| {
                let (a, b) = line.split_once('-').unwrap();

                neighbours.entry(a).or_default().push(b);
                neighbours.entry(b).or_default().push(a);

                (a, b)
            })
            .collect();

        let mut sets: HashSet<[&str; 3]> = HashSet::new();

        connections.iter().for_each(|(a, b)| {
            let a_neighbours = neighbours.get(a).unwrap();
            let b_neighbours = neighbours.get(b).unwrap();

            let intersection: Vec<_> = a_neighbours
                .iter()
                .filter(|x| b_neighbours.contains(x))
                .collect();

            intersection.iter().for_each(|c| {
                let mut set = [*a, *b, *c];
                set.sort();
                sets.insert(set);
            });
        });

        sets.iter()
            .filter(|set| set.iter().any(|c| c.starts_with("t")))
            .count()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
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
}
