use crate::utils::point::Point;
use crate::utils::range::Range;
use std::collections::HashMap;

type Ranges = HashMap<isize, Range>;

pub struct DeltoidSurface {
    #[allow(dead_code)]
    point: Point,
    #[allow(dead_code)]
    distance: usize,
    #[allow(dead_code)]
    ranges: Ranges,
    points: Vec<Point>,
}

impl DeltoidSurface {
    pub fn new(point: Point, distance: usize) -> Self {
        let ranges = Self::build_ranges(point, distance);
        let points = Self::build_points(&ranges);

        Self {
            point,
            distance,
            ranges,
            points,
        }
    }

    pub fn points(&self) -> Vec<Point> {
        self.points.clone()
    }

    fn build_ranges(point: Point, distance: usize) -> Ranges {
        let isize_distance = distance as isize;

        let mut ranges: Ranges = Ranges::with_capacity(distance * 2 + 1);

        for x in -isize_distance..=isize_distance {
            let height_diff = isize_distance - x.abs();
            let x = point.x + x;
            let y_range = Range::new(point.y - height_diff, point.y + height_diff).unwrap();

            ranges.insert(x, y_range);
        }

        ranges
    }

    fn build_points(ranges: &Ranges) -> Vec<Point> {
        ranges
            .iter()
            .flat_map(|(x, y_range)| {
                y_range
                    .iter()
                    .map(|y| Point::new(*x, y))
                    .collect::<Vec<Point>>()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::deltoid_surface::DeltoidSurface;
    use crate::utils::grid::Grid;
    use crate::utils::point::Point;
    use crate::utils::surface_range::SurfaceRange;

    #[test]
    fn deltoid_surface_visual_test() {
        const EXPECTED: &str = r#"........
....#...
...###..
..#####.
...###..
....#...
........
........"#;

        let expected_grid: Grid<char> = Grid::from(EXPECTED);
        let surface = DeltoidSurface::new(Point::new(4, 3), 2);

        let mut current_grid: Grid<char> = Grid::filled(SurfaceRange::square(7), '.');
        current_grid.modify_many(surface.points(), '#');

        assert_eq!(expected_grid.to_string(), current_grid.to_string());
    }

    #[test]
    fn manhattan_distance_is_at_most_distance() {
        let middle = Point::new(4, 3);
        let distance = 3;

        let surface = DeltoidSurface::new(middle, distance);

        assert!(surface
            .points
            .iter()
            .all(|p| p.manhattan_distance(&middle) <= distance as isize));
    }
}
