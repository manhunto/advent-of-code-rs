use crate::solutions::Solution;
use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;

pub struct Day24;

impl Solution for Day24 {
    fn part_one(&self, input: &str) -> String {
        let switches = self.parse(input);
        let z = switches
            .iter()
            .filter(|(k, _)| k.starts_with('z'))
            .map(|(k, v)| (k.clone(), v.resolve(&switches)))
            .collect::<HashMap<_, _>>();

        let mut sorted: Vec<(&String, &u64)> = z.iter().collect();
        sorted.sort_by(|a, b| b.0.cmp(a.0));

        sorted
            .iter()
            .fold(0u64, |acc, (_, &v)| (acc << 1) | v)
            .to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day24 {
    fn parse(&self, input: &str) -> HashMap<String, Switch> {
        let (values, gates) = input.split_once("\n\n").unwrap();

        let values = values.lines().map(|line| {
            let (key, value) = line.split_once(": ").unwrap();

            (key.to_string(), Switch::Value(value.parse().unwrap()))
        });

        let gates = gates.lines().map(|line| {
            let (inputs, output) = line.split_once(" -> ").unwrap();

            (output.to_string(), Switch::Gate(inputs.parse().unwrap()))
        });

        values.chain(gates).collect()
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
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

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
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

#[derive(Debug)]
enum Switch {
    Value(u64),
    Gate(Gate),
}

impl Switch {
    fn resolve(&self, switches: &HashMap<String, Switch>) -> u64 {
        match self {
            Switch::Value(value) => *value,
            Switch::Gate(gate) => {
                let value1 = switches[&gate.input1].resolve(switches);
                let value2 = switches[&gate.input2].resolve(switches);

                match gate.gate_type {
                    GateType::And => value1 & value2,
                    GateType::Or => value1 | value2,
                    GateType::Xor => value1 ^ value2,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day24::Day24;
    use crate::solutions::Solution;

    const SIMPLE_EXAMPLE: &str = r#"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"#;

    #[test]
    fn part_one_simple_example() {
        assert_eq!("4", Day24.part_one(SIMPLE_EXAMPLE));
    }

    const LARGER_EXAMPLE: &str = r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
"#;

    #[test]
    fn part_one_larger_example() {
        assert_eq!("2024", Day24.part_one(LARGER_EXAMPLE));
    }
}
