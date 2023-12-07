use std::time::Instant;
use crate::solutions::{get_solutions};

mod file_system;
mod solutions;

fn main() {
    let solutions = get_solutions();
    let solution = &solutions[3];

    let input = file_system::read_input("04");

    let start_part_one = Instant::now();
    let part_one: String = solution.part_one(&input.as_str());
    let duration_part_one = start_part_one.elapsed();

    let start_part_two = Instant::now();
    let part_two: String = solution.part_two(&input.as_str());
    let duration_part_two = start_part_two.elapsed();

    println!("Part one: {} ({:?})", part_one, duration_part_one);
    println!("Part two: {} ({:?})", part_two, duration_part_two);
}
