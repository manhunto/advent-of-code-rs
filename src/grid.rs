use std::collections::HashMap;
use crate::point::Point;
use crate::range::Range;

#[allow(dead_code)]
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

    pub fn filter(&self, element: T) -> HashMap<&Point, &T> {
        self.cells
            .iter()
            .filter(|(_, e)| **e == element)
            .collect()
    }

    pub fn is_in(&self, point: &Point) -> bool {
        self.x_range.is_in_range(point.x as i64)
            && self.y_range.is_in_range(point.y as i64)
    }
}

// impl<T> Iterator for Grid<T> {
//     type Item = T;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.cells.iter().next()
//     }
// }


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