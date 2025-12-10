use crate::utils::line::Line;
use crate::utils::point::Point;
use crate::utils::traits::IsInside;

#[derive(PartialEq, Clone)]
pub struct Polygon {
    lines: Vec<Line>,
}

/// It supports only Horizontal and Vertical polygon lines
impl Polygon {
    fn new(lines: Vec<Line>) -> Self {
        // todo validation
        // not empty lines - it should be validate in line
        // only vertical or horizontal
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
            Point::new(first_corner.x, second_corner.y),
            Point::new(second_corner.x, second_corner.y),
            Point::new(second_corner.x, first_corner.y),
        ];

        Self::from_iter(points)
    }
}

impl FromIterator<Point> for Polygon {
    fn from_iter<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        let mut lines: Vec<Line> = Vec::new();

        let mut points = iter.into_iter();
        let first = points.next().unwrap();
        let mut current = first;

        for next in points {
            lines.push(Line::new(current, next));
            current = next;
        }

        lines.push(Line::new(current, first));

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

#[cfg(test)]
mod tests {
    use crate::utils::line::Line;
    use crate::utils::point::Point;
    use crate::utils::polygon::Polygon;

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
            &Line::new(Point::new(7, 1), Point::new(11, 1)),
            lines.next().unwrap()
        );
        assert_eq!(
            &Line::new(Point::new(11, 1), Point::new(11, 7)),
            lines.next().unwrap()
        );
        assert_eq!(
            &Line::new(Point::new(11, 7), Point::new(9, 7)),
            lines.next().unwrap()
        );
        assert_eq!(
            &Line::new(Point::new(9, 7), Point::new(9, 5)),
            lines.next().unwrap()
        );
        assert_eq!(
            &Line::new(Point::new(9, 5), Point::new(2, 5)),
            lines.next().unwrap()
        );
        assert_eq!(
            &Line::new(Point::new(2, 5), Point::new(2, 3)),
            lines.next().unwrap()
        );
        assert_eq!(
            &Line::new(Point::new(2, 3), Point::new(7, 3)),
            lines.next().unwrap()
        );
        assert_eq!(
            &Line::new(Point::new(7, 3), Point::new(7, 1)),
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
}
