use crate::problem::AoCProblem;

const CHECK_AT_CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];
const DISPLAY_WIDTH: usize = 40;
const DISPLAY_HEIGHT: usize = 6;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i64),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split(' ').collect();
        match parts[0] {
            "noop" => Instruction::Noop,
            "addx" => Instruction::Addx(parts[1].parse().unwrap()).to_owned(),
            _ => panic!("invalid input")
        }
    }
}

#[derive(Debug, Default)]
pub struct AoCDay10 {
    instructions: Vec<Instruction>
}

#[derive(Debug, Clone, Copy)]
struct CpuRegisters {
    x: i64
}

#[derive(Debug)]
struct Cpu {
    registers: CpuRegisters,
    history: Vec<CpuRegisters>,
}

impl Cpu {
    fn new() -> Self {
        Self {
            registers: CpuRegisters {
                x: 1
            },
            history: Vec::new()
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop => {
                self.history.push(self.registers);
            },
            Instruction::Addx(val) => {
                self.history.push(self.registers);
                self.history.push(self.registers);
                self.registers.x += val;
            }
        }
    }
}

impl AoCProblem for AoCDay10 {
    fn parse_line(&mut self, line: String) {
        self.instructions.push(Instruction::from(line.as_str()));
    }

    fn solve_part1(&self) -> String {
        let mut cpu = Cpu::new();
        for i in &self.instructions {
            cpu.execute(i);
        }
        let mut result = 0;
        for c in CHECK_AT_CYCLES {
            result += cpu.history[c - 1].x * (c as i64);
        }
        result.to_string()
    }

    fn solve_part2(&self) -> String {
        let mut cpu = Cpu::new();
        for i in &self.instructions {
            cpu.execute(i);
        }
        let mut result = String::from("\n");
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let current_x = cpu.history[y * DISPLAY_WIDTH + x].x;
                if current_x - 1 <= x as i64 && x as i64 <= current_x + 1 {
                    result.push('#');
                } else {
                    result.push(' ');
                }
            }
            result.push('\n');
        }
        result
    }
}
