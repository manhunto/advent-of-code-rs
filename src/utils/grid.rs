use crate::utils::direction::Direction;
use crate::utils::point::Point;
use crate::utils::range::Range;
use crate::utils::surface_range::SurfaceRange;
use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    cells: HashMap<Point, T>,
    columns_range: Range,
    rows_range: Range,
}

impl<T> Grid<T>
where
    T: PartialEq,
{
    pub fn new(cells: HashMap<Point, T>) -> Self {
        let columns_range = Self::calculate_columns_range(&cells);
        let rows_range = Self::calculate_rows_range(&cells);

        Self {
            cells,
            columns_range,
            rows_range,
        }
    }

    pub fn from_custom(input: &str, func: fn(char) -> T) -> Self {
        let cells: HashMap<Point, T> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| -> Vec<(Point, T)> {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| (Point::new(x as isize, y as isize), func(c)))
                    .collect()
            })
            .collect();

        Self::new(cells)
    }

    #[cfg(test)]
    pub fn filled(surface_range: SurfaceRange, element: T) -> Self
    where
        T: Clone,
    {
        let mut cells: HashMap<Point, T> = HashMap::with_capacity(surface_range.area());

        for x in surface_range.columns().iter() {
            for y in surface_range.rows().iter() {
                cells.insert(Point::new(x, y), element.clone());
            }
        }

        Self::new(cells)
    }

    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        self.cells.get(&Point::new(x, y))
    }

    pub fn get_for_point(&self, point: &Point) -> Option<&T> {
        self.cells.get(point)
    }

    pub fn is_for_point(&self, point: &Point, element: T) -> bool {
        self.get_for_point(point).is_some_and(|e| e == &element)
    }

    pub fn get_first_position(&self, element: &T) -> Option<Point> {
        self.cells.iter().find_map(|(p, e)| {
            if element == e {
                return Some(*p);
            }

            None
        })
    }

    pub fn get_all_positions(&self, element: &T) -> Vec<Point> {
        self.cells
            .iter()
            .filter(|(_, e)| element == *e)
            .map(|(p, _)| *p)
            .collect()
    }

    pub fn is_in(&self, point: &Point) -> bool {
        self.columns_range.contains(point.x) && self.rows_range.contains(point.y)
    }

    pub fn rows(&self) -> BTreeMap<isize, BTreeMap<&Point, &T>> {
        self.rows_range
            .iter()
            .map(|y| {
                let cells_in_row = self
                    .cells
                    .iter()
                    .filter(|(&point, _)| point.y == y)
                    .collect();

                (y, cells_in_row)
            })
            .collect()
    }

    pub fn columns(&self) -> BTreeMap<isize, BTreeMap<&Point, &T>> {
        self.columns_range
            .iter()
            .map(|x| {
                let cells_in_column = self
                    .cells
                    .iter()
                    .filter(|(&point, _)| point.x == x)
                    .collect();

                (x, cells_in_column)
            })
            .collect()
    }

    pub fn insert_rows(&mut self, rows: Vec<isize>, element: T)
    where
        T: Clone,
    {
        for row in rows.iter().sorted().rev() {
            self.insert_row(*row, element.clone());
        }
    }

    pub fn insert_row(&mut self, row: isize, element: T)
    where
        T: Clone,
    {
        self.move_rows_to_south_from(row);
        self.add_row(row, element);
        self.recalculate_ranges();
    }

    pub fn insert_columns(&mut self, columns: Vec<isize>, element: T)
    where
        T: Clone,
    {
        for column in columns.iter().sorted().rev() {
            self.insert_column(*column, element.clone());
        }
    }

    pub fn insert_column(&mut self, column: isize, element: T)
    where
        T: Clone,
    {
        self.move_columns_to_east_from(column);
        self.add_column(column, element);
        self.recalculate_ranges();
    }

    pub fn rows_range(&self) -> Range {
        self.rows_range
    }

    pub fn columns_range(&self) -> Range {
        self.columns_range
    }

    pub fn surface_range(&self) -> SurfaceRange {
        SurfaceRange::new(self.columns_range(), self.rows_range())
    }

    pub fn modify(&mut self, point: Point, new_value: T) {
        *self.cells.get_mut(&point).unwrap() = new_value;
    }

    #[cfg(test)]
    pub fn modify_many(&mut self, points: Vec<Point>, new_value: T)
    where
        T: Clone,
    {
        for point in points {
            self.modify(point, new_value.clone())
        }
    }

    fn move_rows_to_south_from(&mut self, from: isize) {
        for y in self
            .rows_range
            .iter()
            .skip(from as usize)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            for x in self.columns_range.iter() {
                let old = Point::new(x, y);
                let new = old.move_in(Direction::South);

                self.replace_position(&old, new);
            }
        }
    }

    fn replace_position(&mut self, old: &Point, new: Point) {
        if let Some(v) = self.cells.remove(old) {
            self.cells.insert(new, v);
        }
    }

    fn move_columns_to_east_from(&mut self, from: isize) {
        for x in self
            .columns_range
            .iter()
            .skip(from as usize)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            for y in self.rows_range.iter() {
                let old = Point::new(x, y);
                let new = old.move_in(Direction::East);

                self.replace_position(&old, new);
            }
        }
    }

    fn add_row(&mut self, row: isize, element: T)
    where
        T: Clone,
    {
        for x in self.columns_range.iter() {
            let new = Point::new(x, row);

            self.cells.insert(new, element.clone());
        }
    }

    fn add_column(&mut self, column: isize, element: T)
    where
        T: Clone,
    {
        for y in self.rows_range.iter() {
            let new = Point::new(column, y);

            self.cells.insert(new, element.clone());
        }
    }

    fn calculate_rows_range(cells: &HashMap<Point, T>) -> Range {
        let y: Vec<isize> = cells.keys().map(|k| k.y).collect();

        Range::new(*y.iter().min().unwrap(), *y.iter().max().unwrap()).unwrap()
    }

    fn calculate_columns_range(cells: &HashMap<Point, T>) -> Range {
        let x: Vec<isize> = cells.keys().map(|k| k.x).collect();

        Range::new(*x.iter().min().unwrap(), *x.iter().max().unwrap()).unwrap()
    }

    fn recalculate_ranges(&mut self) {
        self.columns_range = Self::calculate_columns_range(&self.cells);
        self.rows_range = Self::calculate_rows_range(&self.cells)
    }

    pub fn elements_with_points(&self) -> HashMap<T, Vec<Point>>
    where
        T: Eq + Hash + Clone,
    {
        let mut elements: HashMap<T, Vec<Point>> = HashMap::new();
        let surface_range = self.surface_range();

        for x in surface_range.columns().iter() {
            for y in surface_range.rows().iter() {
                let element = self.get(x, y).unwrap();

                elements
                    .entry(element.clone())
                    .or_default()
                    .push(Point::new(x, y));
            }
        }

        elements
    }

    pub fn find<F>(&self, find_func: &F) -> Option<(&Point, &T)>
    where
        F: Fn(&Point, &T) -> bool,
    {
        self.cells
            .iter()
            .find(|(point, element)| find_func(point, element))
    }
}

