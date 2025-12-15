use crate::solutions::Solution;

pub struct Day08;

impl Solution for Day08 {
    fn part_one(&self, input: &str) -> String {
        input
            .lines()
            .map(|word| {
                let result = Self::process(word);

                result.code_length - result.memory_length
            })
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        input
            .lines()
            .map(|word| {
                let encoded_string = format!("\"{}\"", word.escape_debug());

                let original = Self::process(word);
                let encoded = Self::process(&encoded_string);

                encoded.code_length - original.code_length
            })
            .sum::<usize>()
            .to_string()
    }
}

impl Day08 {
    fn process(word: &str) -> Result {
        let mut i: usize = 1;
        let len = word.len();
        let mut memory: usize = 0;

        loop {
            if i == len - 1 {
                break;
            }

            if word.chars().nth(i).unwrap() == '\\' {
                if word.chars().nth(i + 1).unwrap() == 'x' {
                    i += 3;
                } else {
                    i += 1;
                }
            }

            memory += 1;

            i += 1;
        }

        Result {
            code_length: len,
            memory_length: memory,
        }
    }
}

struct Result {
    code_length: usize,
    memory_length: usize,
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

    #[test]
    fn part_two_example_test() {
        assert_eq!("4", Day08.part_two("\"\""));
        assert_eq!("4", Day08.part_two("\"abc\""));
        assert_eq!("6", Day08.part_two("\"aaa\\\"aaa\""));
        assert_eq!("5", Day08.part_two("\"\\x27\""));
    }
}
