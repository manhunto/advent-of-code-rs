use crate::solutions::Solution;
use regex::Regex;

pub struct Day03;

const MUL_REGEX: &str = r"mul\((\d{1,3}),(\d{1,3})\)";
const DO_REGEX: &str = r"do\(\)";
const DONT_REGEX: &str = r"don't\(\)";

impl Solution for Day03 {
    fn part_one(&self, input: &str) -> String {
        let re = Regex::new(MUL_REGEX).unwrap();

        re.captures_iter(input)
            .map(|capture| {
                let left = capture[1].parse::<usize>().unwrap();
                let right = capture[2].parse::<usize>().unwrap();

                left * right
            })
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let dont_re = Regex::new(DONT_REGEX).unwrap();
        let do_re = Regex::new(DO_REGEX).unwrap();
        let re = Regex::new(MUL_REGEX).unwrap();

        let dont_positions: Vec<usize> = dont_re.find_iter(input).map(|m| m.end()).collect();
        let do_positions: Vec<usize> = do_re.find_iter(input).map(|m| m.end()).collect();

        re.captures_iter(input)
            .map(|capture| {
                let mul_position = capture.get(0).unwrap().end();

                let do_recent = self
                    .last_position_before(&do_positions, mul_position)
                    .unwrap_or(0);

                let dont_recent_opt = self.last_position_before(&dont_positions, mul_position);
                if dont_recent_opt.map_or(true, |dont_recent| dont_recent < do_recent) {
                    let left = capture[1].parse::<usize>().unwrap();
                    let right = capture[2].parse::<usize>().unwrap();

                    return left * right;
                }

                0
            })
            .sum::<usize>()
            .to_string()
    }
}

impl Day03 {
    fn last_position_before(&self, vec: &[usize], position: usize) -> Option<usize> {
        vec.iter().copied().filter(|x| *x < position).last()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day03::Day03;
    use crate::solutions::Solution;

    const EXAMPLE: &str =
        r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
    const EXAMPLE_TWO: &str =
        r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

    const MULTILINE_EXAMPLE: &str = r#"don't()mul(4,2)
    mul(3,5)
    do()mul(2,2)
    "#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("161", Day03.part_one(EXAMPLE));
    }

    #[test]
    fn part_two_example_test() {
        assert_eq!("48", Day03.part_two(EXAMPLE_TWO));
        assert_eq!("191274", Day03.part_two("?)do())mul(449,426)"));
        assert_eq!("0", Day03.part_two("?)do())don't()mul(2,3)"));
        assert_eq!("0", Day03.part_two("?)do())don't()3{}mul(2,3)"));
        assert_eq!("20", Day03.part_two("?)do()don't()do()mul(5,4)"));
        assert_eq!("24", Day03.part_two("?)do()mul(2,2)don't()do()mul(5,4)"));
        assert_eq!(
            "24",
            Day03.part_two("?)do()mul(2,2)don't()mul(3,2)do()mul(5,4)")
        );
        assert_eq!("20", Day03.part_two("?)mul(5,4)"));
        assert_eq!("4", Day03.part_two(MULTILINE_EXAMPLE));
    }
}
