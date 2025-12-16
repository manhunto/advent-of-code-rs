use crate::utils::direction::Direction;
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

    #[expect(dead_code)]
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

    /// It extends its size by 1 in the direction of orientation
    #[expect(dead_code)]
    pub fn extend(&self) -> Self {
        match self {
            GridLine::Horizontal {
                y: _,
                x_start,
                x_end,
            } => {
                if x_start < x_end {
                    GridLine::new(self.start().west(), self.end().east()).unwrap()
                } else {
                    GridLine::new(self.start().east(), self.end().west()).unwrap()
                }
            }
            GridLine::Vertical {
                x: _,
                y_start,
                y_end,
            } => {
                if y_start < y_end {
                    GridLine::new(self.start().north(), self.end().south()).unwrap()
                } else {
                    GridLine::new(self.start().south(), self.end().north()).unwrap()
                }
            }
        }
    }

    /// Move line in direction by 1
    #[expect(dead_code)]
    pub fn moved(&self, direction: Direction) -> Option<Self> {
        let start = self.start();
        let end = self.end();

        match direction {
            Direction::North => Self::new(start.north(), end.north()),
            Direction::East => Self::new(start.east(), end.east()),
            Direction::South => Self::new(start.south(), end.south()),
            Direction::West => Self::new(start.west(), end.west()),
            _ => unimplemented!("Direction {} unimplemented to move grid line", direction),
        }
    }

    #[expect(dead_code)]
    pub fn direction(&self) -> Direction {
        self.start().direction(&self.end())
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::direction::Direction;
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

    #[test]
    fn extend_vertical() {
        let line = GridLine::new(Point::new(1, 1), Point::new(4, 1)).unwrap();
        let extended = line.extend();

        assert_eq!(Point::new(0, 1), extended.start());
        assert_eq!(Point::new(5, 1), extended.end());
    }

    #[test]
    fn extend_vertical_reversed() {
        let line = GridLine::new(Point::new(5, 1), Point::new(0, 1)).unwrap();
        let extended = line.extend();

        assert_eq!(Point::new(6, 1), extended.start());
        assert_eq!(Point::new(-1, 1), extended.end());
    }

    #[test]
    fn extend_horizontal() {
        let line = GridLine::new(Point::new(5, 4), Point::new(5, 11)).unwrap();
        let extended = line.extend();

        assert_eq!(Point::new(5, 3), extended.start());
        assert_eq!(Point::new(5, 12), extended.end());
    }

    #[test]
    fn extend_horizontal_reversed() {
        let line = GridLine::new(Point::new(3, 8), Point::new(3, 2)).unwrap();
        let extended = line.extend();

        assert_eq!(Point::new(3, 9), extended.start());
        assert_eq!(Point::new(3, 1), extended.end());
    }

    #[test]
    fn moved_horizontal() {
        let line = GridLine::new(Point::new(3, 8), Point::new(3, 2)).unwrap();
        let east = line.moved(Direction::East);

        assert!(east.is_some());
        assert_eq!(
            GridLine::new(Point::new(4, 8), Point::new(4, 2)).unwrap(),
            east.unwrap()
        );

        let west = line.moved(Direction::West);

        assert!(west.is_some());
        assert_eq!(
            GridLine::new(Point::new(2, 8), Point::new(2, 2)).unwrap(),
            west.unwrap()
        );
    }

    #[test]
    fn moved_vertical() {
        let line = GridLine::new(Point::new(2, 7), Point::new(7, 7)).unwrap();
        let north = line.moved(Direction::North);

        assert!(north.is_some());
        assert_eq!(
            GridLine::new(Point::new(2, 6), Point::new(7, 6)).unwrap(),
            north.unwrap()
        );

        let south = line.moved(Direction::South);

        assert!(south.is_some());
        assert_eq!(
            GridLine::new(Point::new(2, 8), Point::new(7, 8)).unwrap(),
            south.unwrap()
        );
    }

    #[test]
    fn direction_horizontal_east() {
        let line = GridLine::new(Point::new(1, 3), Point::new(4, 3)).unwrap();
        assert_eq!(Direction::East, line.direction());
    }

    #[test]
    fn direction_horizontal_west() {
        let line = GridLine::new(Point::new(4, 3), Point::new(1, 3)).unwrap();
        assert_eq!(Direction::West, line.direction());
    }

    #[test]
    fn direction_vertical_south() {
        let line = GridLine::new(Point::new(2, 1), Point::new(2, 4)).unwrap();
        assert_eq!(Direction::South, line.direction());
    }

    #[test]
    fn direction_vertical_north() {
        let line = GridLine::new(Point::new(2, 4), Point::new(2, 1)).unwrap();
        assert_eq!(Direction::North, line.direction());
    }
}
