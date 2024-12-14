use crate::solutions::Solution;
use crate::utils::moving_point::MovingPoint;
use crate::utils::point::Point;
use itertools::Itertools;

pub struct Day14;

impl Solution for Day14 {
    fn part_one(&self, input: &str) -> String {
        let robots: Vec<MovingPoint> = input
            .lines()
            .map(|s| {
                s.split_whitespace()
                    .collect_tuple()
                    .map(|(p, v)| {
                        let position: Point = p.trim_start_matches("p=").parse().unwrap();
                        let velocity: Point = v.trim_start_matches("v=").parse().unwrap();

                        MovingPoint::from((position, velocity))
                    })
                    .unwrap()
            })
            .collect();

        println!("{:?}", robots);

        String::from('0')
    }

    fn part_two(&self, _input: &str) -> String {
        String::from('0')
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day14::Day14;
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("0", Day14.part_one(EXAMPLE));
    }
}
