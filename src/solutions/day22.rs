use crate::solutions::Solution;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

pub struct Day22;

impl Solution for Day22 {
    fn part_one(&self, input: &str) -> String {
        let bricks: Bricks = Self::parse_input(input);

        bricks
            .bricks_by_lowest_z_asc()
            .iter()
            .fold(0, |sum, brick| {
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
                    return sum + 1;
                }

                sum
            })
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let bricks: Bricks = Self::parse_input(input);

        bricks
            .bricks_by_highest_z_desc()
            .iter()
            .fold(0, |sum, b| {
                sum + Self::fall(&bricks, vec![b.clone()], &mut HashSet::new())
            })
            .to_string()
    }
}

impl Day22 {
    fn parse_input(input: &str) -> Bricks {
        let bricks: Vec<Brick> = input.lines().map(Brick::from).collect();

        Self::settle_down(bricks)
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

    fn fall(bricks: &Bricks, removed_bricks: Vec<Brick>, history: &mut HashSet<Brick>) -> isize {
        let bricks_directly_above: HashSet<Brick> = removed_bricks
            .iter()
            .flat_map(|b| {
                bricks
                    .in_z(b.highest_z() + 1)
                    .into_iter()
                    .filter(|b| removed_bricks.iter().any(|rb| rb.up().collide(b)))
            })
            .collect();

        let in_removed_rows: Vec<Brick> = removed_bricks
            .iter()
            .flat_map(|b| bricks.in_z(b.highest_z()))
            .filter(|b| !removed_bricks.contains(b))
            .collect();

        let mut chain_fall: Vec<Brick> = Vec::new();
        let mut not_fall: Vec<Brick> = Vec::new();
        for above in bricks_directly_above {
            if in_removed_rows.is_empty()
                || in_removed_rows.iter().all(|b| !b.collide(&above.down()))
            {
                chain_fall.push(above.clone());
                history.insert(above.clone());
            } else {
                not_fall.push(above.clone())
            }
        }

        if chain_fall.is_empty() {
            return 0;
        }

        let tmp = HashSet::from_iter(chain_fall.to_vec());
        let tmp: Vec<&Brick> = history.difference(&tmp).collect();

        let next_removed: Vec<Brick> = chain_fall.clone().into_iter().chain(not_fall).collect();
        let result = tmp.len() as isize + Self::fall(bricks, next_removed.clone(), history);

        history.len() as isize
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
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

    fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z + 1,
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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

    fn up(&self) -> Self {
        Self::new(self.from.up(), self.to.up())
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
        let Point3D {
            x: fx,
            y: fy,
            z: fz,
        } = self.from;

        let Point3D {
            x: tx,
            y: ty,
            z: tz,
        } = self.to;

        write!(f, "{fx},{fy},{fz}~{tx},{ty},{tz}")
    }
}

struct Bricks {
    bricks: Vec<Brick>,
    bricks_in_row: HashMap<isize, HashSet<Brick>>,
}

impl Bricks {
    fn new(bricks: Vec<Brick>) -> Self {
        let mut bricks_in_row: HashMap<isize, HashSet<Brick>> = HashMap::new();

        for brick in &bricks {
            for point in &brick.points {
                bricks_in_row
                    .entry(point.z)
                    .or_default()
                    .insert(brick.clone());
            }
        }

        Self {
            bricks,
            bricks_in_row,
        }
    }

    fn in_z(&self, z: isize) -> HashSet<Brick> {
        self.bricks_in_row
            .get(&z)
            .unwrap_or(&HashSet::new())
            .clone()
    }

    fn bricks_by_lowest_z_asc(&self) -> Vec<Brick> {
        self.bricks
            .clone()
            .into_iter()
            .sorted_by(|a, b| a.lowest_z().cmp(&b.lowest_z()))
            .collect()
    }

    fn bricks_by_highest_z_desc(&self) -> Vec<Brick> {
        self.bricks
            .clone()
            .into_iter()
            .sorted_by(|a, b| a.highest_z().cmp(&b.highest_z()).reverse())
            .collect()
    }

    #[cfg(test)]
    fn push_brick(&self, brick: Brick) -> Self {
        let mut new_bricks = self.bricks.clone();
        new_bricks.push(brick.clone());

        Bricks::new(new_bricks)
    }
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day22::{Brick, Bricks, Day22};
    use crate::solutions::Solution;
    use std::collections::HashSet;

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

    #[test]
    fn fall() {
        // a is on the ground
        let a = Brick::from("1,0,0~1,2,0");
        // b and c is on a
        let b = Brick::from("0,0,1~2,0,1");
        assert!(b.down().collide(&a));
        let c = Brick::from("0,2,1~2,2,1");
        assert!(c.down().collide(&a));
        // d is on b
        let d = Brick::from("0,0,2~2,0,2");
        assert!(d.down().collide(&b));
        // e is on c
        let e = Brick::from("0,2,2~2,2,2");
        assert!(e.down().collide(&c));

        let bricks = Bricks::new(vec![a.clone(), b.clone(), c.clone(), d.clone(), e.clone()]);

        assert_eq!(
            4,
            Day22::fall(&bricks, vec![a.clone()], &mut HashSet::new())
        );
        assert_eq!(
            1,
            Day22::fall(&bricks, vec![b.clone()], &mut HashSet::new())
        );
        assert_eq!(
            1,
            Day22::fall(&bricks, vec![c.clone()], &mut HashSet::new())
        );
        assert_eq!(
            0,
            Day22::fall(&bricks, vec![d.clone()], &mut HashSet::new())
        );
        assert_eq!(
            0,
            Day22::fall(&bricks, vec![e.clone()], &mut HashSet::new())
        );

        // ------------

        // horz is on d and e
        let horz = Brick::from("1,0,3~1,2,3");
        assert!(horz.down().collide(&d));
        assert!(horz.down().collide(&e));

        let bricks = bricks.push_brick(horz.clone());

        assert_eq!(
            1,
            Day22::fall(&bricks, vec![b.clone()], &mut HashSet::new())
        );
        assert_eq!(
            1,
            Day22::fall(&bricks, vec![c.clone()], &mut HashSet::new())
        );
        assert_eq!(
            0,
            Day22::fall(&bricks, vec![d.clone()], &mut HashSet::new())
        );
        assert_eq!(
            0,
            Day22::fall(&bricks, vec![e.clone()], &mut HashSet::new())
        );
        assert_eq!(
            0,
            Day22::fall(&bricks, vec![horz.clone()], &mut HashSet::new())
        );
        assert_eq!(
            5,
            Day22::fall(&bricks, vec![a.clone()], &mut HashSet::new())
        );

        // ------------

        // horz2 is on horz
        let horz2 = Brick::from("1,0,4~1,2,4");
        assert!(horz2.down().collide(&horz));

        let bricks = bricks.push_brick(horz2.clone());

        assert_eq!(
            1,
            Day22::fall(&bricks, vec![b.clone()], &mut HashSet::new())
        );
        assert_eq!(
            1,
            Day22::fall(&bricks, vec![c.clone()], &mut HashSet::new())
        );
        assert_eq!(
            0,
            Day22::fall(&bricks, vec![d.clone()], &mut HashSet::new())
        );
        assert_eq!(
            0,
            Day22::fall(&bricks, vec![e.clone()], &mut HashSet::new())
        );
        assert_eq!(
            1,
            Day22::fall(&bricks, vec![horz.clone()], &mut HashSet::new())
        );
        assert_eq!(
            0,
            Day22::fall(&bricks, vec![horz2.clone()], &mut HashSet::new())
        );
        assert_eq!(
            6,
            Day22::fall(&bricks, vec![a.clone()], &mut HashSet::new())
        );
    }

    #[test]
    fn fall_edge_case_with_two_cubes() {
        let horz2 = Brick::from("1,0,1~1,2,1");

        // stick is on horz
        let stick = Brick::from("1,0,2~1,0,3");
        assert!(stick.down().collide(&horz2));

        // cubes are on horz2 too
        let cube1 = Brick::from("1,2,2~1,2,2");
        let cube2 = Brick::from("1,2,3~1,2,3");
        assert!(cube1.down().collide(&horz2));
        assert!(cube2.down().collide(&cube1));
        assert!(!cube1.collide(&stick));
        assert!(!cube2.collide(&stick));

        // horz3 is on cube2 and stick
        let horz3 = Brick::from("1,0,4~1,2,4");
        assert!(horz3.down().collide(&cube2));
        assert!(horz3.down().collide(&stick));

        let bricks = Bricks::new(vec![
            stick.clone(),
            cube1.clone(),
            cube2.clone(),
            horz3.clone(),
            horz2.clone(),
        ]);

        assert_eq!(4, Day22::fall(&bricks, vec![horz2], &mut HashSet::new()));
        assert_eq!(1, Day22::fall(&bricks, vec![cube1], &mut HashSet::new()));
        assert_eq!(0, Day22::fall(&bricks, vec![cube2], &mut HashSet::new()));
        assert_eq!(0, Day22::fall(&bricks, vec![stick], &mut HashSet::new()));
    }

    // https://www.reddit.com/r/adventofcode/comments/18p4wlj/comment/kemjie2/
    #[test]
    fn fall_reddit_comment() {
        let a = Brick::from("0,0,1~0,5,1");
        let b = Brick::from("0,6,1~0,9,1");
        let c = Brick::from("0,0,2~0,0,2");
        let d = Brick::from("0,3,2~0,8,2");

        let bricks = Bricks::new(vec![a.clone(), b.clone(), c.clone(), d.clone()]);

        assert_eq!(1, Day22::fall(&bricks, vec![a], &mut HashSet::new()));
        assert_eq!(0, Day22::fall(&bricks, vec![b], &mut HashSet::new()));
        assert_eq!(0, Day22::fall(&bricks, vec![c], &mut HashSet::new()));
        assert_eq!(0, Day22::fall(&bricks, vec![d], &mut HashSet::new()));
    }
}
