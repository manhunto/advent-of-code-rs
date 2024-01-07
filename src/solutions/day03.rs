use crate::solutions::Solution;
use std::str;

pub struct Day03;

impl Solution for Day03 {
    fn part_one(&self, input: &str) -> String {
        let mut symbols: Vec<Symbol> = vec![];
        let mut numbers: Vec<Number> = vec![];

        for (y, line) in input.lines().enumerate() {
            numbers.append(&mut recognize_numbers(line, y as i32));

            for (x, char) in line.chars().enumerate() {
                if !char.is_numeric() && char != '.' {
                    symbols.push(Symbol::new(x as i32, y as i32));
                }
            }
        }

        numbers
            .iter()
            .filter(|number| number.collide_with_any(&symbols))
            .map(|number| number.number)
            .sum::<i32>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut symbols: Vec<Symbol> = vec![];
        let mut numbers: Vec<Number> = vec![];

        for (y, line) in input.lines().enumerate() {
            numbers.append(&mut recognize_numbers(line, y as i32));

            for (x, char) in line.chars().enumerate() {
                if char == '*' {
                    symbols.push(Symbol::new(x as i32, y as i32));
                }
            }
        }

        symbols
            .iter()
            .map(|x| {
                let collisions = numbers.iter().filter_map(|number| {
                    if number.collide_with(x) {
                        Some(number.number)
                    } else {
                        None
                    }
                });

                if collisions.clone().count() == 2 {
                    return collisions.product();
                }

                0
            })
            .sum::<i32>()
            .to_string()
    }
}

#[derive(PartialEq, Debug)]
struct Number {
    pub number: i32,
    pub positions: Vec<(i32, i32)>,
}

impl Number {
    fn collide_with_any(&self, symbols: &[Symbol]) -> bool {
        symbols.iter().any(|s| self.collide_with(s))
    }

    fn collide_with(&self, symbol: &Symbol) -> bool {
        symbol
            .all_positions()
            .iter()
            .any(|p| self.positions.contains(p))
    }
}

struct Symbol {
    adjacents: [(i32, i32); 9],
}

impl Symbol {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            adjacents: [
                (x - 1, y - 1),
                (x, y - 1),
                (x + 1, y - 1),
                (x - 1, y),
                (x, y),
                (x + 1, y),
                (x - 1, y + 1),
                (x, y + 1),
                (x + 1, y + 1),
            ],
        }
    }

    pub fn all_positions(&self) -> &[(i32, i32)] {
        &self.adjacents
    }
}

fn recognize_numbers(line: &str, y: i32) -> Vec<Number> {
    let mut tmp_digit_positions: Vec<(i32, i32)> = vec![];
    let mut tmp_numbers: Vec<char> = vec![];

    let mut numbers: Vec<Number> = vec![];

    for (x, char) in line.chars().enumerate() {
        if char.is_numeric() {
            tmp_digit_positions.push((x as i32, y));
            tmp_numbers.push(char)
        } else if !tmp_numbers.is_empty() {
            numbers.push(Number {
                number: String::from_iter(&tmp_numbers).parse::<i32>().unwrap(),
                positions: tmp_digit_positions.clone(),
            });

            tmp_numbers.clear();
            tmp_digit_positions.clear();
        }
    }

    if !tmp_numbers.is_empty() {
        numbers.push(Number {
            number: String::from_iter(&tmp_numbers).parse::<i32>().unwrap(),
            positions: tmp_digit_positions.clone(),
        });
    }

    numbers
}

#[cfg(test)]
mod tests {
    use crate::file_system::read_example;
    use crate::solutions::day03::{recognize_numbers, Day03, Number};
    use crate::solutions::Solution;

    #[test]
    fn part_one_example_test() {
        let input = read_example("03");

        assert_eq!("4361", Day03.part_one(input.as_str()));
    }

    #[test]
    fn part_two_example_test() {
        let input = read_example("03");

        assert_eq!("467835", Day03.part_two(input.as_str()));
    }

    #[test]
    fn recognize_number_test() {
        assert_eq!(
            vec![Number {
                number: 467,
                positions: vec![(0, 0), (1, 0), (2, 0)],
            }],
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
                },
            ],
            recognize_numbers("467..114..", 0)
        );

        assert_eq!(vec![] as Vec<Number>, recognize_numbers("...*......", 0));

        assert_eq!(
            vec![Number {
                number: 617,
                positions: vec![(0, 0), (1, 0), (2, 0)],
            }],
            recognize_numbers("617*......", 0)
        );
    }
}
