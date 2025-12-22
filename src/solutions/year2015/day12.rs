use crate::solutions::Solution;
use serde_json::Value;
use std::str::FromStr;

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

    fn part_two(&self, input: &str) -> String {
        input.parse::<Json>().unwrap().sum_without_red().to_string()
    }
}

#[derive(Debug)]
struct Json {
    value: Value,
}

impl Json {
    fn sum_without_red(&self) -> i64 {
        self.walk(&self.value)
    }

    fn walk(&self, part: &Value) -> i64 {
        match part {
            Value::Array(vec) => vec.iter().map(|item| self.walk(item)).sum(),
            Value::Number(n) => n.as_i64().unwrap(),
            Value::Object(object) => {
                if object
                    .values()
                    .any(|item| matches!(item, Value::String(n) if n == "red"))
                {
                    return 0;
                }

                object.values().map(|item| self.walk(item)).sum()
            }
            _ => 0,
        }
    }
}

impl FromStr for Json {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            value: serde_json::from_str(s).unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example_test() {
        assert_eq!("6", Day12.part_one(r#"[1,2,3]"#));
        assert_eq!("6", Day12.part_one(r#"{"a":2,"b":4}"#));
        assert_eq!("3", Day12.part_one(r#"[[[3]]]"#));
        assert_eq!("3", Day12.part_one(r#"{"a":{"b":4},"c":-1}"#));
        assert_eq!("0", Day12.part_one(r#"{"a":[-1,1]}"#));
        assert_eq!("0", Day12.part_one(r#"[-1,{"a":1}]"#));
        assert_eq!("0", Day12.part_one(r#"{}"#));
    }

    #[test]
    fn part_two_example_test() {
        assert_eq!("6", Day12.part_two(r#"[1,2,3]"#));
        assert_eq!("4", Day12.part_two(r#"[1,{"c":"red","b":2},3]"#));
        assert_eq!("0", Day12.part_two(r#"{"d":"red","e":[1,2,3,4],"f":5}"#));
        assert_eq!("6", Day12.part_two(r#"[1,"red",5]"#));
    }
}
