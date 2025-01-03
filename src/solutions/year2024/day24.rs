use crate::solutions::Solution;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Day24;

impl Solution for Day24 {
    fn part_one(&self, input: &str) -> String {
        let (values, gates) = input.split_once("\n\n").unwrap();

        let values: HashMap<String, bool> = values
            .lines()
            .map(|line| {
                let (key, value) = line.split_once(": ").unwrap();
                (key.to_string(), value == "1")
            })
            .collect();

        let gates: HashMap<String, Gate> = gates
            .lines()
            .map(|line| {
                let (inputs, output) = line.split_once(" -> ").unwrap();

                (output.to_string(), inputs.parse().unwrap())
            })
            .collect();

        println!("{:?}", values);
        println!("{:?}", gates);

        String::from("0")
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum GateType {
    And,
    Or,
    Xor,
}

impl FromStr for GateType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(GateType::And),
            "OR" => Ok(GateType::Or),
            "XOR" => Ok(GateType::Xor),
            _ => panic!("Invalid gate type"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Gate {
    input1: String,
    input2: String,
    gate_type: GateType,
}

impl FromStr for Gate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        Ok(Gate {
            input1: parts[0].to_string(),
            input2: parts[2].to_string(),
            gate_type: parts[1].parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day24::Day24;
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"#;

    #[test]
    #[ignore]
    fn part_one_simple_example() {
        assert_eq!("4", Day24.part_one(EXAMPLE));
    }
}
