use crate::solutions::Solution;
use crate::utils::filled_region::FilledRegion;
use crate::utils::point::Point;
use crate::utils::polygon::Polygon;
use crate::utils::surface_range::SurfaceRange;
use crate::utils::traits::IsInside;
use itertools::Itertools;

pub struct Day09;

impl Solution for Day09 {
    fn part_one(&self, input: &str) -> String {
        self.parse(input)
            .tuple_combinations()
            .map(|(a, b)| SurfaceRange::from((a, b)).area())
            .max()
            .unwrap()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let points = self.parse(input);
        let region: FilledRegion = points.clone().collect::<Polygon>().into();

        points
            .tuple_combinations()
            .filter_map(|(a, b)| {
                let rectangle = Polygon::rectangle(a, b);
                if region.is_inside(&rectangle) {
                    return Some(SurfaceRange::from((a, b)).area());
                }

                None
            })
            .max()
            .unwrap()
            .to_string()
    }
}

impl Day09 {
    fn parse<'a>(&self, input: &'a str) -> impl Iterator<Item = Point> + Clone + 'a {
        input.lines().map(|line| line.parse().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2025::day09::Day09;
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("50", Day09.part_one(EXAMPLE));
    }

    #[test]
    fn part_two_example_test() {
        assert_eq!("24", Day09.part_two(EXAMPLE));
    }
}
