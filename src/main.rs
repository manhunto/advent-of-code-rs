use std::fs::read_to_string;
use std::time::Instant;

mod utils;

fn main() {
    // let _type = "example";
    let _type = "puzzle";

    let input = read_to_string(format!("src/01_{}.in", _type))
        .expect("Should be able to read this file");

    let start_part_one = Instant::now();
    let part_one: String = part_one(&input.as_str());
    let duration_part_one = start_part_one.elapsed();

    let start_part_two = Instant::now();
    let part_two: String = part_two(&input.as_str());
    let duration_part_two = start_part_two.elapsed();

    println!("Part one: {} ({:?})", part_one, duration_part_one);
    println!("Part two: {} ({:?})", part_two, duration_part_two);
}

fn part_one(input: &str) -> String {
    input.lines()
        .map(|line: &str| utils::calculate_line(line))
        .sum::<u32>()
        .to_string()
}

fn part_two(input: &str) -> String {
    input.lines()
        .map(|l: &str| {
            let new = utils::replace_words_to_numbers(l);
            utils::calculate_line(new.as_str())
        })
        .sum::<u32>()
        .to_string()
}
