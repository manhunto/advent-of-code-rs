use crate::solutions::Solution;
use crate::utils::point3d::Point3D;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};

pub struct Day22;

impl Solution for Day22 {
    fn part_one(&self, input: &str) -> String {
        let bricks: Bricks = Self::parse_input(input);
        let (supported_by, supporters) = Self::graphs(&bricks);

        bricks
            .bricks_by_lowest_z_asc()
            .iter()
            .filter(|brick| {
                supported_by.get(brick).unwrap().iter().all(|brick_above| {
                    let supporters: Vec<&Brick> = supporters
                        .get(brick_above)
                        .unwrap()
                        .iter()
                        .filter(|b| brick != b)
                        .collect();

                    !supporters.is_empty()
                })
            })
            .collect::<Vec<_>>()
            .len()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let bricks: Bricks = Self::parse_input(input);
        let (supported_by, supporters) = Self::graphs(&bricks);

        bricks
            .bricks_by_lowest_z_asc()
            .iter()
            .map(|b| Self::fall(&supported_by, &supporters, b.clone()))
            .sum::<isize>()
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

    fn graphs(bricks: &Bricks) -> (HashMap<Brick, Vec<Brick>>, HashMap<Brick, Vec<Brick>>) {
        // if remove brick (key) the another bricks will fall (values)
        let supported_by: HashMap<Brick, Vec<Brick>> = bricks
            .bricks_by_highest_z_desc()
            .iter()
            .map(|brick| {
                let above = bricks
                    .in_z(brick.highest_z() + 1)
                    .into_iter()
                    .filter(|b| brick.up().collide(b))
                    .collect();

                (brick.clone(), above)
            })
            .collect();

        // is brick (key) supported by another bricks (values)
        let mut supporters: HashMap<Brick, Vec<Brick>> = HashMap::new();
        for (brick, above) in &supported_by {
            for a in above {
                supporters.entry(a.clone()).or_default().push(brick.clone())
            }
        }

        (supported_by, supporters)
    }

    fn fall(
        supported_by: &HashMap<Brick, Vec<Brick>>,
        supporters: &HashMap<Brick, Vec<Brick>>,
        brick: Brick,
    ) -> isize {
        let mut to_remove: VecDeque<Brick> = VecDeque::new();
        to_remove.push_back(brick);

        let mut already_removed: Vec<Brick> = Vec::new();
        let mut count: isize = 0;

        while let Some(current) = to_remove.pop_front() {
            let next_to_fall = supported_by.get(&current).unwrap();
            already_removed.push(current.clone());

            for next in next_to_fall {
                let supporters: Vec<&Brick> = supporters
                    .get(next)
                    .unwrap()
                    .iter()
                    .filter(|b| !already_removed.contains(b))
                    .collect();

                if supporters.is_empty() {
                    to_remove.push_back(next.clone());
                    if !already_removed.contains(next) {
                        count += 1;
                    }
                }
            }
        }

        count
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
    use crate::solutions::year2023::day22::{Brick, Bricks, Day22};
    use crate::solutions::year2023::read_2023_example;
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_2023_example("22");

        assert_eq!("5", Day22.part_one(input.as_str()));
    }

    #[test]
    fn part_two_example_test() {
        let input = read_2023_example("22");

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
        let (g1, g2) = Day22::graphs(&bricks);

        assert_eq!(4, Day22::fall(&g1, &g2, a.clone()));
        assert_eq!(1, Day22::fall(&g1, &g2, b.clone()));
        assert_eq!(1, Day22::fall(&g1, &g2, c.clone()));
        assert_eq!(0, Day22::fall(&g1, &g2, d.clone()));
        assert_eq!(0, Day22::fall(&g1, &g2, e.clone()));

        // ------------

        // horz is on d and e
        let horz = Brick::from("1,0,3~1,2,3");
        assert!(horz.down().collide(&d));
        assert!(horz.down().collide(&e));

        let bricks = bricks.push_brick(horz.clone());
        let (g1, g2) = Day22::graphs(&bricks);

        assert_eq!(1, Day22::fall(&g1, &g2, b.clone()));
        assert_eq!(1, Day22::fall(&g1, &g2, c.clone()));
        assert_eq!(0, Day22::fall(&g1, &g2, d.clone()));
        assert_eq!(0, Day22::fall(&g1, &g2, e.clone()));
        assert_eq!(0, Day22::fall(&g1, &g2, horz.clone()));
        assert_eq!(5, Day22::fall(&g1, &g2, a.clone()));

        // ------------

        // horz2 is on horz
        let horz2 = Brick::from("1,0,4~1,2,4");
        assert!(horz2.down().collide(&horz));

        let bricks = bricks.push_brick(horz2.clone());
        let (g1, g2) = Day22::graphs(&bricks);

        assert_eq!(1, Day22::fall(&g1, &g2, b.clone()));
        assert_eq!(1, Day22::fall(&g1, &g2, c.clone()));
        assert_eq!(0, Day22::fall(&g1, &g2, d.clone()));
        assert_eq!(0, Day22::fall(&g1, &g2, e.clone()));
        assert_eq!(1, Day22::fall(&g1, &g2, horz.clone()));
        assert_eq!(0, Day22::fall(&g1, &g2, horz2.clone()));
        assert_eq!(6, Day22::fall(&g1, &g2, a.clone()));
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
        let (g1, g2) = Day22::graphs(&bricks);

        assert_eq!(4, Day22::fall(&g1, &g2, horz2));
        assert_eq!(1, Day22::fall(&g1, &g2, cube1));
        assert_eq!(0, Day22::fall(&g1, &g2, cube2));
        assert_eq!(0, Day22::fall(&g1, &g2, stick));
    }

    // https://www.reddit.com/r/adventofcode/comments/18p4wlj/comment/kemjie2/
    #[test]
    fn fall_reddit_comment() {
        let a = Brick::from("0,0,1~0,5,1");
        let b = Brick::from("0,6,1~0,9,1");
        let c = Brick::from("0,0,2~0,0,2");
        let d = Brick::from("0,3,2~0,8,2");

        let bricks = Bricks::new(vec![a.clone(), b.clone(), c.clone(), d.clone()]);
        let (g1, g2) = Day22::graphs(&bricks);

        assert_eq!(1, Day22::fall(&g1, &g2, a));
        assert_eq!(0, Day22::fall(&g1, &g2, b));
        assert_eq!(0, Day22::fall(&g1, &g2, c));
        assert_eq!(0, Day22::fall(&g1, &g2, d));
    }
}
