use crate::utils::grid_line::GridLine;
use crate::utils::point::Point;
use crate::utils::traits::{Intersect, IsInside};
use itertools::Itertools;

#[derive(PartialEq, Clone)]
pub struct Polygon {
    lines: Vec<GridLine>,
}

impl Polygon {
    fn new(lines: Vec<GridLine>) -> Self {
        Self { lines }
    }

    pub fn points(&self) -> Vec<Point> {
        self.lines
            .iter()
            .flat_map(|line| {
                let len = line.points().len();

                line.points().drain(..(len - 1)).collect::<Vec<_>>()
            })
            .collect()
    }

    pub fn rectangle(first_corner: Point, second_corner: Point) -> Self {
        let points = [
            Point::new(first_corner.x, first_corner.y),
            Point::new(second_corner.x, first_corner.y),
            Point::new(second_corner.x, second_corner.y),
            Point::new(first_corner.x, second_corner.y),
        ];

        Self::from_iter(points)
    }

    #[allow(dead_code)]
    pub fn extend(&self) -> Self {
        let lines = self
            .lines
            .iter()
            .map(|line| {
                let direction = line.direction().ccw();

                line.extend().moved(direction)
            })
            .collect_vec();

        Self { lines }
    }
}

impl FromIterator<Point> for Polygon {
    fn from_iter<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        let mut lines: Vec<GridLine> = Vec::new();

        let mut points = iter.into_iter();
        let first = points.next().unwrap();
        let mut current = first;

        for next in points {
            lines.push(
                GridLine::new(current, next)
                    .expect("Polygon only supports horizontal and vertical lines"),
            );
            current = next;
        }

        lines.push(
            GridLine::new(current, first)
                .expect("Polygon only supports horizontal and vertical lines"),
        );

        Self::new(lines)
    }
}

impl IsInside<Point> for Polygon {
    fn is_inside(&self, value: &Point) -> bool {
        if self.lines.iter().any(|line| line.is_on(value)) {
            return true;
        }

        let mut intersections = 0;
        for line in &self.lines {
            if !line.is_vertical() {
                continue;
            }

            let p1 = line.start();
            let p2 = line.end();

            if p1.x <= value.x {
                continue;
            }

            let (y_min, y_max) = if p1.y < p2.y {
                (p1.y, p2.y)
            } else {
                (p2.y, p1.y)
            };

            if value.y >= y_min && value.y < y_max {
                intersections += 1;
            }
        }

        intersections % 2 == 1
    }
}

impl IsInside<Polygon> for Polygon {
    fn is_inside(&self, value: &Polygon) -> bool {
        value.points().iter().all(|point| self.is_inside(point))
    }
}

impl Intersect<Polygon> for Polygon {
    fn intersect(&self, value: &Polygon) -> bool {
        value
            .lines
            .iter()
            .any(|value_line| self.intersect(value_line))
    }
}

