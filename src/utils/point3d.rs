use itertools::Itertools;
use std::fmt::{Display, Formatter};

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
