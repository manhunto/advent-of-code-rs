use crate::utils::direction::Direction;
use crate::utils::point::Point;
use std::collections::HashSet;

/// Represents a contiguous region of points with a filled body.
/// This means that there are no holes in the region and all points are connected.
#[derive(Debug, PartialEq)]
pub struct FilledRegion {
    points: HashSet<Point>,
}

impl TryFrom<HashSet<Point>> for FilledRegion {
    type Error = String;

    fn try_from(value: HashSet<Point>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err("FilledRegion must have at least one point".to_string());
        }

        if value.len() > 1 {
            let has_only_valid = value
                .iter()
                .all(|&p| p.adjacent().iter().filter(|&pp| value.contains(pp)).count() > 0);

            if !has_only_valid {
                return Err("Invalid filled region. Is built with not adjacent points.".to_string());
            }
        }

        Ok(Self { points: value })
    }
}

impl FilledRegion {
    pub fn perimeter(&self) -> usize {
        self.points.iter().fold(0, |perimeter, point| {
            // point inside filled region has 0 surroundings, so it isn't on the perimeter
            let how_many_surrounding = point
                .adjacent()
                .iter()
                .filter(|p| self.points.contains(p))
                .count();

            perimeter + 4 - how_many_surrounding
        })
    }

    pub fn area(&self) -> usize {
        self.points.len()
    }

    pub fn points(&self) -> HashSet<Point> {
        self.points.clone()
    }

    pub fn corners(&self) -> usize {
        let edges: HashSet<(Point, Direction)> = self
            .points
            .iter()
            .flat_map(|&p| {
                let mut edges = vec![];
                if !self.points.contains(&p.north()) {
                    edges.push((p, Direction::North))
                }

                if !self.points.contains(&p.south()) {
                    edges.push((p, Direction::South))
                }

                if !self.points.contains(&p.west()) {
                    edges.push((p, Direction::West))
                }

                if !self.points.contains(&p.east()) {
                    edges.push((p, Direction::East))
                }

                edges
            })
            .collect();

        edges
            .iter()
            .filter(|(p, dir)| match dir {
                Direction::North if !edges.contains(&(p.east(), Direction::North)) => true,
                Direction::South if !edges.contains(&(p.east(), Direction::South)) => true,
                Direction::West if !edges.contains(&(p.south(), Direction::West)) => true,
                Direction::East if !edges.contains(&(p.south(), Direction::East)) => true,
                _ => false,
            })
            .count()
    }
}

#[cfg(test)]
mod test {
    use crate::utils::filled_region::FilledRegion;
    use crate::utils::grid::Grid;
    use crate::utils::point::Point;
    use std::collections::HashSet;

    #[test]
    fn try_from_empty_hashset() {
        assert_eq!(
            Err("FilledRegion must have at least one point".to_string()),
            FilledRegion::try_from(HashSet::new())
        );
    }

    #[test]
    fn try_from_invalid_hashset() {
        let set = HashSet::from_iter(vec![Point::new(1, 1), Point::new(1, 3)]);

        assert_eq!(
            Err("Invalid filled region. Is built with not adjacent points.".to_string()),
            FilledRegion::try_from(set)
        );

        let set = HashSet::from_iter(vec![Point::new(1, 1), Point::new(2, 2)]);

        assert_eq!(
            Err("Invalid filled region. Is built with not adjacent points.".to_string()),
            FilledRegion::try_from(set)
        );
    }

    #[test]
    fn try_from_is_valid() {
        let set = HashSet::from_iter(vec![
            Point::new(1, 1),
            Point::new(1, 2),
            Point::new(1, 3),
            Point::new(2, 3),
        ]);

        assert!(FilledRegion::try_from(set).is_ok());
    }

    #[test]
    fn try_from_hashmap_with_one_element() {
        let set = HashSet::from_iter(vec![Point::new(1, 1)]);

        assert!(FilledRegion::try_from(set).is_ok());
    }

    #[test]
    fn corners() {
        const EXAMPLE: &str = r#"AAAA
BBCD
BBCC
EEEC"#;

        let grid = Grid::<char>::from(EXAMPLE);

        assert_eq!(4, filled_region_from_grid(&grid, 'A').corners());
        assert_eq!(4, filled_region_from_grid(&grid, 'B').corners());
        assert_eq!(4, filled_region_from_grid(&grid, 'D').corners());
        assert_eq!(4, filled_region_from_grid(&grid, 'E').corners());
        assert_eq!(8, filled_region_from_grid(&grid, 'C').corners());
    }

    #[test]
    fn corners_e_shape() {
        const EXAMPLE: &str = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#;

        let grid = Grid::<char>::from(EXAMPLE);

        assert_eq!(12, filled_region_from_grid(&grid, 'E').corners());
    }

    #[test]
    fn corners_with_inside() {
        const EXAMPLE: &str = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#;

        let grid = Grid::<char>::from(EXAMPLE);

        assert_eq!(12, filled_region_from_grid(&grid, 'A').corners());
    }

    #[test]
    fn perimeter_of_single_point() {
        let filled_region = FilledRegion::try_from(HashSet::from([Point::new(0, 0)])).unwrap();

        assert_eq!(4, filled_region.perimeter());
    }

    #[test]
    fn perimeter_of_2x1_line() {
        let filled_region =
            FilledRegion::try_from(HashSet::from([Point::new(0, 0), Point::new(0, 1)])).unwrap();

        assert_eq!(6, filled_region.perimeter());
    }

    #[test]
    fn perimeter_of_2x2_square() {
        let filled_region = FilledRegion::try_from(HashSet::from([
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(1, 0),
            Point::new(1, 1),
        ]))
        .unwrap();

        assert_eq!(8, filled_region.perimeter());
    }

    #[test]
    fn perimeter_of_l_shape() {
        let filled_region = FilledRegion::try_from(HashSet::from([
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(2, 1),
        ]))
        .unwrap();

        assert_eq!(10, filled_region.perimeter());
    }

    #[test]
    fn perimeter_of_plus_shape() {
        let filled_region = FilledRegion::try_from(HashSet::from([
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(1, 1),
            Point::new(2, 1),
            Point::new(1, 2),
        ]))
        .unwrap();

        assert_eq!(12, filled_region.perimeter());
    }

    // todo: move get filled region to grid??
    // todo: handle multiple filled regions with the same element
    fn filled_region_from_grid(grid: &Grid<char>, element: char) -> FilledRegion {
        FilledRegion::try_from(HashSet::from_iter(grid.get_all_positions(&element))).unwrap()
    }
}
