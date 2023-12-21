use std::collections::{HashMap};
use std::fmt;
use std::fmt::Display;
use crate::point::Point;
use crate::range::Range;

#[derive(Debug)]
pub struct Grid<T> {
    cells: HashMap<Point, T>,
    x_range: Range,
    y_range: Range,
}

impl<T> Grid<T>
    where T: PartialEq
{
    pub fn new(cells: HashMap<Point, T>) -> Self {
        let x: Vec<i32> = cells
            .keys()
            .map(|k| k.x)
            .collect();

        let y: Vec<i32> = cells
            .keys()
            .map(|k| k.y)
            .collect();

        Self {
            cells,
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

    pub fn rows(&self) -> HashMap<i32, HashMap<&Point, &T>> {
        self.y_range
            .iter()
            .map(|y| {
                let cells_in_row = self.cells
                    .iter()
                    .filter(|(&point, _)| point.y == y as i32)
                    .collect();

                (y as i32, cells_in_row)
            })
            .collect()
    }

    pub fn columns(&self) -> HashMap<i32, HashMap<&Point, &T>> {
        self.x_range
            .iter()
            .map(|x| {
                let cells_in_column = self.cells
                    .iter()
                    .filter(|(&point, _)| point.x == x as i32)
                    .collect();

                (x as i32, cells_in_column)
            })
            .collect()
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

impl<T> From<&str> for Grid<T>
    where T: From<char> + PartialEq
{
    fn from(value: &str) -> Self {
        let cells: HashMap<Point, T> = value
            .lines()
            .enumerate()
            .map(|(y, line)| -> Vec<(Point, T)> {
                line
                    .chars()
                    .enumerate()
                    .map(|(x, c)| (Point::new(x as i32, y as i32), T::from(c)))
                    .collect()
            })
            .flatten()
            .collect();

        Grid::new(cells)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::grid::Grid;
    use crate::point::Point;

    #[test]
    fn get() {
        let grid: Grid<char> = grid();

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
        let grid: Grid<char> = grid();

        assert_eq!("AB\nCD\n", grid.to_string());
    }

    #[test]
    fn rows() {
        let grid: Grid<char> = grid();

        let func = |rows: &HashMap<i32, HashMap<&Point, &char>>, y| -> Vec<char> {
            rows
                .get(&y)
                .unwrap()
                .iter()
                .map(|(_, &&c)| c)
                .collect()
        };

        let rows = grid.rows();

        let row_1: Vec<char> = func(&rows, 0);
        assert_eq!(2, row_1.len());
        assert!(row_1.contains(&'A'));
        assert!(row_1.contains(&'B'));

        let row_2: Vec<char> = func(&rows, 1);
        assert_eq!(2, row_2.len());
        assert!(row_2.contains(&'C'));
        assert!(row_2.contains(&'D'));
    }

    #[test]
    fn columns() {
        let grid: Grid<char> = grid();

        let func = |rows: &HashMap<i32, HashMap<&Point, &char>>, x| -> Vec<char> {
            rows
                .get(&x)
                .unwrap()
                .iter()
                .map(|(_, &&c)| c)
                .collect()
        };

        let columns = grid.columns();

        let column_1: Vec<char> = func(&columns, 0);
        assert_eq!(2, column_1.len());
        assert!(column_1.contains(&'A'));
        assert!(column_1.contains(&'C'));

        let column_2: Vec<char> = func(&columns, 1);
        assert_eq!(2, column_2.len());
        assert!(column_2.contains(&'B'));
        assert!(column_2.contains(&'D'));
    }

    fn grid() -> Grid<char> {
        let mut hash_map: HashMap<Point, char> = HashMap::new();
        hash_map.insert(Point::new(0, 0), 'A');
        hash_map.insert(Point::new(0, 1), 'C');
        hash_map.insert(Point::new(1, 1), 'D');
        hash_map.insert(Point::new(1, 0), 'B');

        Grid::new(hash_map)
    }
}