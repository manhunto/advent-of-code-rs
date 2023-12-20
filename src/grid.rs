use std::collections::HashMap;
use crate::point::Point;

#[allow(dead_code)]
pub struct Grid<T> {
    cells: HashMap<Point, T>,
}

impl<T> Grid<T>
    where T: PartialEq
{
    #[allow(dead_code)]
    pub fn new(points: HashMap<Point, T>) -> Self {
        Self { cells: points }
    }

    #[allow(dead_code)]
    pub fn get(&self, x: i32, y: i32) -> Option<&T> {
        self.cells.get(&Point::new(x, y))
    }

    pub fn get_for_point(&self, point: &Point) -> Option<&T> {
        self.cells.get(&point)
    }

    pub fn get_first_position(&self, element: &T) -> Option<Point> {
        self.cells
            .iter()
            .find_map(|(p, e)| {
                if element == e {
                    return Some(p.clone());
                }

                return None;
            })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::grid::Grid;
    use crate::point::Point;

    #[test]
    fn get() {
        let mut hash_map: HashMap<Point, char> = HashMap::new();
        hash_map.insert(Point::new(0, 0), 'A');
        hash_map.insert(Point::new(1, 0), 'B');
        hash_map.insert(Point::new(0, 1), 'C');
        hash_map.insert(Point::new(1, 1), 'D');

        let grid = Grid::new(hash_map);

        assert_eq!(Some(&'A'), grid.get(0, 0));
        assert_eq!(Some(&'B'), grid.get(1, 0));
        assert_eq!(Some(&'C'), grid.get(0, 1));
        assert_eq!(Some(&'D'), grid.get(1, 1));

        assert!(grid.get(1, 2).is_none())
    }

    #[test]
    fn get_first_position() {
        let mut hash_map: HashMap<Point, char> = HashMap::new();
        hash_map.insert(Point::new(2, 3), 'X');

        let grid = Grid::new(hash_map);

        assert_eq!(Point::new(2, 3), grid.get_first_position(&'X').unwrap());

        assert!(grid.get_first_position(&'A').is_none())
    }
}