use crate::day17::Opcode::{Adv, Bdv, Bst, Bxc, Bxl, Cdv, Jnz, Out};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

// CodSpeed compatibility
#[allow(dead_code, clippy::useless_format)]
pub fn part1(input: &str) -> String {
    format!("{}",part1_solution(&parse(input)))
}
#[allow(dead_code, clippy::useless_format)]
pub fn part2(input: &str) -> String {
    format!("{}", part2_solution(&parse(input)))
}
// CodSpeed compatibility end

#[derive(Copy, Clone)]
enum StepResult {
    Out(u8),
    SetA(u64),
    SetB(u64),
    SetC(u64),
    Jump(u64),
    None,
}

#[derive(Clone, Copy, Default)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}
#[derive(Clone)]
struct Computer {
    registers: Registers,
    pc: usize,
    instructions: Vec<u8>,
}

impl Computer {
    fn step(&mut self) -> Option<u8> {
        let opcode: Opcode = Opcode::from(self.instructions[self.pc]);
        let operand: u8 = self.instructions[self.pc + 1];
        self.pc += 2;

        let argument = if opcode.needs_operand_resolve() {
            Operand::from(operand).resolve(&self.registers)
        } else {
            operand as u64
        };

        let result = opcode.execute(argument, &self.registers);

        match result {
            StepResult::Out(out) => return Some(out),
            StepResult::SetA(a) => {
                self.registers.a = a;
            }
            StepResult::SetB(b) => {
                self.registers.b = b;
            }
            StepResult::SetC(c) => {
                self.registers.c = c;
            }
            StepResult::Jump(pc) => {
                self.pc = pc as usize;
            }
            StepResult::None => {}
        }

        None
    }

    fn is_halted(&self) -> bool {
        self.pc >= self.instructions.len()
    }
}

#[derive(Clone, Copy)]
#[repr(u8)]
enum Opcode {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}
impl Opcode {
    const fn from(opcode: u8) -> Self {
        match opcode {
            0 => Adv,
            1 => Bxl,
            2 => Bst,
            3 => Jnz,
            4 => Bxc,
            5 => Out,
            6 => Bdv,
            7 => Cdv,
            _ => panic!("Unknown opcode"),
        }
    }
    #[inline]
    fn needs_operand_resolve(&self) -> bool {
        match self {
            Adv => true,
            Bxl => false,
            Bst => true,
            Jnz => false,
            Bxc => false,
            Out => true,
            Bdv => true,
            Cdv => true,
        }
    }
    #[inline]
    fn execute(&self, argument: u64, registers: &Registers) -> StepResult {
        match self {
            Adv => StepResult::SetA(registers.a / 2_u64.pow(argument as u32)),
            Bxl => StepResult::SetB(registers.b ^ argument),
            Bst => StepResult::SetB(argument % 8),
            Jnz => {
                if registers.a != 0 {
                    StepResult::Jump(argument)
                } else {
                    StepResult::None
                }
            }
            Bxc => StepResult::SetB(registers.b ^ registers.c),
            Out => StepResult::Out((argument % 8) as u8),
            Bdv => StepResult::SetB(registers.a / 2_u64.pow(argument as u32)),
            Cdv => StepResult::SetC(registers.a / 2_u64.pow(argument as u32)),
        }
    }
}

#[derive(Clone, Copy)]
#[repr(u8)]
enum Operand {
    Lit0 = 0,
    Lit1 = 1,
    Lit2 = 2,
    Lit3 = 3,
    A = 4,
    B = 5,
    C = 6,
}

impl Operand {
    const fn from(operand: u8) -> Self {
        match operand {
            0 => Operand::Lit0,
            1 => Operand::Lit1,
            2 => Operand::Lit2,
            3 => Operand::Lit3,
            4 => Operand::A,
            5 => Operand::B,
            6 => Operand::C,
            _ => panic!("Unknown operand"),
        }
    }
    const fn resolve(&self, registers: &Registers) -> u64 {
        match self {
            Operand::Lit0 => 0,
            Operand::Lit1 => 1,
            Operand::Lit2 => 2,
            Operand::Lit3 => 3,
            Operand::A => registers.a,
            Operand::B => registers.b,
            Operand::C => registers.c,
        }
    }
}

#[aoc_generator(day17)]
fn parse(input: &str) -> Computer {
    let mut lines = input.lines();
    let line_a = lines.next().expect("Bad input");
    let _ = lines.next();
    let _ = lines.next();
    let _ = lines.next();
    let program = lines.next().expect("Bad input");

    Computer {
        registers: Registers {
            a: line_a[12..].parse::<u64>().expect("Bad input"),
            ..Default::default()
        },

        pc: 0,
        instructions: program[9..]
            .split(",")
            .map(|n| n.parse::<u8>().expect("Bad input"))
            .collect(),
    }
}

fn compute<'a>(a: u64, computer: &Computer, output: &'a mut Vec<u8>) -> &'a mut Vec<u8> {
    output.clear();
    let mut computer = computer.clone();
    computer.registers.a = a;

    while !computer.is_halted() {
        if let Some(result) = computer.step() {
            output.push(result);
        }
    }
    output
}
#[aoc(day17, part1)]
fn part1_solution(input: &Computer) -> String {
    let mut output: Vec<u8> = Vec::with_capacity(input.instructions.len());
    compute(input.registers.a, input, &mut output)
        .iter()
        .join(",")
}

#[aoc(day17, part2)]
fn part2_solution(input: &Computer) -> u64 {
    let target = input.instructions.clone();
    let mut output: Vec<u8> = Vec::with_capacity(input.instructions.len());

    let loop_size = {
        let mut counter = 0u64;
        while compute(counter, input, &mut output).len() < 2 {
            counter += 1;
        }
        counter
    };
    let target_len = target.len();

    let mut a = loop_size.pow((target_len - 1) as u32);

    while !target.eq(compute(a, input, &mut output)) {
        a += 1;
        let mut loops: i32 = (target_len - 1) as i32;
        a %= loop_size.pow(target_len as u32);
        while loops >= 0 {
            compute(a, input, &mut output);

            if output[loops as usize] == target[loops as usize] {
                loops -= 1;
            } else {
                a += loop_size.pow(loops as u32);
                if loops < (target_len - 1) as i32 {
                    loops += 1;
                }
            }
        }
    }

    while target.eq(compute(a - 1, input, &mut output)) {
        a -= 1;
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;

    const SELF_RETURNING_PROGRAM: &str = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1_solution(&parse(EXAMPLE)), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_solution(&parse(SELF_RETURNING_PROGRAM)), 117440);
    }
}
