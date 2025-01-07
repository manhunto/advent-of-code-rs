use crate::solutions::year2024::day17::InstructionType::{Adv, Bdv, Bst, Bxc, Bxl, Cdv, Jnz, Out};
use crate::solutions::Solution;
use itertools::Itertools;
use std::str::FromStr;

pub struct Day17;

impl Solution for Day17 {
    fn part_one(&self, input: &str) -> String {
        let (registers, program) = input.split_terminator("\n\n").collect_tuple().unwrap();
        let lines: Vec<&str> = registers.lines().collect();

        let mut register = Register {
            a: self.parse_register(&lines, 0),
            b: self.parse_register(&lines, 1),
            c: self.parse_register(&lines, 2),
        };

        let program = self.parse_program(program);

        self.execute(&mut register, program).iter().join(",")
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day17 {
    fn parse_register(&self, lines: &[&str], index: usize) -> usize {
        lines
            .get(index)
            .map(|s| {
                let (.., value) = s.split_once(": ").unwrap();

                value.parse().unwrap()
            })
            .unwrap()
    }

    fn parse_program(&self, program_line: &str) -> Program {
        program_line
            .trim_start_matches("Program: ")
            .trim_end()
            .parse()
            .unwrap()
    }

    fn execute(&self, register: &mut Register, program: Program) -> Vec<usize> {
        let mut instruction_pointer = 0;
        let mut output = Vec::new();

        while let Some(opcode) = program.program.get(instruction_pointer) {
            let operation = InstructionType::from(*opcode);
            let operand = program.program.get(instruction_pointer + 1).unwrap();
            let mut do_jump = true;

            match operation {
                Adv => {
                    let combo_operand = self.combo_operand(operand, register);
                    register.a /= 2usize.pow(combo_operand as u32);
                }
                Bdv => {
                    let combo_operand = self.combo_operand(operand, register);
                    register.b = register.a / 2usize.pow(combo_operand as u32);
                }
                Cdv => {
                    let combo_operand = self.combo_operand(operand, register);
                    register.c = register.a / 2usize.pow(combo_operand as u32);
                }
                Bxl => {
                    let operand_usize = *operand as usize;
                    register.b ^= operand_usize;
                }
                Bst => {
                    let combo_operand = self.combo_operand(operand, register);
                    register.b = combo_operand % 8;
                }
                Bxc => {
                    register.b ^= register.c;
                }
                Jnz => {
                    let operand_usize = *operand as usize;
                    if register.a != 0 && instruction_pointer != operand_usize {
                        instruction_pointer = operand_usize;
                        do_jump = false;
                    }
                }
                Out => {
                    let combo_operand = self.combo_operand(operand, register);
                    output.push(combo_operand % 8);
                }
            }

            if do_jump {
                instruction_pointer += 2;
            }
        }

        output
    }

    fn combo_operand(&self, operand: &u8, register: &Register) -> usize {
        match operand {
            0..=3 => *operand as usize,
            4 => register.a,
            5 => register.b,
            6 => register.c,
            7 => unreachable!("should not happened"),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Register {
    a: usize,
    b: usize,
    c: usize,
}

struct Program {
    program: Vec<u8>,
}

impl FromStr for Program {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            program: s
                .split_terminator(',')
                .map(|i| i.parse().unwrap())
                .collect(),
        })
    }
}

enum InstructionType {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u8> for InstructionType {
    fn from(value: u8) -> Self {
        match value {
            0 => Adv,
            1 => Bxl,
            2 => Bst,
            3 => Jnz,
            4 => Bxc,
            5 => Out,
            6 => Bdv,
            7 => Cdv,
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2024::day17::{Day17, Program, Register};
    use crate::solutions::Solution;

    const EXAMPLE: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;

    #[test]
    fn part_one_example() {
        assert_eq!("4,6,3,5,6,3,5,2,1,0", Day17.part_one(EXAMPLE));
    }

    #[test]
    fn execute_1() {
        let mut register = RegisterBuilder::default().c(9).build();
        let program: Program = "2,6".parse().unwrap();

        let result = Day17.execute(&mut register, program);

        assert_eq!(register, RegisterBuilder::default().c(9).b(1).build());
        assert!(result.is_empty());
    }

    #[test]
    fn execute_2() {
        let mut register = RegisterBuilder::default().a(10).build();
        let program: Program = "5,0,5,1,5,4".parse().unwrap();

        let result = Day17.execute(&mut register, program);

        assert_eq!(result, vec![0, 1, 2]);
    }

    #[test]
    fn execute_3() {
        let mut register = RegisterBuilder::default().a(2024).build();
        let program: Program = "0,1,5,4,3,0".parse().unwrap();

        let result = Day17.execute(&mut register, program);

        assert_eq!(result, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(register.a, 0);
    }

    #[test]
    fn execute_4() {
        let mut register = RegisterBuilder::default().b(29).build();
        let program: Program = "1,7".parse().unwrap();

        let result = Day17.execute(&mut register, program);

        assert_eq!(register.b, 26);
        assert!(result.is_empty())
    }

    #[test]
    fn execute_5() {
        let mut register = RegisterBuilder::default().b(2024).c(43690).build();
        let program: Program = "4,0".parse().unwrap();

        let result = Day17.execute(&mut register, program);

        assert_eq!(register.b, 44354);
        assert!(result.is_empty())
    }

    #[test]
    fn bdv_test() {
        let mut register = RegisterBuilder::default().a(2024).build();
        let program: Program = "6,2".parse().unwrap();

        let result = Day17.execute(&mut register, program);

        assert_eq!(register.a, 2024);
        assert_eq!(register.b, 506);
        assert!(result.is_empty())
    }

    #[test]
    fn cdv_test() {
        let mut register = RegisterBuilder::default().a(28).build();
        let program: Program = "7,3".parse().unwrap();

        let result = Day17.execute(&mut register, program);

        assert_eq!(register.a, 28);
        assert_eq!(register.c, 3);
        assert!(result.is_empty())
    }

    #[derive(Default)]
    struct RegisterBuilder {
        a: usize,
        b: usize,
        c: usize,
    }

    impl RegisterBuilder {
        fn a(&mut self, a: usize) -> &mut Self {
            self.a = a;
            self
        }

        fn b(&mut self, b: usize) -> &mut Self {
            self.b = b;
            self
        }

        fn c(&mut self, c: usize) -> &mut Self {
            self.c = c;
            self
        }

        fn build(&self) -> Register {
            Register {
                a: self.a,
                b: self.b,
                c: self.c,
            }
        }
    }
}
