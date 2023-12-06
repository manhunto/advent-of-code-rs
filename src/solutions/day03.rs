use crate::solutions::Solution;
use std::str;

pub struct Day03;

impl Solution for Day03 {
    fn part_one(&self, input: &str) -> String {
        let mut possible_parts_positions: Vec<(usize, usize)> = vec![];
        let mut numbers: Vec<Number> = vec![];

        for (y, line) in input.lines().enumerate() {

            numbers.append(&mut recognize_numbers(line, y));

            for (x, char) in line.chars().enumerate() {
                if char.is_numeric() == false && char != '.' {
                    possible_parts_positions.push((x - 1, y - 1));
                    possible_parts_positions.push((x, y - 1));
                    possible_parts_positions.push((x + 1, y - 1));

                    possible_parts_positions.push((x - 1, y));
                    possible_parts_positions.push((x + 1, y));

                    possible_parts_positions.push((x - 1, y + 1));
                    possible_parts_positions.push((x, y + 1));
                    possible_parts_positions.push((x + 1, y + 1));
                }
            }
        }

        numbers
            .iter()
            .filter(|number| number.collide_with_any(&possible_parts_positions))
            .map(|number| number.number)
            .sum::<i32>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        String::from("0")
    }
}

#[derive(PartialEq, Debug)]
struct Number {
    pub number: i32,
    pub positions: Vec<(usize, usize)>,
}

impl Number {
    fn collide_with_any(&self, positions: &Vec<(usize, usize)>) -> bool {
        for position in positions {
            if self.positions.contains(&position) {
                return true;
            }
        }

        return false;
    }
}

fn recognize_numbers(line: &str, y: usize) -> Vec<Number> {
    let mut tmp_digit_positions: Vec<(usize, usize)> = vec![];
    let mut tmp_numbers: Vec<char> = vec![];

    let mut numbers: Vec<Number> = vec![];

    for (x, char) in line.chars().enumerate() {
        if char.is_numeric() {
            tmp_digit_positions.push((x, y));
            tmp_numbers.push(char)
        } else {
            if tmp_numbers.is_empty() == false {
                numbers.push(
                    Number {
                        number: String::from_iter(&tmp_numbers).parse::<i32>().unwrap(),
                        positions: tmp_digit_positions.clone(),
                    }
                );

                tmp_numbers.clear();
                tmp_digit_positions.clear();
            }
        }
    }

    if tmp_numbers.is_empty() == false {
        numbers.push(
            Number {
                number: String::from_iter(&tmp_numbers).parse::<i32>().unwrap(),
                positions: tmp_digit_positions.clone(),
            }
        );
    }

    numbers
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day03::{Day03, Number, recognize_numbers};
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("03");

        assert_eq!("4361", Day03.part_one(&input.as_str()));
    }

    #[test]
    fn recognize_number_test() {
        assert_eq!(
            vec![
                Number {
                    number: 467,
                    positions: vec![(0, 0), (1, 0), (2, 0)],
                }
            ],
            recognize_numbers("467", 0)
        );

        assert_eq!(
            vec![
                Number {
                    number: 467,
                    positions: vec![(0, 0), (1, 0), (2, 0)],
                },
                Number {
                    number: 114,
                    positions: vec![(5, 0), (6, 0), (7, 0)],
                }
            ],
            recognize_numbers("467..114..", 0)
        );

        assert_eq!(
            vec![] as Vec<Number>,
            recognize_numbers("...*......", 0)
        );

        assert_eq!(
            vec![
                Number {
                    number: 617,
                    positions: vec![(0, 0), (1, 0), (2, 0)],
                }
            ],
            recognize_numbers("617*......", 0)
        );
    }
}
