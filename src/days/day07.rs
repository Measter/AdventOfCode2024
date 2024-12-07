use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{
    Report, Result,
    eyre::{OptionExt, eyre},
};
use smallvec::SmallVec;

pub const DAY: Day = Day {
    day: 7,
    name: "Bridge Repair",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part::<true>(&data)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part::<false>(&data)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Mul,
    Concat,
}

impl Operator {
    fn apply(self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Mul => lhs * rhs,
            Operator::Concat => match rhs {
                ..10 => lhs * 10 + rhs,
                10..100 => lhs * 100 + rhs,
                _ => lhs * 1000 + rhs,
            },
        }
    }
}

#[derive(Debug)]
struct Equation {
    expected: u64,
    operands: SmallVec<[u64; 16]>,
}

struct PermGen<'item, 'buf, T> {
    done: bool,
    items: &'item [T],
    indices: &'buf mut Vec<usize>,
    output: &'buf mut Vec<&'item T>,
}

impl<'item, 'buf, T> PermGen<'item, 'buf, T> {
    pub fn new(
        n: usize,
        output: &'buf mut Vec<&'item T>,
        indices: &'buf mut Vec<usize>,
        items: &'item [T],
    ) -> Self {
        assert!(n > 0);
        assert!(!items.is_empty());

        indices.clear();
        indices.resize(n, 0);

        Self {
            items,
            done: false,
            indices,
            output,
        }
    }

    pub fn next<'b>(&'b mut self) -> Option<&'b [&'item T]> {
        if self.done {
            return None;
        }

        self.output.clear();
        for &i in &*self.indices {
            self.output.push(&self.items[i]);
        }

        let mut carry = true;
        let mut i = 0;
        while carry && i < self.indices.len() {
            self.indices[i] += 1;
            if self.indices[i] == self.items.len() {
                carry = true;
                self.indices[i] = 0;
                i += 1;
            } else {
                carry = false;
            }
        }

        if carry {
            self.done = true;
        }

        Some(self.output)
    }
}

fn parse(input: &str) -> Result<Vec<Equation>> {
    let mut equations = Vec::new();

    for line in input.trim().lines() {
        let (expected, operands) = line.trim().split_once(':').ok_or_eyre("invalid input")?;
        equations.push(Equation {
            expected: expected.trim().parse()?,
            operands: operands
                .split_ascii_whitespace()
                .map(str::parse)
                .collect::<Result<_, _>>()?,
        });
    }

    Ok(equations)
}

fn part<const P1: bool>(data: &[Equation]) -> u64 {
    let mut total = 0;
    let mut obuf = Vec::new();
    let mut ibuf = Vec::new();

    let ops: &[Operator] = if P1 {
        &[Operator::Add, Operator::Mul]
    } else {
        &[Operator::Add, Operator::Mul, Operator::Concat]
    };

    for eq in data {
        let mut ops = PermGen::new(eq.operands.len() - 1, &mut obuf, &mut ibuf, ops);

        while let Some(op_perm) = ops.next() {
            let [first, rest @ ..] = eq.operands.as_slice() else {
                unreachable!()
            };

            let mut sum = *first;

            for (&rhs, &&op) in rest.iter().zip(op_perm) {
                sum = op.apply(sum, rhs);
            }

            if sum == eq.expected {
                total += sum;
                break;
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn part1_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = 3749;
        let actual = part::<true>(&parsed);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = 11387;
        let actual = part::<false>(&parsed);

        assert_eq!(expected, actual);
    }
}
