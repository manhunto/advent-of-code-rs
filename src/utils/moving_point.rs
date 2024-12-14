use crate::utils::point::Point;

#[derive(Debug)]
pub struct MovingPoint {
    #[allow(dead_code)]
    position: Point,
    #[allow(dead_code)]
    velocity: Point,
}

impl From<(Point, Point)> for MovingPoint {
    fn from(value: (Point, Point)) -> Self {
        Self {
            position: value.0,
            velocity: value.1,
        }
    }
}
