use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct LightGrid<T> {
    cells: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> LightGrid<T>
where
    T: Clone,
{
    pub fn new(cells: Vec<T>, width: usize, height: usize) -> Self {
        assert_eq!(
            cells.len(),
            width * height,
            "cells length must equal width * height"
        );
        Self {
            cells,
            width,
            height,
        }
    }

    pub fn from_str_with<F>(input: &str, mut func: F) -> Self
    where
        F: FnMut(u8) -> T,
    {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len();
        let width = lines.first().map(|line| line.len()).unwrap_or(0);

        let cells: Vec<T> = lines
            .iter()
            .flat_map(|line| line.as_bytes())
            .map(|&c| func(c))
            .collect();

        Self::new(cells, width, height)
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            Some(&self.cells[y * self.width + x])
        } else {
            None
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < self.width && y < self.height {
            Some(&mut self.cells[y * self.width + x])
        } else {
            None
        }
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = value;
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    #[allow(dead_code)]
    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.cells.iter().enumerate().map(|(i, cell)| {
            let x = i % self.width;
            let y = i / self.width;
            (x, y, cell)
        })
    }

    #[allow(dead_code)]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut T)> {
        let width = self.width;
        self.cells.iter_mut().enumerate().map(move |(i, cell)| {
            let x = i % width;
            let y = i / width;
            (x, y, cell)
        })
    }

    pub fn map<U, F>(&self, mut func: F) -> LightGrid<U>
    where
        F: FnMut(usize, usize, &T) -> U,
        U: Clone,
    {
        let cells: Vec<U> = self
            .cells
            .iter()
            .enumerate()
            .map(|(i, cell)| {
                let x = i % self.width;
                let y = i / self.width;
                func(x, y, cell)
            })
            .collect();

        LightGrid::new(cells, self.width, self.height)
    }

    pub fn count<F>(&self, mut predicate: F) -> usize
    where
        F: FnMut(&T) -> bool,
    {
        self.cells.iter().filter(|cell| predicate(cell)).count()
    }

    pub fn adjacent_with_diagonals(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::with_capacity(8);

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                    result.push((nx as usize, ny as usize));
                }
            }
        }

        result
    }
}

impl<T> LightGrid<T>
where
    T: Clone + PartialEq,
{
    pub fn count_equal(&self, value: &T) -> usize {
        self.count(|cell| cell == value)
    }
}

impl FromStr for LightGrid<u8> {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_str_with(s, |c| c))
    }
}

impl FromStr for LightGrid<char> {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_str_with(s, |c| c as char))
    }
}

impl<T> fmt::Display for LightGrid<T>
where
    T: fmt::Display + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.get(x, y).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_grid_basic() {
        let grid = LightGrid::from_str_with("AB\nCD", |c| c);

        assert_eq!(grid.width(), 2);
        assert_eq!(grid.height(), 2);
        assert_eq!(grid.get(0, 0), Some(&b'A'));
        assert_eq!(grid.get(1, 1), Some(&b'D'));
    }

    #[test]
    fn test_light_grid_map() {
        let grid = LightGrid::from_str_with("ab\ncd", |c| c);
        let upper = grid.map(|_, _, &c| c.to_ascii_uppercase());

        assert_eq!(upper.get(0, 0), Some(&b'A'));
        assert_eq!(upper.get(1, 1), Some(&b'D'));
    }

    #[test]
    fn test_adjacent_with_diagonals() {
        let grid = LightGrid::from_str_with("...\n...\n...", |c| c);

        // Center cell should have 8 neighbors
        let neighbors = grid.adjacent_with_diagonals(1, 1);
        assert_eq!(neighbors.len(), 8);

        // Corner cell should have 3 neighbors
        let neighbors = grid.adjacent_with_diagonals(0, 0);
        assert_eq!(neighbors.len(), 3);
    }

    #[test]
    fn test_count_equal() {
        let grid = LightGrid::from_str_with(".#.#\n#..#", |c| c);
        assert_eq!(grid.count_equal(&b'#'), 4);
        assert_eq!(grid.count_equal(&b'.'), 4);
    }

    #[test]
    fn test_from_str_trait_bytes() {
        let grid: LightGrid<u8> = "AB\nCD".parse().unwrap();
        assert_eq!(grid.get(0, 0), Some(&b'A'));
        assert_eq!(grid.get(1, 1), Some(&b'D'));
    }

    #[test]
    fn test_from_str_trait_chars() {
        let grid: LightGrid<char> = "AB\nCD".parse().unwrap();
        assert_eq!(grid.get(0, 0), Some(&'A'));
        assert_eq!(grid.get(1, 1), Some(&'D'));
    }
}
