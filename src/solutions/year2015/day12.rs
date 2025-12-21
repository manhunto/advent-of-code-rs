use crate::solutions::Solution;

pub struct Day12;

impl Solution for Day12 {
    fn part_one(&self, input: &str) -> String {
        input
            .trim()
            .split(|char| {
                ['[', ']', '{', '}', ',', '"', ':'].contains(&char) || char.is_alphabetic()
            })
            .filter(|s| !s.is_empty())
            .map(|d| d.parse::<i64>().unwrap())
            .sum::<i64>()
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
        assert_eq!("6", Day12.part_one("[1,2,3]"));
        assert_eq!("6", Day12.part_one(r#"{"a":2,"b":4}"#));
        assert_eq!("3", Day12.part_one(r#"[[[3]]]"#));
        assert_eq!("3", Day12.part_one(r#"{"a":{"b":4},"c":-1}"#));
        assert_eq!("0", Day12.part_one(r#"{"a":[-1,1]}"#));
        assert_eq!("0", Day12.part_one(r#"[-1,{"a":1}]"#));
        assert_eq!("0", Day12.part_one(r#"{}"#));
    }
}
