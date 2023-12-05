use std::fs::read_to_string;
use std::time::Instant;

mod utils;

fn main() {
    // let _type = "example";
    let _type = "puzzle";

    let input = read_to_string(format!("src/01_{}.in", _type))
        .expect("Should be able to read this file");

    let lines1: Vec<&str> = input.split("\n").collect();

    let start_part_one = Instant::now();
    let part_one: u32 = part_one(&lines1);
    let duration_part_one = start_part_one.elapsed();

    let start_part_two = Instant::now();
    let part_two: u32 = part_two(&lines1);
    let duration_part_two = start_part_two.elapsed();

    println!("Part one: {} ({:?})", part_one, duration_part_one);
    println!("Part two: {} ({:?})", part_two, duration_part_two);
}

fn part_one(_lines: &Vec<&str>) -> u32 {
    let mut sum: u32 = 0;

    for line in _lines {
        sum += utils::calculate_line(line);
    }

    sum
}

fn part_two(_lines: &Vec<&str>) -> u32 {
    let mut sum: u32 = 0;

    for line in _lines {
        let new = utils::replace_words_to_numbers(&line);

        let val = utils::calculate_line(new.as_str());
        println!("{} -> {} -> {}", line, new, val);
        sum += val;
    }

    sum
}
