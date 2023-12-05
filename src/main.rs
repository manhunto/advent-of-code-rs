use std::fs::read_to_string;
use std::time::Instant;

mod utils;

fn main() {
    // let _type = "example";
    let _type = "puzzle";

    let input = read_to_string(format!("src/01_{}.in", _type))
        .expect("Should be able to read this file");

    let start_part_one = Instant::now();
    let part_one: u32 = part_one(&input.as_str());
    let duration_part_one = start_part_one.elapsed();

    let start_part_two = Instant::now();
    let part_two: u32 = part_two(&input.as_str());
    let duration_part_two = start_part_two.elapsed();

    println!("Part one: {} ({:?})", part_one, duration_part_one);
    println!("Part two: {} ({:?})", part_two, duration_part_two);
}

fn part_one(input: &str) -> u32 {
    input.lines()
        .map(|l: &str| utils::calculate_line(l))
        .sum()
}

fn part_two(input: &str) -> u32 {
    input.lines()
        .map(|l: &str| {
            let new = utils::replace_words_to_numbers(l);
            utils::calculate_line(new.as_str())
        })
        .sum()
}