impl<T> Display for Grid<T>
where
    T: Display + Ord,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printed_grid = String::new();
        for y in self.rows_range.iter() {
            for x in self.columns_range.iter() {
                let el = self.get(x, y).unwrap();

                printed_grid += el.to_string().as_str()
            }

            printed_grid += "\n";
        }

        write!(f, "{}", printed_grid)
    }
}

impl<T> From<&str> for Grid<T>
where
    T: From<char> + PartialEq,
{
    fn from(value: &str) -> Self {
        let cells: HashMap<Point, T> = value
            .lines()
            .enumerate()
            .flat_map(|(y, line)| -> Vec<(Point, T)> {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| (Point::new(x as isize, y as isize), T::from(c)))
                    .collect()
            })
            .collect();

        Grid::new(cells)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::grid::Grid;
    use crate::utils::point::Point;
    use std::collections::{BTreeMap, HashMap};

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
        let rows = grid.rows();

        let row_1: Vec<char> = get_chars(&rows, 0);
        assert_eq!(2, row_1.len());
        assert!(row_1.contains(&'A'));
        assert!(row_1.contains(&'B'));

        let row_2: Vec<char> = get_chars(&rows, 1);
        assert_eq!(2, row_2.len());
        assert!(row_2.contains(&'C'));
        assert!(row_2.contains(&'D'));
    }

    #[test]
    fn columns() {
        let grid: Grid<char> = grid();
        let columns = grid.columns();

        let column_1: Vec<char> = get_chars(&columns, 0);
        assert_eq!(2, column_1.len());
        assert!(column_1.contains(&'A'));
        assert!(column_1.contains(&'C'));

        let column_2: Vec<char> = get_chars(&columns, 1);
        assert_eq!(2, column_2.len());
        assert!(column_2.contains(&'B'));
        assert!(column_2.contains(&'D'));
    }

    #[test]
    fn insert_row() {
        let mut grid: Grid<char> = grid();

        grid.insert_row(1, '.');
        assert_eq!("AB\n..\nCD\n", grid.to_string());

        grid.insert_row(3, '.');
        assert_eq!("AB\n..\nCD\n..\n", grid.to_string());

        grid.insert_row(0, '.');
        assert_eq!("..\nAB\n..\nCD\n..\n", grid.to_string());
    }

    #[test]
    fn insert_column() {
        let mut grid: Grid<char> = grid();

        grid.insert_column(1, '.');
        assert_eq!("A.B\nC.D\n", grid.to_string());

        grid.insert_column(3, '.');
        assert_eq!("A.B.\nC.D.\n", grid.to_string());

        grid.insert_column(0, '.');
        assert_eq!(".A.B.\n.C.D.\n", grid.to_string());
    }

    #[test]
    fn insert_columns() {
        let mut grid: Grid<char> = grid();

        grid.insert_columns(vec![1, 2, 0], '.');

        assert_eq!(".A.B.\n.C.D.\n", grid.to_string());
    }

    #[test]
    fn insert_rows() {
        let mut grid: Grid<char> = grid();

        grid.insert_rows(vec![1, 2, 0], '.');

        assert_eq!("..\nAB\n..\nCD\n..\n", grid.to_string());
    }

    #[test]
    fn elements_with_points() {
        const GRID: &str = r#".....
..a.b
a..a.
.....
....a"#;

        let mut expected: HashMap<char, Vec<Point>> = HashMap::new();
        expected.insert(
            'a',
            vec![
                Point::new(2, 1),
                Point::new(0, 2),
                Point::new(3, 2),
                Point::new(4, 4),
            ],
        );
        expected.insert('b', vec![Point::new(4, 1)]);

        let grid: Grid<char> = Grid::from(GRID);
        let mut result = grid.elements_with_points();
        result.remove(&'.');

        for value in expected.values_mut() {
            value.sort();
        }
        for value in result.values_mut() {
            value.sort();
        }

        assert_eq!(expected, result);
    }

    fn grid() -> Grid<char> {
        let mut hash_map: HashMap<Point, char> = HashMap::new();
        hash_map.insert(Point::new(0, 0), 'A');
        hash_map.insert(Point::new(0, 1), 'C');
        hash_map.insert(Point::new(1, 1), 'D');
        hash_map.insert(Point::new(1, 0), 'B');

        Grid::new(hash_map)
    }

    fn get_chars(
        data: &BTreeMap<isize, BTreeMap<&Point, &char>>,
        row_or_column: isize,
    ) -> Vec<char> {
        data.get(&row_or_column)
            .unwrap()
            .iter()
            .map(|(_, &&c)| c)
            .collect()
    }
}
