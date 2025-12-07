use crate::utils::point::Point;

#[derive(Debug, Copy, Clone)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn start(&self) -> Point {
        self.start
    }

    pub fn end(&self) -> Point {
        self.end
    }

    pub fn new(start: Point, end: Point) -> Self {
        // check if it is vertical, horizontal or diagonal only

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

    pub fn is_on(&self, point: &Point) -> bool {
        let a = self.start;
        let b = self.end;
        let p = *point;

        let cross_product = (p.y - a.y) * (b.x - a.x) - (p.x - a.x) * (b.y - a.y);

        if cross_product != 0 {
            return false;
        }

        p.x >= a.x.min(b.x) && p.x <= a.x.max(b.x) && p.y >= a.y.min(b.y) && p.y <= a.y.max(b.y)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::line::Line;
    use crate::utils::point::Point;

    #[test]
    fn is_on() {
        let line = Line::new(Point::new(0, 0), Point::new(10, 10));

        assert!(line.is_on(&Point::new(0, 0))); // Start point
        assert!(line.is_on(&Point::new(5, 5))); // On the line
        assert!(line.is_on(&Point::new(10, 10))); // End point
        assert!(!line.is_on(&Point::new(15, 15))); // Outside the segment
        assert!(!line.is_on(&Point::new(1, 5))); // Not on the line (not collinear)
    }
}
