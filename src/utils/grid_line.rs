use crate::utils::point::Point;
use crate::utils::range::Range;

/// Represents a line on a grid that can only be horizontal or vertical
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GridLine {
    Horizontal {
        y: isize,
        x_start: isize,
        x_end: isize,
    },
    Vertical {
        x: isize,
        y_start: isize,
        y_end: isize,
    },
}

impl GridLine {
    pub fn new(start: Point, end: Point) -> Option<Self> {
        if start.x == end.x {
            Some(Self::Vertical {
                x: start.x,
                y_start: start.y,
                y_end: end.y,
            })
        } else if start.y == end.y {
            Some(Self::Horizontal {
                y: start.y,
                x_start: start.x,
                x_end: end.x,
            })
        } else {
            None
        }
    }

    pub fn start(&self) -> Point {
        match self {
            Self::Horizontal { y, x_start, .. } => Point::new(*x_start, *y),
            Self::Vertical { x, y_start, .. } => Point::new(*x, *y_start),
        }
    }

    pub fn end(&self) -> Point {
        match self {
            Self::Horizontal { y, x_end, .. } => Point::new(*x_end, *y),
            Self::Vertical { x, y_end, .. } => Point::new(*x, *y_end),
        }
    }

    pub fn is_vertical(&self) -> bool {
        matches!(self, Self::Vertical { .. })
    }

    #[allow(dead_code)]
    pub fn is_horizontal(&self) -> bool {
        matches!(self, Self::Horizontal { .. })
    }

    /// Returns all points on this grid line from start to end
    pub fn points(&self) -> Vec<Point> {
        match self {
            Self::Horizontal { y, x_start, x_end } => Range::from_unordered(*x_start, *x_end)
                .iter()
                .map(|x| Point::new(x, *y))
                .collect(),
            Self::Vertical { x, y_start, y_end } => Range::from_unordered(*y_start, *y_end)
                .iter()
                .map(|y| Point::new(*x, y))
                .collect(),
        }
    }

    /// Checks if a point is on this grid line
    pub fn is_on(&self, point: &Point) -> bool {
        match self {
            Self::Horizontal { y, x_start, x_end } => {
                point.y == *y
                    && point.x >= (*x_start).min(*x_end)
                    && point.x <= (*x_start).max(*x_end)
            }
            Self::Vertical { x, y_start, y_end } => {
                point.x == *x
                    && point.y >= (*y_start).min(*y_end)
                    && point.y <= (*y_start).max(*y_end)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::grid_line::GridLine;
    use crate::utils::point::Point;

    #[test]
    fn new_horizontal() {
        let line = GridLine::new(Point::new(1, 3), Point::new(4, 3));
        assert!(line.is_some());
        assert!(line.unwrap().is_horizontal());
    }

    #[test]
    fn new_vertical() {
        let line = GridLine::new(Point::new(2, 1), Point::new(2, 4));
        assert!(line.is_some());
        assert!(line.unwrap().is_vertical());
    }

    #[test]
    fn new_diagonal_returns_none() {
        let line = GridLine::new(Point::new(0, 0), Point::new(3, 3));
        assert!(line.is_none());
    }

    #[test]
    fn points_horizontal() {
        let line = GridLine::new(Point::new(1, 3), Point::new(4, 3)).unwrap();
        let expected = vec![
            Point::new(1, 3),
            Point::new(2, 3),
            Point::new(3, 3),
            Point::new(4, 3),
        ];
        assert_eq!(line.points(), expected);
    }

    #[test]
    fn points_horizontal_reversed() {
        let line = GridLine::new(Point::new(4, 3), Point::new(1, 3)).unwrap();
        let expected = vec![
            Point::new(1, 3),
            Point::new(2, 3),
            Point::new(3, 3),
            Point::new(4, 3),
        ];
        assert_eq!(line.points(), expected);
    }

    #[test]
    fn points_vertical() {
        let line = GridLine::new(Point::new(2, 1), Point::new(2, 4)).unwrap();
        let expected = vec![
            Point::new(2, 1),
            Point::new(2, 2),
            Point::new(2, 3),
            Point::new(2, 4),
        ];
        assert_eq!(line.points(), expected);
    }

    #[test]
    fn points_vertical_reversed() {
        let line = GridLine::new(Point::new(2, 4), Point::new(2, 1)).unwrap();
        let expected = vec![
            Point::new(2, 1),
            Point::new(2, 2),
            Point::new(2, 3),
            Point::new(2, 4),
        ];
        assert_eq!(line.points(), expected);
    }

    #[test]
    fn is_on_horizontal() {
        let line = GridLine::new(Point::new(1, 3), Point::new(4, 3)).unwrap();
        assert!(line.is_on(&Point::new(1, 3)));
        assert!(line.is_on(&Point::new(2, 3)));
        assert!(line.is_on(&Point::new(4, 3)));
        assert!(!line.is_on(&Point::new(0, 3)));
        assert!(!line.is_on(&Point::new(5, 3)));
        assert!(!line.is_on(&Point::new(2, 4)));
    }

    #[test]
    fn is_on_vertical() {
        let line = GridLine::new(Point::new(2, 1), Point::new(2, 4)).unwrap();
        assert!(line.is_on(&Point::new(2, 1)));
        assert!(line.is_on(&Point::new(2, 2)));
        assert!(line.is_on(&Point::new(2, 4)));
        assert!(!line.is_on(&Point::new(2, 0)));
        assert!(!line.is_on(&Point::new(2, 5)));
        assert!(!line.is_on(&Point::new(3, 2)));
    }
}
