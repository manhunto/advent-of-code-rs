use crate::solutions::year2015::day23::Instruction::{
    Half, Increment, Jump, JumpIfEven, JumpIfOne, Triple,
};
use crate::solutions::year2015::day23::Registry::{A, B};
use crate::solutions::Solution;
use std::str::FromStr;

pub struct Day23;

impl Solution for Day23 {
    fn part_one(&self, input: &str) -> String {
        let mut cpu = Cpu::default();

        self.apply_instructions(&mut cpu, input);

        cpu.get_registry_value(&B).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut cpu = Cpu::default();
        cpu.set_registry_value(&A, 1);

        self.apply_instructions(&mut cpu, input);

        cpu.get_registry_value(&B).to_string()
    }
}

impl Day23 {
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

#[derive(Debug, PartialEq)]
enum Instruction {
    Half(Registry),
    Triple(Registry),
    Increment(Registry),
    Jump(i32),
    JumpIfEven(Registry, i32),
    JumpIfOne(Registry, i32),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..3] {
            "hlf" => Ok(Half(s.as_bytes()[4].into())),
            "tpl" => Ok(Triple(s.as_bytes()[4].into())),
            "inc" => Ok(Increment(s.as_bytes()[4].into())),
            "jmp" => Ok(Jump(s[4..].parse().unwrap())),
            "jie" => Ok(JumpIfEven(s.as_bytes()[4].into(), s[7..].parse().unwrap())),
            "jio" => Ok(JumpIfOne(s.as_bytes()[4].into(), s[7..].parse().unwrap())),
            _ => Err("Invalid instruction".to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Registry {
    A = 0,
    B = 1,
}

impl From<u8> for Registry {
    fn from(value: u8) -> Self {
        match value {
            b'a' => A,
            b'b' => B,
            _ => unreachable!(),
        }
    }
}

#[derive(Default)]
struct Cpu {
    registers: [i32; 2],
    index: i32,
}

impl Cpu {
    fn run(&mut self, instruction: &Instruction) {
        match instruction {
            Half(r) => self.registers[*r as usize] /= 2,
            Triple(r) => self.registers[*r as usize] *= 3,
            Increment(r) => self.registers[*r as usize] += 1,
            _ => {}
        };

        self.index = match instruction {
            Jump(jump) => self.index + jump,
            JumpIfEven(r, jump) if self.get_registry_value(r) % 2 == 0 => self.index + jump,
            JumpIfOne(r, jump) if self.get_registry_value(r) == 1 => self.index + jump,
            _ => self.index + 1,
        };
    }

    fn get_registry_value(&self, registry: &Registry) -> i32 {
        self.registers[*registry as usize]
    }

    fn set_registry_value(&mut self, registry: &Registry, value: i32) {
        self.registers[*registry as usize] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"inc a
jio a, +2
tpl a
inc a"#;

    #[test]
    fn apply_instruction() {
        let mut cpu = Cpu::default();
        Day23.apply_instructions(&mut cpu, EXAMPLE);

        assert_eq!(cpu.get_registry_value(&A), 2);
    }

    #[test]
    fn instruction_parse() {
        assert_eq!("hlf a".parse::<Instruction>().unwrap(), Half(A));
        assert_eq!("tpl b".parse::<Instruction>().unwrap(), Triple(B));
        assert_eq!("inc a".parse::<Instruction>().unwrap(), Increment(A));
        assert_eq!("jmp +22".parse::<Instruction>().unwrap(), Jump(22));
        assert_eq!("jmp -7".parse::<Instruction>().unwrap(), Jump(-7));
        assert_eq!(
            "jie a, +4".parse::<Instruction>().unwrap(),
            JumpIfEven(A, 4)
        );
        assert_eq!(
            "jie a, -24".parse::<Instruction>().unwrap(),
            JumpIfEven(A, -24)
        );
        assert_eq!("jio b, +9".parse::<Instruction>().unwrap(), JumpIfOne(B, 9));
        assert_eq!(
            "jio b, -42".parse::<Instruction>().unwrap(),
            JumpIfOne(B, -42)
        );
    }

    #[test]
    fn instruction_apply_half() {
        let mut cpu = Cpu {
            index: 2,
            ..Cpu::default()
        };
        cpu.set_registry_value(&B, 10);

        cpu.run(&Half(B));

        assert_eq!(cpu.index, 3);
        assert_eq!(cpu.get_registry_value(&B), 5);
    }

    #[test]
    fn instruction_apply_triple() {
        let mut cpu = Cpu {
            index: 2,
            ..Cpu::default()
        };
        cpu.set_registry_value(&B, 10);

        cpu.run(&Triple(B));

        assert_eq!(cpu.index, 3);
        assert_eq!(cpu.get_registry_value(&B), 30);
    }

    #[test]
    fn instruction_apply_increment() {
        let mut cpu = Cpu {
            index: 2,
            ..Cpu::default()
        };
        cpu.set_registry_value(&B, 10);

        cpu.run(&Increment(B));

        assert_eq!(cpu.index, 3);
        assert_eq!(cpu.get_registry_value(&B), 11);
    }

    #[test]
    fn instruction_apply_jump() {
        let mut cpu = Cpu {
            index: 2,
            ..Cpu::default()
        };

        cpu.run(&Jump(4));

        assert_eq!(cpu.index, 6);
    }

    #[test]
    fn instruction_apply_jump_if_even() {
        let mut cpu = Cpu {
            index: 2,
            ..Cpu::default()
        };

        cpu.set_registry_value(&A, 10);
        cpu.set_registry_value(&B, 11);

        cpu.run(&JumpIfEven(A, 3));
        assert_eq!(cpu.index, 5);

        cpu.index = 2;

        cpu.run(&JumpIfEven(B, 3));
        assert_eq!(cpu.index, 3);
    }

    #[test]
    fn instruction_apply_jump_if_one() {
        let mut cpu = Cpu {
            index: 2,
            ..Cpu::default()
        };

        cpu.set_registry_value(&A, 1);
        cpu.set_registry_value(&B, 2);

        cpu.run(&JumpIfOne(A, 3));
        assert_eq!(cpu.index, 5);

        cpu.index = 2;

        cpu.run(&JumpIfOne(B, 3));
        assert_eq!(cpu.index, 3);
    }
}
