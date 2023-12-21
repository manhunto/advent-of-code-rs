use std::collections::{HashMap};
use std::fmt;
use std::fmt::Display;
use crate::point::Point;
use crate::range::Range;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Grid<T> {
    cells: HashMap<Point, T>,
    x_range: Range,
    y_range: Range,
}

impl<T> Grid<T>
    where T: PartialEq
{
    pub fn new(points: HashMap<Point, T>) -> Self {
        let x: Vec<i32> = points
            .keys()
            .map(|k| k.x)
            .collect();

        let y: Vec<i32> = points
            .keys()
            .map(|k| k.y)
            .collect();

        Self {
            cells: points,
            x_range: Range::new(*x.iter().min().unwrap() as i64, *x.iter().max().unwrap() as i64).unwrap(),
            y_range: Range::new(*y.iter().min().unwrap() as i64, *y.iter().max().unwrap() as i64).unwrap(),
        }
    }

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

    pub fn is_in(&self, point: &Point) -> bool {
        self.x_range.is_in_range(point.x as i64)
            && self.y_range.is_in_range(point.y as i64)
    }
}

impl<T> Display for Grid<T>
    where T: Display + Ord
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printed_grid = String::new();
        for y in self.y_range.iter() {
            for x in self.x_range.iter() {
                let el = self.get(x as i32, y as i32).unwrap();

                printed_grid += el.to_string().as_str()
            }

            printed_grid += "\n";
        }

        write!(f, "{}", printed_grid)
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

    #[test]
    fn display() {
        let mut hash_map: HashMap<Point, char> = HashMap::new();
        hash_map.insert(Point::new(0, 0), 'A');
        hash_map.insert(Point::new(0, 1), 'C');
        hash_map.insert(Point::new(1, 1), 'D');
        hash_map.insert(Point::new(1, 0), 'B');

        let grid = Grid::new(hash_map);

        assert_eq!("AB\nCD\n", grid.to_string());
    }
}