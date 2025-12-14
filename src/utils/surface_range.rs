use crate::utils::direction::Direction::{East, North, South, West};
use crate::utils::point::Point;
use crate::utils::range::Range;
use crate::utils::vector::Vector;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct SurfaceRange {
    x_range: Range,
    y_range: Range,
}

impl SurfaceRange {
    pub fn new(x_range: Range, y_range: Range) -> Self {
        Self { x_range, y_range }
    }

    pub fn from_points(start_x: isize, end_x: isize, start_y: isize, end_y: isize) -> Self {
        Self::new(
            Range::from_unordered(start_x, end_x),
            Range::from_unordered(start_y, end_y),
        )
    }

    pub fn square(size: isize) -> Self {
        Self::from_points(0, size, 0, size)
    }

    pub fn x(&self) -> Range {
        self.x_range
    }

    pub fn columns(&self) -> Range {
        self.x()
    }

    pub fn y(&self) -> Range {
        self.y_range
    }

    pub fn rows(&self) -> Range {
        self.y()
    }

    pub fn contains(&self, point: Point) -> bool {
        self.x_range.contains(point.x) && self.y_range.contains(point.y)
    }

    pub fn area(&self) -> usize {
        (self.x_range.len() * self.y_range.len()) as usize
    }

    pub fn perimeter(&self) -> usize {
        (self.x_range.len() * 2 + self.y_range.len() * 2) as usize
    }

    pub fn vectors_pointing_inwards(&self) -> Vec<Vector> {
        let mut vectors: Vec<Vector> = Vec::with_capacity(self.perimeter());
        let columns = self.columns();
        let rows = self.rows();

        for x in columns.iter() {
            vectors.push(Vector::new(Point::new(x, 0), South));
            vectors.push(Vector::new(Point::new(x, rows.end()), North));
        }

        for y in rows.iter() {
            vectors.push(Vector::new(Point::new(0, y), East));
            vectors.push(Vector::new(Point::new(columns.end(), y), West));
        }

        vectors
    }

    pub fn top_left_corner(&self) -> Point {
        Point::new(self.x_range.start(), self.y_range.start())
    }

    pub fn bottom_right_corner(&self) -> Point {
        Point::new(self.x_range.end(), self.y_range.end())
    }

    pub fn _shrink(&self, by: isize) -> Self {
        Self {
            x_range: self.x_range._shrink(by).unwrap(),
            y_range: self.y_range._shrink(by).unwrap(),
        }
    }

    pub fn points(&self) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::with_capacity(self.area());

        for x in self.columns().iter() {
            for y in self.rows().iter() {
                points.push(Point::new(x, y));
            }
        }

        points
    }
}

impl From<(Point, Point)> for SurfaceRange {
    fn from((a, b): (Point, Point)) -> Self {
        Self::from_points(a.x, b.x, a.y, b.y)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::surface_range::SurfaceRange;

    #[test]
    fn shrink() {
        let surface_range = SurfaceRange::square(10);
        let shrunk = surface_range._shrink(1);

        let expected = SurfaceRange::from_points(1, 9, 1, 9);
        assert_eq!(expected, shrunk);
    }
}
