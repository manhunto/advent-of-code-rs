use crate::solutions::Solution;
use crate::utils::point3d::Point3D;
use itertools::Itertools;

const INPUT_CONNECTIONS: usize = 1_000;

type Pair = (Point3D, Point3D);

pub struct Day08 {
    connections: usize,
}

impl Solution for Day08 {
    fn part_one(&self, input: &str) -> String {
        let junction_boxes = self.parse(input);
        let mut circuits: Vec<Vec<Point3D>> = Vec::new();

        for pair in self.closest_limited(junction_boxes) {
            let left_circuit = circuits
                .iter()
                .position(|circuit| circuit.contains(&pair.0));

            let right_circuit = circuits
                .iter()
                .position(|circuit| circuit.contains(&pair.1));

            match (left_circuit, right_circuit) {
                (Some(left), Some(right)) => {
                    if left == right {
                        continue;
                    }

                    for in_circuit in circuits[right].clone() {
                        circuits[left].push(in_circuit);
                    }
                    circuits.remove(right);
                }
                (None, Some(right)) => circuits[right].push(pair.0),
                (Some(left), None) => circuits[left].push(pair.1),
                (None, None) => {
                    circuits.push(vec![pair.0, pair.1]);
                }
            }
        }

        circuits
            .iter()
            .map(|circuit| circuit.len())
            .sorted()
            .rev()
            .take(3)
            .product::<usize>()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day08 {
    fn parse(&self, input: &str) -> Vec<Point3D> {
        input.lines().map(|line| line.parse().unwrap()).collect()
    }

    fn closest_limited(&self, boxes: Vec<Point3D>) -> impl Iterator<Item = Pair> {
        self.closest_all(boxes).take(self.connections)
    }

    fn closest_all(&self, boxes: Vec<Point3D>) -> impl Iterator<Item = Pair> {
        let mut calculated: Vec<(f64, Pair)> = Vec::new();
        for i in 0..boxes.len() {
            for j in i + 1..boxes.len() {
                calculated.push((boxes[i].distance(&boxes[j]), (boxes[i], boxes[j])));
            }
        }

        calculated.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        calculated.into_iter().map(|x| x.1)
    }
}

impl Default for Day08 {
    fn default() -> Self {
        Self {
            connections: INPUT_CONNECTIONS,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2025::day08::Day08;
    use crate::solutions::Solution;

    const TEST_CONNECTIONS: usize = 10;

    impl Day08 {
        fn new_for_tests() -> Self {
            Self {
                connections: TEST_CONNECTIONS,
            }
        }
    }

    const EXAMPLE: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("40", Day08::new_for_tests().part_one(EXAMPLE));
    }
}
