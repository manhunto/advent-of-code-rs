use crate::solutions::Solution;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub struct Day22;

impl Solution for Day22 {
    fn part_one(&self, input: &str) -> String {
        let bricks: Vec<Brick> = Self::parse_input(input);
        let bricks: Bricks = Self::settle_down(bricks);

        let mut disintegrated: isize = 0;

        for brick in &bricks.bricks {
            let bricks_in_row_above = bricks.in_z(brick.highest_z() + 1);
            let in_this_row: Vec<Brick> = bricks
                .in_z(brick.highest_z())
                .into_iter()
                .filter(|b| b != brick)
                .collect();

            if bricks_in_row_above
                .iter()
                .all(|above| in_this_row.iter().any(|b| b.collide(&above.down())))
            {
                disintegrated += 1;
            }
        }

        disintegrated.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let bricks: Vec<Brick> = Self::parse_input(input);
        let bricks: Bricks = Self::settle_down(bricks);

        bricks
            .bricks_frow_down()
            .iter()
            .fold(0, |sum, b| sum + Self::fall(&bricks, b))
            .to_string()
    }
}

impl Day22 {
    fn parse_input(input: &str) -> Vec<Brick> {
        input.lines().map(Brick::from).collect()
    }

    fn settle_down(bricks: Vec<Brick>) -> Bricks {
        let mut settled_down: Vec<Brick> = Vec::with_capacity(bricks.len());

        let bricks_from_down = bricks
            .iter()
            .sorted_by(|a, b| a.lowest_z().cmp(&b.lowest_z()));

        for brick in bricks_from_down {
            let mut brick = brick.clone();
            let bricks_below = settled_down.iter().rev().take(50).collect::<Vec<&Brick>>();

            loop {
                let brick_below = brick.down();
                if bricks_below.iter().any(|b| b.collide(&brick_below))
                    || brick_below.lowest_z() == 0
                {
                    settled_down.push(brick.clone());
                    break;
                }

                brick = brick_below;
            }
        }

        Bricks::new(settled_down)
    }

    fn fall(bricks: &Bricks, brick: &Brick) -> isize {
        let mut how_much_fail = 0;
        let bricks_in_row_above = bricks.in_z(brick.highest_z() + 1);
        let in_this_row: Vec<Brick> = bricks
            .in_z(brick.highest_z())
            .into_iter()
            .filter(|b| b != brick)
            .collect();

        for above in bricks_in_row_above {
            if in_this_row.is_empty() || in_this_row.iter().all(|b| !b.collide(&above.down())) {
                how_much_fail += 1 + Self::fall(bricks, &above);
            }
        }

        how_much_fail
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Point3D {
    x: isize,
    y: isize,
    z: isize,
}

impl Point3D {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z - 1,
        }
    }
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
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Brick {
    from: Point3D,
    to: Point3D,
    points: Vec<Point3D>,
}

impl Brick {
    fn new(from: Point3D, to: Point3D) -> Self {
        let mut points: Vec<Point3D> = Vec::new();
        for x in from.x.min(to.x)..=from.x.max(to.x) {
            for y in from.y.min(to.y)..=from.y.max(to.y) {
                for z in from.z.min(to.z)..=from.z.max(to.z) {
                    points.push(Point3D::new(x, y, z));
                }
            }
        }

        Self { from, to, points }
    }

    fn lowest_z(&self) -> isize {
        self.from.z.min(self.to.z)
    }

    fn highest_z(&self) -> isize {
        self.from.z.max(self.to.z)
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.points.len()
    }

    fn down(&self) -> Self {
        Self::new(self.from.down(), self.to.down())
    }

    fn collide(&self, other: &Self) -> bool {
        self.points.iter().any(|p| other.points.contains(p))
    }
}

impl From<&str> for Brick {
    fn from(value: &str) -> Self {
        let (left, right) = value.split_terminator('~').collect_tuple().unwrap();

        Self::new(Point3D::from(left), Point3D::from(right))
    }
}

impl Display for Brick {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.from, self.to)
    }
}

struct Bricks {
    bricks: Vec<Brick>,
    bricks_in_row: HashMap<isize, Vec<Brick>>,
}

impl Bricks {
    fn new(bricks: Vec<Brick>) -> Self {
        let mut bricks_in_row: HashMap<isize, Vec<Brick>> = HashMap::new();

        for brick in &bricks {
            for point in &brick.points {
                bricks_in_row
                    .entry(point.z)
                    .or_default()
                    .push(brick.clone())
            }
        }

        Self {
            bricks,
            bricks_in_row,
        }
    }

    fn in_z(&self, z: isize) -> Vec<Brick> {
        self.bricks_in_row.get(&z).unwrap_or(&Vec::new()).to_vec()
    }

    fn bricks_frow_down(&self) -> Vec<Brick> {
        self.bricks
            .clone()
            .into_iter()
            .sorted_by(|a, b| a.lowest_z().cmp(&b.lowest_z()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day22::{Brick, Day22};
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("22");

        assert_eq!("5", Day22.part_one(input.as_str()));
    }

    #[test]
    fn part_two_example_test() {
        let input = read_example("22");

        assert_eq!("7", Day22.part_two(input.as_str()));
    }

    #[test]
    fn brick_len_test() {
        assert_eq!(1, Brick::from("2,2,2~2,2,2").len());
        assert_eq!(2, Brick::from("0,0,10~1,0,10").len());
        assert_eq!(2, Brick::from("0,0,10~0,1,10").len());
        assert_eq!(10, Brick::from("0,0,1~0,0,10").len());
    }
}
