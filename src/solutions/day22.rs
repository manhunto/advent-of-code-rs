use std::fmt::{Display, Formatter};
use itertools::Itertools;
use crate::solutions::Solution;

pub struct Day22;

impl Solution for Day22 {
    fn part_one(&self, input: &str) -> String {
        let bricks: Vec<(Point3D, Point3D)> = input
            .lines()
            .map(|line| {
                let (left, right) = line.split_terminator('~').collect_tuple().unwrap();

                (Point3D::from(left), Point3D::from(right))
            })
            .collect();

        for (left, right) in bricks {
            println!("{} {}", left, right);
        }

        String::from('0')
    }

    fn part_two(&self, input: &str) -> String {
        String::from('0')    }
}


struct Point3D {
    x: isize,
    y: isize,
    z: isize,
}

impl From<&str> for Point3D {
    fn from(value: &str) -> Self {
        let (x, y, z) = value.split_terminator(',').collect_tuple().unwrap();

        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            z: z.parse().unwrap(),
        }
    }
}

impl Display for Point3D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day22::Day22;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("22");

        assert_eq!("5", Day22.part_one(input.as_str()));
    }
}
