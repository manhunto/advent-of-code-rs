use crate::solutions::Solution;
use crate::utils::pair_generator::unique_pairs;
use itertools::Itertools;
use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;

type Switches = HashMap<String, Switch>;

pub struct Day24;

impl Solution for Day24 {
    fn part_one(&self, input: &str) -> String {
        let switches = self.parse(input);

        self.resolve(&switches).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let switches = self.parse(input);

        let result_x = self.value_for(&switches, 'x');
        let result_y = self.value_for(&switches, 'y');
        let expected = result_x & result_y;

        let possible_changes: Vec<String> = switches
            .keys()
            .filter(|k| !k.starts_with(['x', 'y']))
            .cloned()
            .collect();

        const PAIR_SIZE: usize = 2; // todo: in the input there is 4 as pair size

        let pairs = unique_pairs(possible_changes, PAIR_SIZE);

        for unique_pairs in pairs {
            let mut replaced = switches.clone();
            for (left, right) in &unique_pairs {
                if let (Some(val1), Some(val2)) = (replaced.remove(left), replaced.remove(right)) {
                    replaced.insert(left.clone(), val2);
                    replaced.insert(right.clone(), val1);
                }
            }

            let result = self.resolve(&replaced);
            if result == expected {
                let mut answer: Vec<String> = unique_pairs
                    .into_iter()
                    .flat_map(|(l, r)| vec![l, r])
                    .collect();

                answer.sort_unstable();

                return answer.iter().join(",");
            }
        }

        unreachable!()
    }
}

impl Day24 {
    fn parse(&self, input: &str) -> Switches {
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

    fn resolve(&self, switches: &Switches) -> u64 {
        let z = switches
            .iter()
            .filter(|(k, _)| k.starts_with('z'))
            .map(|(k, v)| (k.clone(), v.resolve(switches)))
            .collect::<HashMap<_, _>>();

        self.hash_map_to_u64(z)
    }

    fn value_for(&self, switches: &Switches, key: char) -> u64 {
        let starts_with = switches
            .iter()
            .filter(|(k, _)| k.starts_with(key))
            .map(|(k, v)| {
                (
                    k.clone(),
                    match v {
                        Switch::Value(value) => *value,
                        _ => panic!("Invalid switch"),
                    },
                )
            })
            .collect::<HashMap<_, _>>();

        self.hash_map_to_u64(starts_with)
    }

    fn hash_map_to_u64(&self, resolved: HashMap<String, u64>) -> u64 {
        let mut sorted: Vec<(&String, &u64)> = resolved.iter().collect();
        sorted.sort_by(|a, b| b.0.cmp(a.0));

        sorted.iter().fold(0u64, |acc, (_, &v)| (acc << 1) | v)
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

#[derive(Debug, Clone)]
enum Switch {
    Value(u64),
    Gate(Gate),
}

impl Switch {
    fn resolve(&self, switches: &Switches) -> u64 {
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

    const PART_TWO_EXAMPLE: &str = r#"x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00"#;

    #[test]
    fn part_two_example() {
        assert_eq!("z00,z01,z02,z05", Day24.part_two(PART_TWO_EXAMPLE));
    }
}
