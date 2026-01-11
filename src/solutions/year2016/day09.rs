use crate::solutions::Solution;

pub struct Day09;

impl Solution for Day09 {
    fn part_one(&self, input: &str) -> String {
        let mut i = 0;
        let mut length = 0;
        let chars = input.trim().chars().collect::<Vec<char>>();

        while i < chars.len() {
            if chars[i].is_ascii_uppercase() {
                length += 1;
            }

            if chars[i] == '(' {
                let mut capture = Vec::new();

                i += 1;

                while chars[i] != ')' {
                    capture.push(chars[i]);
                    i += 1;
                }

                let marker = capture.iter().collect::<String>();
                let (c, t) = marker.split_once('x').unwrap();
                let count = c.parse::<usize>().unwrap();
                let times = t.parse::<usize>().unwrap();

                length += count * times;
                i += count;
            }

            i += 1;
        }

        length.to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        assert_eq!("6", Day09.part_one("ADVENT"));
        assert_eq!("7", Day09.part_one("A(1x5)BC"));
        assert_eq!("9", Day09.part_one("(3x3)XYZ"));
        assert_eq!("11", Day09.part_one("A(2x2)BCD(2x2)EFG"));
        assert_eq!("6", Day09.part_one("(6x1)(1x3)A"));
        assert_eq!("18", Day09.part_one("X(8x2)(3x3)ABCY"));
    }
}
