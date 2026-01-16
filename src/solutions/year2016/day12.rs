use crate::solutions::Solution;
use std::str::FromStr;
use Instruction::{Copy, Decrement, Increment, JumpNoZero};

pub struct Day12;

impl Solution for Day12 {
    fn part_one(&self, input: &str) -> String {
        let mut cpu = Cpu::default();

        self.apply_instructions(&mut cpu, input);

        cpu.get_registry_value(&Registry::A).to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day12 {
    fn apply_instructions(&self, cpu: &mut Cpu, input: &str) {
        let instructions = self.parse_instructions(input);

        while let Some(instruction) = instructions.get(cpu.index as usize) {
            cpu.run(instruction);
        }
    }

    fn parse_instructions(&self, input: &str) -> Vec<Instruction> {
        input.lines().map(|l| l.parse().unwrap()).collect()
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Registry {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}

impl From<u8> for Registry {
    fn from(value: u8) -> Self {
        match value {
            b'a' => Self::A,
            b'b' => Self::B,
            b'c' => Self::C,
            b'd' => Self::D,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum ValueType {
    Registry(Registry),
    Value(i32),
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Copy(Registry, ValueType),
    Increment(Registry),
    Decrement(Registry),
    JumpNoZero(ValueType, i32),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..3] {
            "cpy" => {
                let (v, r) = s[4..].split_once(' ').unwrap();
                let to: Registry = r.as_bytes()[0].into();

                if let Ok(n) = v.parse::<i32>() {
                    return Ok(Copy(to, ValueType::Value(n)));
                }

                let from: Registry = v.as_bytes()[0].into();
                Ok(Copy(to, ValueType::Registry(from)))
            }
            "inc" => Ok(Increment(s.as_bytes()[4].into())),
            "dec" => Ok(Decrement(s.as_bytes()[4].into())),
            "jnz" => {
                let (v, r) = s[4..].split_once(' ').unwrap();
                let jump: i32 = r.parse().unwrap();

                if let Ok(n) = v.parse::<i32>() {
                    return Ok(JumpNoZero(ValueType::Value(n), jump));
                }

                let register: Registry = v.as_bytes()[0].into();
                Ok(JumpNoZero(ValueType::Registry(register), jump))
            }
            _ => Err("Invalid instruction".to_string()),
        }
    }
}

#[derive(Default)]
struct Cpu {
    registers: [i32; 4],
    index: i32,
}

impl Cpu {
    fn run(&mut self, instruction: &Instruction) {
        match instruction {
            Copy(r, v) => {
                self.registers[*r as usize] = match v {
                    ValueType::Registry(r) => self.registers[*r as usize],
                    ValueType::Value(v) => *v,
                }
            }
            Increment(r) => self.registers[*r as usize] += 1,
            Decrement(r) => self.registers[*r as usize] -= 1,
            _ => {}
        };

        self.index = match instruction {
            JumpNoZero(r, jump) => match r {
                ValueType::Registry(r) if self.get_registry_value(r) != 0 => self.index + *jump,
                ValueType::Value(v) if *v != 0 => self.index + *jump,
                _ => self.index + 1,
            },
            _ => self.index + 1,
        };
    }

    fn get_registry_value(&self, registry: &Registry) -> i32 {
        self.registers[*registry as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a"#;

    #[test]
    fn part_one_example() {
        assert_eq!("42", Day12.part_one(EXAMPLE));
    }
}
