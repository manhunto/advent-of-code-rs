use crate::utils::point::Point;

pub struct DeltoidSurface {
    point: Point,
    distance: usize,
}

impl DeltoidSurface {
    pub fn new(point: Point, distance: usize) -> Self {
        Self { point, distance }
    }

    pub fn points(&self) -> Vec<Point> {
        let isize_distance = self.distance as isize;
        let required_capacity = (2 * self.distance.pow(2)) + 2 * self.distance + 1;
        let mut points: Vec<Point> = Vec::with_capacity(required_capacity);

        for x in -isize_distance..=isize_distance {
            let height_diff = isize_distance - x.abs();
            let x = self.point.x + x;

            for y in self.point.y - height_diff..=self.point.y + height_diff {
                points.push(Point::new(x, y));
            }
        }

        points
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
            .points()
            .iter()
            .all(|p| p.manhattan_distance(&middle) <= distance as isize));
    }

    #[test]
    fn points_count() {
        let middle = Point::new(4, 3);

        assert_eq!(5, DeltoidSurface::new(middle, 1).points().len());
        assert_eq!(13, DeltoidSurface::new(middle, 2).points().len());
        assert_eq!(25, DeltoidSurface::new(middle, 3).points().len());
        assert_eq!(41, DeltoidSurface::new(middle, 4).points().len());
    }
}
