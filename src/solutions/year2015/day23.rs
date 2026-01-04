use crate::solutions::year2015::day23::Instruction::{
    Half, Increment, Jump, JumpIfEven, JumpIfOne, Triple,
};
use crate::solutions::Solution;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Day23;

impl Solution for Day23 {
    fn part_one(&self, input: &str) -> String {
        let registry = self.apply_instructions(input);

        registry.get(&b'b').to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day23 {
    fn apply_instructions(&self, input: &str) -> Registry {
        let instructions: Vec<Instruction> = input.lines().map(|l| l.parse().unwrap()).collect();
        let mut registry = Registry::default();
        let mut i = 0i32;

        while i < instructions.len() as i32 {
            i = instructions
                .get(i as usize)
                .unwrap()
                .apply(i, &mut registry);
        }

        registry
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Half(u8),
    Triple(u8),
    Increment(u8),
    Jump(i32),
    JumpIfEven(u8, i32),
    JumpIfOne(u8, i32),
}

impl Instruction {
    fn apply(&self, i: i32, registry: &mut Registry) -> i32 {
        match self {
            Half(r) => *registry.get_mut(r) /= 2,
            Triple(r) => *registry.get_mut(r) *= 3,
            Increment(r) => *registry.get_mut(r) += 1,
            _ => {}
        };

        match self {
            Jump(jump) => i + jump,
            JumpIfEven(r, jump) if registry.get(r) % 2 == 0 => i + jump,
            JumpIfOne(r, jump) if registry.get(r) == 1 => i + jump,
            _ => i + 1,
        }
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..3] {
            "hlf" => Ok(Half(s.as_bytes()[4])),
            "tpl" => Ok(Triple(s.as_bytes()[4])),
            "inc" => Ok(Increment(s.as_bytes()[4])),
            "jmp" => Ok(Jump(s[4..].parse().unwrap())),
            "jie" => Ok(JumpIfEven(s.as_bytes()[4], s[7..].parse().unwrap())),
            "jio" => Ok(JumpIfOne(s.as_bytes()[4], s[7..].parse().unwrap())),
            _ => Err("Invalid instruction".to_string()),
        }
    }
}

#[derive(Default)]
struct Registry {
    registry: HashMap<u8, i32>,
}

impl Registry {
    fn get_mut(&mut self, r: &u8) -> &mut i32 {
        self.registry.entry(*r).or_insert(0)
    }

    fn get(&self, r: &u8) -> i32 {
        *self.registry.get(r).unwrap_or(&0)
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
        let registry = Day23.apply_instructions(EXAMPLE);

        assert_eq!(registry.get(&b'a'), 2);
    }

    #[test]
    fn instruction_parse() {
        assert_eq!("hlf a".parse::<Instruction>().unwrap(), Half(b'a'));
        assert_eq!("tpl b".parse::<Instruction>().unwrap(), Triple(b'b'));
        assert_eq!("inc c".parse::<Instruction>().unwrap(), Increment(b'c'));
        assert_eq!("jmp +22".parse::<Instruction>().unwrap(), Jump(22));
        assert_eq!("jmp -7".parse::<Instruction>().unwrap(), Jump(-7));
        assert_eq!(
            "jie e, +4".parse::<Instruction>().unwrap(),
            JumpIfEven(b'e', 4)
        );
        assert_eq!(
            "jie f, -24".parse::<Instruction>().unwrap(),
            JumpIfEven(b'f', -24)
        );
        assert_eq!(
            "jio g, +9".parse::<Instruction>().unwrap(),
            JumpIfOne(b'g', 9)
        );
        assert_eq!(
            "jio h, -42".parse::<Instruction>().unwrap(),
            JumpIfOne(b'h', -42)
        );
    }

    #[test]
    fn instruction_apply_half() {
        let mut registry = Registry::default();
        *registry.get_mut(&b'b') = 10;

        let next = Half(b'b').apply(2, &mut registry);

        assert_eq!(next, 3);
        assert_eq!(5, registry.get(&b'b'));
    }

    #[test]
    fn instruction_apply_triple() {
        let mut registry = Registry::default();
        *registry.get_mut(&b'b') = 10;

        let next = Triple(b'b').apply(2, &mut registry);

        assert_eq!(next, 3);
        assert_eq!(30, registry.get(&b'b'));
    }

    #[test]
    fn instruction_apply_increment() {
        let mut registry = Registry::default();
        *registry.get_mut(&b'b') = 10;

        let next = Increment(b'b').apply(2, &mut registry);

        assert_eq!(next, 3);
        assert_eq!(11, registry.get(&b'b'));
    }

    #[test]
    fn instruction_apply_jump() {
        let mut registry = Registry::default();

        let next = Jump(4).apply(2, &mut registry);

        assert_eq!(next, 6);
    }

    #[test]
    fn instruction_apply_jump_if_even() {
        let mut registry = Registry::default();
        *registry.get_mut(&b'b') = 10;
        *registry.get_mut(&b'e') = 11;

        let next = JumpIfEven(b'b', 3).apply(2, &mut registry);
        assert_eq!(next, 5);

        let next = JumpIfEven(b'e', 3).apply(2, &mut registry);
        assert_eq!(next, 3);
    }

    #[test]
    fn instruction_apply_jump_if_one() {
        let mut registry = Registry::default();
        *registry.get_mut(&b'b') = 1;
        *registry.get_mut(&b'e') = 2;

        let next = JumpIfOne(b'b', 3).apply(2, &mut registry);
        assert_eq!(next, 5);

        let next = JumpIfOne(b'e', 3).apply(2, &mut registry);
        assert_eq!(next, 3);
    }
}
