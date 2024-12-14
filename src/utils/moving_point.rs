use crate::utils::point::Point;

#[derive(Debug)]
pub struct MovingPoint {
    position: Point,
    velocity: Point,
}

impl MovingPoint {
    pub fn position(&self) -> Point {
        self.position
    }

    pub fn velocity(&self) -> Point {
        self.velocity
    }

    pub fn with_position(&self, position: Point) -> Self {
        Self {
            position,
            velocity: self.velocity,
        }
    }
}

impl From<(Point, Point)> for MovingPoint {
    fn from(value: (Point, Point)) -> Self {
        Self {
            position: value.0,
            velocity: value.1,
        }
    }
}
