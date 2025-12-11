use crate::solutions::Solution;
use crate::utils::graphs::all_paths::AllPaths;
use crate::utils::graphs::graph::Graph;
use std::collections::VecDeque;

pub struct Day11;

const LABEL_YOU: &str = "you";
const LABEL_OUT: &str = "out";
const LABEL_SVR: &str = "svr";
const LABEL_DAC: &str = "dac";
const LABEL_FFT: &str = "fft";

impl Solution for Day11 {
    fn part_one(&self, input: &str) -> String {
        let graph = self.parse(input);
        let all_paths: AllPaths<&str> = (&graph).into();

        all_paths.paths(LABEL_YOU, LABEL_OUT).len().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let graph = self.parse(input);
        let all_paths: AllPaths<&str> = (&graph).into();

        let should_count_path =
            |path: &VecDeque<&str>| path.contains(&LABEL_DAC) && path.contains(&LABEL_FFT);

        all_paths
            .count_paths(LABEL_SVR, LABEL_OUT, should_count_path)
            .to_string()
    }
}

impl Day11 {
    fn parse<'a>(&self, input: &'a str) -> Graph<&'a str> {
        let mut graph = Graph::directed();

        input.lines().for_each(|line| {
            let (node, outputs) = line.split_once(": ").unwrap();
            let outputs_vec = outputs.split_whitespace();

            for output in outputs_vec {
                graph.add_edge(node, output);
            }
        });

        graph
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2025::day11::Day11;
    use crate::solutions::Solution;

    const EXAMPLE_PART_ONE: &str = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("5", Day11.part_one(EXAMPLE_PART_ONE));
    }

    const EXAMPLE_PART_TWO: &str = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#;

    #[test]
    fn part_two_example_test() {
        assert_eq!("2", Day11.part_two(EXAMPLE_PART_TWO));
    }
}
