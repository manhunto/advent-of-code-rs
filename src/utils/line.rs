use crate::utils::point::Point;

#[derive(Debug, Copy, Clone)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    pub fn intersect(self, other: &Self) -> Option<Point> {
        let a = self.start;
        let b = self.end;
        let a1 = b.y - a.y;
        let b1 = a.x - b.x;
        let c1 = a1 * a.x + b1 * a.y;

        let c = other.start;
        let d = other.end;
        let a2 = d.y - c.y;
        let b2 = c.x - d.x;
        let c2 = a2 * c.x + b2 * c.y;

        let determinant = a1 * b2 - a2 * b1;

        if determinant == 0 {
            return None;
        }

        let x = (b2 as f64 * c1 as f64 - b1 as f64 * c2 as f64) / determinant as f64;
        let y = (a1 as f64 * c2 as f64 - a2 as f64 * c1 as f64) / determinant as f64;

        Some(Point::new(x as isize, y as isize))
    }
}