impl Intersect<GridLine> for Polygon {
    fn intersect(&self, value: &GridLine) -> bool {
        self.lines.iter().any(|line| line.intersect(value))
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::grid_line::GridLine;
    use crate::utils::point::Point;
    use crate::utils::polygon::Polygon;
    use crate::utils::traits::Intersect;

    #[test]
    fn from_iterator() {
        let points = [
            Point::new(7, 1),
            Point::new(11, 1),
            Point::new(11, 7),
            Point::new(9, 7),
            Point::new(9, 5),
            Point::new(2, 5),
            Point::new(2, 3),
            Point::new(7, 3),
        ];

        let polygon: Polygon = points.into_iter().collect();
        let mut lines = polygon.lines.iter();

        assert_eq!(
            &GridLine::new(Point::new(7, 1), Point::new(11, 1)).unwrap(),
            lines.next().unwrap()
        );
        assert_eq!(
            &GridLine::new(Point::new(11, 1), Point::new(11, 7)).unwrap(),
            lines.next().unwrap()
        );
        assert_eq!(
            &GridLine::new(Point::new(11, 7), Point::new(9, 7)).unwrap(),
            lines.next().unwrap()
        );
        assert_eq!(
            &GridLine::new(Point::new(9, 7), Point::new(9, 5)).unwrap(),
            lines.next().unwrap()
        );
        assert_eq!(
            &GridLine::new(Point::new(9, 5), Point::new(2, 5)).unwrap(),
            lines.next().unwrap()
        );
        assert_eq!(
            &GridLine::new(Point::new(2, 5), Point::new(2, 3)).unwrap(),
            lines.next().unwrap()
        );
        assert_eq!(
            &GridLine::new(Point::new(2, 3), Point::new(7, 3)).unwrap(),
            lines.next().unwrap()
        );
        assert_eq!(
            &GridLine::new(Point::new(7, 3), Point::new(7, 1)).unwrap(),
            lines.next().unwrap()
        );
        assert!(lines.next().is_none());
    }

    #[test]
    fn points_are_unique_rectangle() {
        let points = [
            Point::new(0, 0),
            Point::new(0, 3),
            Point::new(2, 3),
            Point::new(2, 0),
        ];

        let polygon: Polygon = points.into_iter().collect();

        assert_eq!(10, polygon.points().len());
    }

    #[test]
    fn points_are_unique_complex() {
        let points = [
            Point::new(0, 0),
            Point::new(3, 0),
            Point::new(3, 2),
            Point::new(5, 2),
            Point::new(5, 5),
            Point::new(2, 5),
            Point::new(2, 7),
            Point::new(0, 7),
        ];

        let polygon: Polygon = points.into_iter().collect();

        assert_eq!(24, polygon.points().len());
    }

    #[test]
    fn extend_rectangle() {
        let rectangle = Polygon::rectangle(Point::new(2, 2), Point::new(5, 5));
        let extended = rectangle.extend();

        let mut lines = extended.lines.iter();

        assert_eq!(
            &GridLine::new(Point::new(1, 1), Point::new(6, 1)).unwrap(),
            lines.next().unwrap()
        );
        assert_eq!(
            &GridLine::new(Point::new(6, 1), Point::new(6, 6)).unwrap(),
            lines.next().unwrap()
        );
        assert_eq!(
            &GridLine::new(Point::new(6, 6), Point::new(1, 6)).unwrap(),
            lines.next().unwrap()
        );
        assert_eq!(
            &GridLine::new(Point::new(1, 6), Point::new(1, 1)).unwrap(),
            lines.next().unwrap()
        );
    }

    #[test]
    // fixme: there is a bug, check it and fix
    fn extend_complex_l_shape() {
        // Create an L-shaped polygon
        let points = [
            Point::new(2, 2),
            Point::new(5, 2),
            Point::new(5, 4),
            Point::new(4, 4),
            Point::new(4, 6),
            Point::new(2, 6),
        ];

        let polygon: Polygon = points.into_iter().collect();
        let extended = polygon.extend();

        let mut lines = extended.lines.iter();

        // Bottom edge: (2,2)->(5,2) extends and moves north
        assert_eq!(
            &GridLine::new(Point::new(1, 1), Point::new(6, 1)).unwrap(),
            lines.next().unwrap()
        );
        // Right edge (lower): (5,2)->(5,4) extends and moves east
        assert_eq!(
            &GridLine::new(Point::new(6, 1), Point::new(6, 5)).unwrap(),
            lines.next().unwrap()
        );
        // Inner horizontal: (5,4)->(4,4) extends and moves south
        assert_eq!(
            &GridLine::new(Point::new(6, 5), Point::new(3, 5)).unwrap(),
            lines.next().unwrap()
        );
        // Right edge (upper): (4,4)->(4,6) extends and moves east
        assert_eq!(
            &GridLine::new(Point::new(5, 3), Point::new(5, 7)).unwrap(),
            lines.next().unwrap()
        );
        // Top edge: (4,6)->(2,6) extends and moves south
        assert_eq!(
            &GridLine::new(Point::new(5, 7), Point::new(1, 7)).unwrap(),
            lines.next().unwrap()
        );
        // Left edge: (2,6)->(2,2) extends and moves west
        assert_eq!(
            &GridLine::new(Point::new(1, 7), Point::new(1, 1)).unwrap(),
            lines.next().unwrap()
        );
    }

    #[test]
    fn intersect_polygon_with_grid_line() {
        let polygon = Polygon::rectangle(Point::new(2, 2), Point::new(5, 5));

        // Horizontal line crosses through polygon
        let horizontal = GridLine::new(Point::new(0, 3), Point::new(7, 3)).unwrap();
        assert!(polygon.intersect(&horizontal));

        // Vertical line crosses through polygon
        let vertical = GridLine::new(Point::new(3, 0), Point::new(3, 7)).unwrap();
        assert!(polygon.intersect(&vertical));

        // Horizontal line on top edge
        let on_edge = GridLine::new(Point::new(1, 2), Point::new(6, 2)).unwrap();
        assert!(polygon.intersect(&on_edge));

        // Horizontal line above polygon (no intersection)
        let above = GridLine::new(Point::new(0, 1), Point::new(7, 1)).unwrap();
        assert!(!polygon.intersect(&above));

        // Horizontal line below polygon (no intersection)
        let below = GridLine::new(Point::new(0, 6), Point::new(7, 6)).unwrap();
        assert!(!polygon.intersect(&below));

        // Vertical line to the left (no intersection)
        let left = GridLine::new(Point::new(1, 0), Point::new(1, 7)).unwrap();
        assert!(!polygon.intersect(&left));

        // Vertical line to the right (no intersection)
        let right = GridLine::new(Point::new(6, 0), Point::new(6, 7)).unwrap();
        assert!(!polygon.intersect(&right));

        // Line touches corner
        let corner = GridLine::new(Point::new(2, 0), Point::new(2, 3)).unwrap();
        assert!(polygon.intersect(&corner));
    }

    #[test]
    fn intersect_two_polygons_overlapping() {
        let polygon1 = Polygon::rectangle(Point::new(2, 2), Point::new(5, 5));
        let polygon2 = Polygon::rectangle(Point::new(4, 4), Point::new(7, 7));

        // Overlapping rectangles should intersect
        assert!(polygon1.intersect(&polygon2));
        assert!(polygon2.intersect(&polygon1));
    }

    #[test]
    fn intersect_two_polygons_separated() {
        let polygon1 = Polygon::rectangle(Point::new(2, 2), Point::new(5, 5));
        let polygon2 = Polygon::rectangle(Point::new(7, 7), Point::new(10, 10));

        // Separated rectangles should not intersect
        assert!(!polygon1.intersect(&polygon2));
        assert!(!polygon2.intersect(&polygon1));
    }

    #[test]
    fn intersect_two_polygons_touching_edge() {
        use crate::utils::traits::Intersect;

        let polygon1 = Polygon::rectangle(Point::new(2, 2), Point::new(5, 5));
        let polygon2 = Polygon::rectangle(Point::new(5, 2), Point::new(8, 5));

        // Rectangles sharing an edge should intersect
        assert!(polygon1.intersect(&polygon2));
        assert!(polygon2.intersect(&polygon1));
    }

    #[test]
    fn intersect_two_polygons_touching_corner() {
        let polygon1 = Polygon::rectangle(Point::new(2, 2), Point::new(5, 5));
        let polygon2 = Polygon::rectangle(Point::new(5, 5), Point::new(8, 8));

        // Rectangles sharing only a corner should intersect
        assert!(polygon1.intersect(&polygon2));
        assert!(polygon2.intersect(&polygon1));
    }

    #[test]
    fn intersect_two_polygons_one_inside_other() {
        let outer = Polygon::rectangle(Point::new(1, 1), Point::new(10, 10));
        let inner = Polygon::rectangle(Point::new(4, 4), Point::new(6, 6));

        // When one polygon is completely inside another with no touching edges,
        // they don't intersect (no lines cross)
        assert!(!outer.intersect(&inner));
        assert!(!inner.intersect(&outer));
    }

    #[test]
    fn intersect_complex_polygons() {
        // L-shaped polygon
        let l_shape = Polygon::from_iter([
            Point::new(2, 2),
            Point::new(5, 2),
            Point::new(5, 4),
            Point::new(4, 4),
            Point::new(4, 6),
            Point::new(2, 6),
        ]);

        // Rectangle that intersects with the L
        let rectangle = Polygon::rectangle(Point::new(3, 3), Point::new(6, 5));
        assert!(l_shape.intersect(&rectangle));
        assert!(rectangle.intersect(&l_shape));

        // Rectangle that doesn't intersect
        let separate = Polygon::rectangle(Point::new(7, 7), Point::new(10, 10));
        assert!(!l_shape.intersect(&separate));
        assert!(!separate.intersect(&l_shape));
    }
}
