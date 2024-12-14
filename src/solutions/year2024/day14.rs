use crate::solutions::Solution;
use crate::utils::moving_point::MovingPoint;
use crate::utils::point::Point;
use crate::utils::surface_range::SurfaceRange;
use itertools::Itertools;
use std::collections::HashSet;
use std::ops::Neg;

pub struct Day14 {
    surface: SurfaceRange,
}

impl Solution for Day14 {
    fn part_one(&self, input: &str) -> String {
        let mut robots = self.parse(input);

        for _ in 0..100 {
            robots = self.move_all(robots);
        }

        let start_x = self.surface.x().start();
        let end_x = self.surface.x().end();
        let middle_x = self.surface.x().end() / 2;
        let start_y = self.surface.y().start();
        let end_y = self.surface.y().end();
        let middle_y = self.surface.y().end() / 2;

        let surface1 = SurfaceRange::from_points(start_x, middle_x - 1, start_y, middle_y - 1);
        let surface2 = SurfaceRange::from_points(middle_x + 1, end_x, start_y, middle_y - 1);
        let surface3 = SurfaceRange::from_points(start_x, middle_x - 1, middle_y + 1, end_y);
        let surface4 = SurfaceRange::from_points(middle_x + 1, end_x, middle_y + 1, end_y);

        [surface1, surface2, surface3, surface4]
            .iter()
            .map(|surface| {
                robots
                    .iter()
                    .filter(|robot| surface.contains(robot.position()))
                    .count()
            })
            .product::<usize>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut robots = self.parse(input);

        let mut second = 0;
        loop {
            second += 1;
            robots = self.move_all(robots);

            let points = robots.iter().map(|robot| robot.position()).collect_vec();
            let points: HashSet<Point> = HashSet::from_iter(points);

            // when every robot is on unique position
            if points.len() == robots.len() {
                return second.to_string();
            }
        }
    }
}

impl Day14 {
    fn parse(&self, input: &str) -> Vec<MovingPoint> {
        input
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
            .collect()
    }

    fn move_robot(&self, moving_point: MovingPoint) -> MovingPoint {
        let mut next = moving_point.move_();

        let y = self.surface.y();
        if y.is_before(next.position().y) {
            let teleport = y.end() - next.position().y.neg() + 1;

            next = next.with_position_y(teleport);
        } else if y.is_after(next.position().y) {
            let teleport = y.start() + next.position().y - y.end() - 1;

            next = next.with_position_y(teleport);
        }

        let x = self.surface.x();
        if x.is_before(next.position().x) {
            let teleport = x.end() - next.position().x.neg() + 1;

            next = next.with_position_x(teleport);
        } else if x.is_after(next.position().x) {
            let teleport = x.start() + next.position().x - x.end() - 1;

            next = next.with_position_x(teleport);
        }

        next
    }

    fn move_all(&self, robots: Vec<MovingPoint>) -> Vec<MovingPoint> {
        robots.into_iter().map(|r| self.move_robot(r)).collect()
    }
}

impl Default for Day14 {
    fn default() -> Self {
        Self {
            surface: SurfaceRange::from_points(0, 101 - 1, 0, 103 - 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day14::Day14;
    use crate::solutions::Solution;
    use crate::utils::moving_point::MovingPoint;
    use crate::utils::point::Point;
    use crate::utils::surface_range::SurfaceRange;

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
        assert_eq!("12", solution().part_one(EXAMPLE));
    }

    #[test]
    fn move_robot_example() {
        let solution = solution();
        let robot = MovingPoint::from((Point::new(2, 4), Point::new(2, -3)));

        let robot = solution.move_robot(robot);
        assert_eq!(Point::new(4, 1), robot.position());

        let robot = solution.move_robot(robot);
        assert_eq!(Point::new(6, 5), robot.position());

        let robot = solution.move_robot(robot);
        assert_eq!(Point::new(8, 2), robot.position());

        let robot = solution.move_robot(robot);
        assert_eq!(Point::new(10, 6), robot.position());
    }

    fn solution() -> Day14 {
        Day14 {
            surface: SurfaceRange::from_points(0, 11 - 1, 0, 7 - 1),
        }
    }
}
