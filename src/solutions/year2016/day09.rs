use crate::solutions::Solution;

pub struct Day09;

impl Solution for Day09 {
    fn part_one(&self, input: &str) -> String {
        File::v1(input).decompressed_length().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        File::v2(input).decompressed_length().to_string()
    }
}

struct File {
    chars: Vec<char>,
    version: Version,
}

impl File {
    fn v1(data: &str) -> Self {
        Self {
            chars: data.trim().chars().collect::<Vec<char>>(),
            version: Version::V1,
        }
    }

    fn v2(data: &str) -> Self {
        Self {
            chars: data.trim().chars().collect::<Vec<char>>(),
            version: Version::V2,
        }
    }

    fn decompressed_length(self) -> usize {
        match self.version {
            Version::V1 => self.decompressed_length_v1(),
            Version::V2 => self.decompressed_length_v2(),
        }
    }

    fn decompressed_length_v1(self) -> usize {
        let mut i = 0;
        let mut length = 0;

        while i < self.chars.len() {
            if self.chars[i].is_ascii_uppercase() {
                length += 1;
            }

            if self.chars[i] == '(' {
                let (count, times) = self.capture_marker(&mut i);

                length += count * times;
                i += count;
            }

            i += 1;
        }

        length
    }

    fn capture_marker(&self, i: &mut usize) -> (usize, usize) {
        let mut capture = Vec::new();

        *i += 1;

        while self.chars[*i] != ')' {
            capture.push(self.chars[*i]);
            *i += 1;
        }

        let marker = capture.iter().collect::<String>();
        let (c, t) = marker.split_once('x').unwrap();
        let count = c.parse::<usize>().unwrap();
        let times = t.parse::<usize>().unwrap();

        (count, times)
    }

    fn decompressed_length_v2(self) -> usize {
        0
    }
}

enum Version {
    V1,
    V2,
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
