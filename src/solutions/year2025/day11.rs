use crate::solutions::Solution;
use crate::utils::graphs::all_paths::AllPaths;
use crate::utils::graphs::graph::Graph;

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

        all_paths.count_paths(LABEL_YOU, LABEL_OUT).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let graph = self.parse(input);
        let all_paths: AllPaths<&str> = (&graph).into();

        let svr_dac = all_paths.count_paths(LABEL_SVR, LABEL_DAC);
        let dac_fft = all_paths.count_paths(LABEL_DAC, LABEL_FFT);
        let fft_out = all_paths.count_paths(LABEL_FFT, LABEL_OUT);
        let svr_dac_fft_out = svr_dac * dac_fft * fft_out;

        let svr_fft = all_paths.count_paths(LABEL_SVR, LABEL_FFT);
        let fft_dac = all_paths.count_paths(LABEL_FFT, LABEL_DAC);
        let dac_out = all_paths.count_paths(LABEL_DAC, LABEL_OUT);
        let scr_fft_dac_out = svr_fft * fft_dac * dac_out;

        (svr_dac_fft_out + scr_fft_dac_out).to_string()
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
