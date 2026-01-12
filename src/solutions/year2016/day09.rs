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
            } else if self.chars[i] == '(' {
                let (capture_length, repeat) = self.capture_marker(&self.chars, &mut i);

                length += capture_length * repeat;
                i += capture_length;
            }

            i += 1;
        }

        length
    }

    fn capture_marker(&self, chars: &[char], i: &mut usize) -> (usize, usize) {
        *i += 1;

        let marker = chars
            .iter()
            .skip(*i)
            .take_while(|c| **c != ')')
            .collect::<String>();

        *i += marker.len();

        self.parse_marker(&marker)
    }

    fn decompressed_length_v2(self) -> usize {
        self.decompressed_length_of_slice(&self.chars)
    }

    fn decompressed_length_of_slice(&self, chars: &[char]) -> usize {
        let mut i = 0;
        let mut length = 0;

        while i < chars.len() {
            if chars[i].is_ascii_uppercase() {
                length += 1;
            } else if chars[i] == '(' {
                let (capture_length, repeat) = self.capture_marker(chars, &mut i);

                let slice = &chars[i + 1..i + 1 + capture_length];

                length += self.decompressed_length_of_slice(slice) * repeat;
                i += capture_length;
            }

            i += 1;
        }

        length
    }

    fn parse_marker(&self, marker: &str) -> (usize, usize) {
        marker
            .split_once('x')
            .and_then(|(len, rep)| Some((len.parse().ok()?, rep.parse().ok()?)))
            .unwrap()
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

    #[test]
    fn part_two_example() {
        assert_eq!("9", Day09.part_two("(3x3)XYZ"));
        assert_eq!("20", Day09.part_two("X(8x2)(3x3)ABCY"));
        assert_eq!(
            "241920",
            Day09.part_two("(27x12)(20x12)(13x14)(7x10)(1x12)A")
        );
        assert_eq!(
            "445",
            Day09.part_two("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN")
        );
    }
}
