use crate::direction::Direction::{East, North, South, West};
use crate::point::Point;
use crate::range::Range;
use crate::utils::vector::Vector;

#[derive(Copy, Clone)]
pub struct SurfaceRange {
    x_range: Range,
    y_range: Range,
}

impl SurfaceRange {
    pub fn new(x_range: Range, y_range: Range) -> Self {
        Self { x_range, y_range }
    }

    pub fn from_points(ax: isize, ay: isize, bx: isize, by: isize) -> Self {
        Self::new(
            Range::new(ax, ay).unwrap(),
            Range::new(bx, by).unwrap(),
        )
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
        self.x_range.is_in_range(point.x) && self.y_range.is_in_range(point.y)
    }

    #[cfg(test)]
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
            vectors.push(Vector::new(
                Point::new(x, rows.end()),
                North,
            ));
        }

        for y in rows.iter() {
            vectors.push(Vector::new(Point::new(0, y), East));
            vectors.push(Vector::new(
                Point::new(columns.end(), y),
                West,
            ));
        }

        vectors
    }

    pub fn top_left_corner(&self) -> Point {
        Point::new(self.x_range.start(), self.y_range.start())
    }

    pub fn bottom_right_corner(&self) -> Point {
        Point::new(self.x_range.end(), self.y_range.end())
    }
}
