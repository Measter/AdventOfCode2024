use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result, eyre::eyre};

pub const DAY: Day = Day {
    day: 17,
    name: "Chronospatial Computer",
    part_1: run_part1,
    part_2: None,
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1(&data)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl OpCode {
    fn from_int(i: u8) -> Option<Self> {
        match i {
            0 => Some(Self::Adv),
            1 => Some(Self::Bxl),
            2 => Some(Self::Bst),
            3 => Some(Self::Jnz),
            4 => Some(Self::Bxc),
            5 => Some(Self::Out),
            6 => Some(Self::Bdv),
            7 => Some(Self::Cdv),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Combo {
    Reg(u8),
    Imm(u8),
}

impl Combo {
    fn from_int(i: u8) -> Option<Self> {
        match i {
            0..=3 => Some(Self::Imm(i)),
            4..=6 => Some(Self::Reg(i - 4)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct Machine {
    ip: u8,
    reg: [u32; 3],
}

impl Machine {
    fn read_op(&mut self, pgm: &[u8]) -> Option<OpCode> {
        let next = pgm
            .get(self.ip as usize)
            .and_then(|&opcode| OpCode::from_int(opcode));

        self.ip += 1;
        next
    }

    fn read_literal(&mut self, pgm: &[u8]) -> Option<u8> {
        let next = pgm.get(self.ip as usize).copied();

        self.ip += 1;
        next
    }

    fn read_combi(&mut self, pgm: &[u8]) -> Option<Combo> {
        let next = pgm
            .get(self.ip as usize)
            .and_then(|&val| Combo::from_int(val));

        self.ip += 1;
        next
    }
}

fn parse(input: &str) -> Result<(Vec<u8>, Machine)> {
    let mut machine = Machine { ip: 0, reg: [0; 3] };

    let (regs, pgm) = input
        .split_once("\n\n")
        .ok_or_else(|| eyre!("Invalid input"))?;

    for (i, line) in regs.trim().lines().enumerate() {
        let Some((_, val)) = line.split_once(':') else {
            return Err(eyre!("Invalid register line: {}", line));
        };

        machine.reg[i] = val.trim().parse()?;
    }

    let ops = pgm
        .split_once(':')
        .ok_or_else(|| eyre!("Invalid program"))?
        .1
        .trim()
        .split(',')
        .map(str::parse::<u8>)
        .collect::<Result<_, _>>()?;

    Ok((ops, machine))
}

fn part1((pgm, machine): &(Vec<u8>, Machine)) -> String {
    let mut machine = machine.clone();
    let mut output = String::new();

    while let Some(opcode) = machine.read_op(pgm) {
        match opcode {
            OpCode::Adv | OpCode::Bdv | OpCode::Cdv => {
                let Some(operand) = machine.read_combi(pgm) else {
                    break;
                };

                let divisor = match operand {
                    Combo::Reg(i) => machine.reg[i as usize],
                    Combo::Imm(i) => i as u32,
                };

                let reg = match opcode {
                    OpCode::Adv => 0,
                    OpCode::Bdv => 1,
                    OpCode::Cdv => 2,
                    _ => unreachable!(),
                };
                machine.reg[reg] = machine.reg[0] / u32::pow(2, divisor);
            }
            OpCode::Bxl => {
                let Some(operand) = machine.read_literal(pgm) else {
                    break;
                };

                machine.reg[1] ^= operand as u32;
            }
            OpCode::Bst => {
                let Some(operand) = machine.read_combi(pgm) else {
                    break;
                };

                let op_value = match operand {
                    Combo::Reg(i) => machine.reg[i as usize],
                    Combo::Imm(i) => i as u32,
                };

                machine.reg[1] = op_value & 0x7;
            }
            OpCode::Jnz => {
                let Some(operand) = machine.read_literal(pgm) else {
                    break;
                };

                if machine.reg[0] != 0 {
                    machine.ip = operand;
                }
            }
            OpCode::Bxc => {
                let _ = machine.read_literal(pgm);
                machine.reg[1] ^= machine.reg[2];
            }
            OpCode::Out => {
                let Some(operand) = machine.read_combi(pgm) else {
                    break;
                };

                let op_value = match operand {
                    Combo::Reg(i) => machine.reg[i as usize],
                    Combo::Imm(i) => i as u32,
                };

                output.push(((op_value & 0x7) as u8 + b'0') as char);
                output.push(',');
            }
        }
    }

    output.pop(); // remove trailing comma
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn combo_test() {
        assert_eq!(Combo::from_int(0), Some(Combo::Imm(0)));
        assert_eq!(Combo::from_int(1), Some(Combo::Imm(1)));
        assert_eq!(Combo::from_int(2), Some(Combo::Imm(2)));
        assert_eq!(Combo::from_int(3), Some(Combo::Imm(3)));
        assert_eq!(Combo::from_int(4), Some(Combo::Reg(0)));
        assert_eq!(Combo::from_int(5), Some(Combo::Reg(1)));
        assert_eq!(Combo::from_int(6), Some(Combo::Reg(2)));
        assert!(Combo::from_int(7).is_none());
    }

    #[test]
    fn part1_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = "4,6,3,5,6,3,5,2,1,0";
        let actual = part1(&parsed);

        assert_eq!(expected, actual);
    }
}
