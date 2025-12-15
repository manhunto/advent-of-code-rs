use crate::solutions::Solution;
use std::collections::HashMap;
use std::str::FromStr;

type Wires = HashMap<String, Instruction>;

pub struct Day07;

impl Solution for Day07 {
    fn part_one(&self, input: &str) -> String {
        self.signal(input, "a")
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day07 {
    fn signal(&self, input: &str, wire: &str) -> String {
        let instructions = self.parse(input);
        let main = instructions.get(wire).unwrap();
        let mut cache = HashMap::new();

        main.calculate(&instructions, &mut cache).to_string()
    }

    fn parse(&self, input: &str) -> Wires {
        input
            .lines()
            .map(|line| {
                let (instruction, output) = line.split_once(" -> ").unwrap();
                (
                    output.to_string(),
                    Instruction::from_str(instruction).unwrap(),
                )
            })
            .collect()
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    Value(Value),
    And(Value, Value),
    Or(Value, Value),
    LShift(Value, u16),
    RShift(Value, u16),
    Not(Value),
}

impl Instruction {
    fn calculate(&self, wires: &Wires, cache: &mut HashMap<String, u16>) -> u16 {
        match self {
            Instruction::Value(value) => match value {
                Value::Numeric(number) => *number,
                Value::Variable(s) => {
                    if let Some(from_cache) = cache.get(s) {
                        return *from_cache;
                    }

                    let result = wires.get(s).unwrap().calculate(wires, cache);

                    cache.insert(s.clone(), result);

                    result
                }
            },
            Instruction::And(a, b) => {
                let left = Instruction::Value(a.clone()).calculate(wires, cache);
                let right = Instruction::Value(b.clone()).calculate(wires, cache);

                left & right
            }
            Instruction::Or(a, b) => {
                let left = Instruction::Value(a.clone()).calculate(wires, cache);
                let right = Instruction::Value(b.clone()).calculate(wires, cache);

                left | right
            }
            Instruction::LShift(a, b) => {
                let left = Instruction::Value(a.clone()).calculate(wires, cache);

                left << b
            }
            Instruction::RShift(a, b) => {
                let left = Instruction::Value(a.clone()).calculate(wires, cache);

                left >> b
            }
            Instruction::Not(a) => {
                let value = Instruction::Value(a.clone()).calculate(wires, cache);

                !value
            }
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_variable = |variable: &str| {
            if let Ok(number) = variable.parse::<u16>() {
                Ok(Value::Numeric(number))
            } else {
                Ok(Value::Variable(variable.to_string()))
            }
        };

        if let Ok(number) = s.parse::<u16>() {
            return Ok(Instruction::Value(Value::Numeric(number)));
        }

        if let Some((left, right)) = s.split_once(" AND ") {
            return Ok(Instruction::And(
                parse_variable(left)?,
                parse_variable(right)?,
            ));
        }

        if let Some((left, right)) = s.split_once(" OR ") {
            return Ok(Instruction::Or(
                parse_variable(left)?,
                parse_variable(right)?,
            ));
        }

        if let Some((left, right)) = s.split_once(" LSHIFT ") {
            return Ok(Instruction::LShift(
                parse_variable(left)?,
                right.parse().map_err(|_| ())?,
            ));
        }

        if let Some((left, right)) = s.split_once(" RSHIFT ") {
            return Ok(Instruction::RShift(
                parse_variable(left)?,
                right.parse().map_err(|_| ())?,
            ));
        }

        if let Some(not) = s.strip_prefix("NOT ") {
            return Ok(Instruction::Not(parse_variable(not)?));
        }

        Ok(Instruction::Value(Value::Variable(s.to_string())))
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Value {
    Numeric(u16),
    Variable(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"123 -> x
456 -> y
y -> z
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
123 AND y -> s"#;

    #[test]
    fn part_one_example_test() {
        assert_eq!("123", Day07.signal(EXAMPLE, "x"));
        assert_eq!("456", Day07.signal(EXAMPLE, "y"));
        assert_eq!("456", Day07.signal(EXAMPLE, "z"));
        assert_eq!("72", Day07.signal(EXAMPLE, "d"));
        assert_eq!("72", Day07.signal(EXAMPLE, "s"));
        assert_eq!("507", Day07.signal(EXAMPLE, "e"));
        assert_eq!("492", Day07.signal(EXAMPLE, "f"));
        assert_eq!("114", Day07.signal(EXAMPLE, "g"));
        assert_eq!("65412", Day07.signal(EXAMPLE, "h"));
        assert_eq!("65079", Day07.signal(EXAMPLE, "i"));
    }

    #[test]
    fn instruction_parse() {
        assert_eq!(
            Ok(Instruction::Value(Value::Numeric(456))),
            Instruction::from_str("456")
        );

        assert_eq!(
            Ok(Instruction::And(
                Value::Variable("x".to_string()),
                Value::Variable("y".to_string())
            )),
            Instruction::from_str("x AND y")
        );

        assert_eq!(
            Ok(Instruction::And(
                Value::Variable("x".to_string()),
                Value::Numeric(2)
            )),
            Instruction::from_str("x AND 2")
        );

        assert_eq!(
            Ok(Instruction::Or(
                Value::Variable("x".to_string()),
                Value::Variable("y".to_string())
            )),
            Instruction::from_str("x OR y")
        );

        assert_eq!(
            Ok(Instruction::LShift(Value::Variable("x".to_string()), 2)),
            Instruction::from_str("x LSHIFT 2")
        );

        assert_eq!(
            Ok(Instruction::RShift(Value::Variable("x".to_string()), 3)),
            Instruction::from_str("x RSHIFT 3")
        );

        assert_eq!(
            Ok(Instruction::Not(Value::Variable("y".to_string()),)),
            Instruction::from_str("NOT y")
        );
    }
}
