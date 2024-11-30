use crate::utils::point::Point;
use std::ops::{Add, Div};

pub fn shoelace_formula(points: &[Point]) -> isize {
    let len = points.len();

    let (area, perimeter) =
        points
            .iter()
            .enumerate()
            .fold((0isize, 0isize), |(sum, perimeter), (i, p1)| {
                let l = (i + 1) % len;
                let p2 = points[l];

                let new_perimeter = perimeter + p1.manhattan_distance(&p2);
                let new_area = sum + (p1.y * p2.x) - (p1.x * p2.y);

                (new_area, new_perimeter)
            });

    area.abs().add(perimeter).div(2).add(1)
}

#[cfg(test)]
mod tests {
    use crate::utils::point::Point;
    use crate::utils::shoelace_formula::shoelace_formula;

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
