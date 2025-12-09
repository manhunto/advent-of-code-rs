use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul, Sub};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Point3D {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Point3D {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    pub fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z - 1,
        }
    }

    pub fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z + 1,
        }
    }

    pub fn distance(&self, other: &Self) -> f64 {
        let diff = *self - *other;

        ((diff.x.abs().pow(2) + diff.y.abs().pow(2) + diff.z.abs().pow(2)) as f64).sqrt()
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

impl FromStr for Point3D {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl Add<Point3D> for Point3D {
    type Output = Self;

    fn add(self, rhs: Point3D) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Mul<isize> for Point3D {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Sub<Point3D> for Point3D {
    type Output = Self;

    fn sub(self, rhs: Point3D) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Display for Point3D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let p1 = Point3D::new(1, 2, 3);
        let p2 = Point3D::new(4, 5, 6);

        assert_eq!(p1.distance(&p2), 5.196152422706632);
    }

    #[test]
    fn test_distance_to_self() {
        let p1 = Point3D::new(1, 2, 3);

        assert_eq!(p1.distance(&p1), 0.0);
    }

    #[test]
    fn test_distance_along_single_axis() {
        let p1 = Point3D::new(1, 2, 3);
        let p2 = Point3D::new(1, 2, 8);

        assert_eq!(5.0, p1.distance(&p2));
    }

    #[test]
    fn test_distance_from_origin() {
        let p1 = Point3D::new(3, 4, 0);
        let origin = Point3D::new(0, 0, 0);

        assert_eq!(5.0, p1.distance(&origin));
    }
}
