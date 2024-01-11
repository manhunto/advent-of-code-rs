use crate::pair_generator::pairs;
use crate::point::Point;
use crate::solutions::Solution;
use crate::utils::point3d::Point3D;
use crate::utils::surface_range::SurfaceRange;
use itertools::Itertools;

pub struct Day24;

impl Solution for Day24 {
    fn part_one(&self, input: &str) -> String {
        Self::solve(input, 200000000000000, 400000000000000)
    }

    fn part_two(&self, _input: &str) -> String {
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

    fn solve(input: &str, from: isize, to: isize) -> String {
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

        Self::new(
            Point::new(pos.x as i32, pos.y as i32),
            Point::new(vel.x as i32, vel.y as i32),
        )
    }
}

#[derive(Debug, Copy, Clone)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    fn intersect(self, other: &Self) -> Option<Point> {
        let a = self.start;
        let b = self.end;
        let a1 = b.y - a.y;
        let b1 = a.x - b.x;
        let c1 = a1 * a.x + b1 * a.y;

        let c = other.start;
        let d = other.end;
        let a2 = d.y - c.y;
        let b2 = c.x - d.x;
        let c2 = a2 * c.x + b2 * c.y;

        let determinant = a1 * b2 - a2 * b1;

        if determinant == 0 {
            return None;
        }

        let x = (b2 as f64 * c1 as f64 - b1 as f64 * c2 as f64) / determinant as f64;
        let y = (a1 as f64 * c2 as f64 - a2 as f64 * c1 as f64) / determinant as f64;

        Some(Point::new(x as i32, y as i32))
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day24::Day24;

    #[test]
    fn part_one_example_test() {
        let input = read_example("24");

        assert_eq!("2", Day24::solve(input.as_str(), 7, 27));
    }
}
