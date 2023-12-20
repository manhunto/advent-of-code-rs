use std::ops::{Div, Mul, Sub};
use crate::point::Point;

pub fn shoelace_formula(points: &Vec<Point>) -> i32 {
    let len = points.len();

    points
        .iter()
        .enumerate()
        .fold(0, |s, (i, p)| {
            let l = (i + 1) % len;

            s + (p.y * points[l].x) - (p.x * points[l].y)
        })
        .abs()
        .div(2)
}

pub fn shoelace_formula_without_border(points: &Vec<Point>) -> i32 {
    shoelace_formula(points)
        .mul(2)
        .sub(points.len() as i32)
        .div(2)
}

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use crate::shoelace_formula::{shoelace_formula, shoelace_formula_without_border};

    #[test]
    fn shoelace_formula_test() {
        assert_eq!(4, shoelace_formula(&square_2()));
        assert_eq!(16, shoelace_formula(&square_4()));
    }

    #[test]
    fn shoelace_formula_without_border_test() {
        assert_eq!(2, shoelace_formula_without_border(&square_2()));
        assert_eq!(14, shoelace_formula_without_border(&square_4()));
    }

    fn square_2() -> Vec<Point> {
        vec![
            Point::new(0, 0),
            Point::new(0, 2),
            Point::new(2, 2),
            Point::new(2, 0),
        ]
    }

    fn square_4() -> Vec<Point> {
        vec![
            Point::new(0, 0),
            Point::new(0, 4),
            Point::new(4, 4),
            Point::new(4, 0),
        ]
    }
}