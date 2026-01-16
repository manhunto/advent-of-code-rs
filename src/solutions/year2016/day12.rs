use crate::solutions::Solution;
use std::ops::{Index, IndexMut};
use std::str::FromStr;
use Instruction::{Copy, Decrement, Increment, JumpNoZero};

pub struct Day12;

impl Solution for Day12 {
    fn part_one(&self, input: &str) -> String {
        let mut cpu = Cpu::default();

        self.apply_instructions(&mut cpu, input);

        cpu.get_registry_value(&Registry::A).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut cpu = Cpu::default();
        cpu.set_registry_value(&Registry::C, 1);

        self.apply_instructions(&mut cpu, input);

        cpu.get_registry_value(&Registry::A).to_string()
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
enum Value {
    Registry(Registry),
    Numeric(i32),
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i32>()
            .map(Value::Numeric)
            .or_else(|_| Ok(Value::Registry(s.as_bytes()[0].into())))
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Copy(Value, Registry),
    Increment(Registry),
    Decrement(Registry),
    JumpNoZero(Value, i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        Ok(match parts[0] {
            "cpy" => Copy(parts[1].parse()?, parts[2].as_bytes()[0].into()),
            "inc" => Increment(parts[1].as_bytes()[0].into()),
            "dec" => Decrement(parts[1].as_bytes()[0].into()),
            "jnz" => JumpNoZero(parts[1].parse()?, parts[2].parse().unwrap()),
            _ => unreachable!(),
        })
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
            Copy(value, r) => self[*r] = self.value(value),
            Increment(r) => self[*r] += 1,
            Decrement(r) => self[*r] -= 1,
            _ => {}
        };

        self.index = match instruction {
            JumpNoZero(value, jump) if self.value(value) != 0 => self.index + *jump,
            _ => self.index + 1,
        };
    }

    fn value(&self, value: &Value) -> i32 {
        match value {
            Value::Numeric(v) => *v,
            Value::Registry(r) => self[*r],
        }
    }

    fn get_registry_value(&self, registry: &Registry) -> i32 {
        self[*registry]
    }

    fn set_registry_value(&mut self, registry: &Registry, value: i32) {
        self[*registry] = value;
    }
}

impl Index<Registry> for Cpu {
    type Output = i32;
    fn index(&self, r: Registry) -> &i32 {
        &self.registers[r as usize]
    }
}

impl IndexMut<Registry> for Cpu {
    fn index_mut(&mut self, r: Registry) -> &mut i32 {
        &mut self.registers[r as usize]
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
