use std::ops::{Add, Div};
use crate::point::Point;

pub fn shoelace_formula(points: &Vec<Point>) -> i32 {
    let len = points.len();
    let mut perimiter = 0;
    points
        .iter()
        .enumerate()
        .fold(0, |s, (i, p1)| {
            let l = (i + 1) % len;
            let p2 = points[l];

            perimiter += p1.manhattan_distance(&p2);

            s + (p1.y * p2.x) - (p1.x * p2.y)
        })
        .abs()
        .add(perimiter)
        .div(2)
        .add(1)
}

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use crate::shoelace_formula::{shoelace_formula};

    #[test]
    fn shoelace_formula_test() {
        assert_eq!(9, shoelace_formula(&square_2()));
        assert_eq!(25, shoelace_formula(&square_4()));
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