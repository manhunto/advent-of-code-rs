use std::collections::HashMap;
use crate::point::Point;

struct Grid<T> {
    points: HashMap<Point, T>,
}

impl<T> Grid<T> {
    fn new(points: HashMap<Point, T>) -> Self {
        Self { points }
    }

    fn get(&self, x: i32, y: i32) -> Option<&T> {
        self.points.get(&Point::new(x, y))
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
}