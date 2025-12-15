use crate::solutions::Solution;

pub struct Day08;

impl Solution for Day08 {
    fn part_one(&self, input: &str) -> String {
        input
            .lines()
            .map(|l| {
                let mut i: usize = 1;
                let len = l.len();
                let mut characters: usize = 0;

                loop {
                    if i == len - 1 {
                        break;
                    }

                    if l.chars().nth(i).unwrap() == '\\' {
                        if l.chars().nth(i + 1).unwrap() == 'x' {
                            i += 3;
                        } else {
                            i += 1;
                        }
                    }

                    characters += 1;

                    i += 1;
                }

                len - characters
            })
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example_test() {
        assert_eq!("2", Day08.part_one("\"\""));
        assert_eq!("2", Day08.part_one("\"abc\""));
        assert_eq!("3", Day08.part_one("\"aaa\\\"aaa\""));
        assert_eq!("5", Day08.part_one("\"\\x27\""));
    }
}
