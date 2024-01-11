use crate::pair_generator::pairs;
use crate::point::Point;
use crate::solutions::Solution;
use crate::utils::line::Line;
use crate::utils::point3d::Point3D;
use crate::utils::surface_range::SurfaceRange;
use itertools::Itertools;

pub struct Day24;

impl Solution for Day24 {
    fn part_one(&self, input: &str) -> String {
        Self::solve_part_one(input, 200000000000000, 400000000000000)
    }

    fn part_two(&self, input: &str) -> String {
        let hails = Self::parse(input);

        println!("{} {}", hails.first().unwrap().in_time(5), 5);
        println!("{} {}", hails.get(1).unwrap().in_time(3), 3);
        // for hail in hails {
        //     println!("{:?}", hail);
        // }

        String::from('0')
    }
}

impl Day24 {
    fn parse(input: &str) -> Vec<Hail> {
        input
            .lines()
            .map(|line| {
                let (pos, vel) = line.split_terminator(" @ ").collect_tuple().unwrap();

                Hail::new(Self::parse_point(pos), Self::parse_point(vel))
            })
            .collect()
    }

    fn parse_point(str: &str) -> Point3D {
        Point3D::from(str.replace(' ', "").as_str())
    }

    fn solve_part_one(input: &str, from: isize, to: isize) -> String {
        let hails: Vec<Hail2D> = Self::parse(input).into_iter().map(Hail2D::from).collect();
        let pairs = pairs(hails);

        let surface = SurfaceRange::from_points(from, to, from, to);

        pairs
            .iter()
            .filter(|(a, b)| {
                a.line().intersect(&b.line()).is_some_and(|result| {
                    surface.contains(result) && !a.is_past(result) && !b.is_past(result)
                })
            })
            .collect::<Vec<_>>()
            .len()
            .to_string()
    }
}

#[derive(Debug, Clone)]
struct Hail {
    position: Point3D,
    velocity: Point3D,
}

impl Hail {
    fn new(position: Point3D, velocity: Point3D) -> Self {
        Self { position, velocity }
    }

    fn in_time(&self, time: isize) -> Point3D {
        self.position + self.velocity * time
    }
}

#[derive(Debug, Clone, Copy)]
struct Hail2D {
    position: Point,
    velocity: Point,
}

impl Hail2D {
    fn new(position: Point, velocity: Point) -> Self {
        Self { position, velocity }
    }

    fn line(&self) -> Line {
        Line::new(self.position, self.position + self.velocity)
    }

    fn is_past(&self, point: Point) -> bool {
        let next = self.position + self.velocity;
        let to_start = self.position.manhattan_distance(&point);
        let to_next = next.manhattan_distance(&point);

        to_next > to_start
    }
}

impl From<Hail> for Hail2D {
    fn from(value: Hail) -> Self {
        let pos = value.position;
        let vel = value.velocity;

        Self::new(Point::new(pos.x, pos.y), Point::new(vel.x, vel.y))
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day24::Day24;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("24");

        assert_eq!("2", Day24::solve_part_one(input.as_str(), 7, 27));
    }

    #[test]
    fn part_two_example_test() {
        let input = read_example("24");

        assert_eq!("47", Day24.part_two(input.as_str()));
    }
}
