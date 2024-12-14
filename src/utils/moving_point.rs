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

    pub fn move_(&self) -> MovingPoint {
        Self {
            position: self.position + self.velocity,
            velocity: self.velocity,
        }
    }

    pub fn with_position_y(&self, new_y: isize) -> Self {
        self.with_position(self.position().with_y(new_y))
    }

    pub fn with_position_x(&self, new_x: isize) -> Self {
        self.with_position(self.position().with_x(new_x))
    }

    fn with_position(&self, position: Point) -> Self {
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
