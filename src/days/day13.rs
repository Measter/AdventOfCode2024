use std::{
    collections::{BinaryHeap, HashMap},
    ops::Add,
};

use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result, eyre::eyre};

pub const DAY: Day = Day {
    day: 13,
    name: "Claw Contraption",
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: u16,
    y: u16,
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Machine {
    prize: Point,
    btn_a: Point,
    btn_b: Point,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cost {
    a_count: u8,
    b_count: u8,
}

impl Cost {
    fn get_cost(self) -> u16 {
        self.a_count as u16 * 3 + self.b_count as u16 * 1
    }
}

impl Ord for Cost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_cost().cmp(&other.get_cost())
    }
}

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Cost {
    const MAX: Self = Cost {
        a_count: u8::MAX,
        b_count: u8::MAX,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    pos: Point,
    cost: Cost,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost).then(self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Result<Vec<Machine>> {
    let mut machines = Vec::new();

    for chunk in input.split("\n\n") {
        let mut lines = chunk.lines();

        let btn_a = lines.next().ok_or_else(|| eyre!("Missing btn_a"))?;
        let btn_b = lines.next().ok_or_else(|| eyre!("Missing btn_b"))?;
        let prize = lines.next().ok_or_else(|| eyre!("Missing prize"))?;

        let prize = prize
            .strip_prefix("Prize: ")
            .ok_or_else(|| eyre!("Invalid prize"))?;
        let btn_a = btn_a
            .strip_prefix("Button A: ")
            .ok_or_else(|| eyre!("Invalid btn_a"))?;
        let btn_b = btn_b
            .strip_prefix("Button B: ")
            .ok_or_else(|| eyre!("Invalid btn_b"))?;

        let prize = {
            let (x, y) = prize
                .split_once(',')
                .ok_or_else(|| eyre!("Invalid prize"))?;

            Point {
                x: x[2..].trim().parse()?,
                y: y[3..].trim().parse()?,
            }
        };

        let btn_a = {
            let (x, y) = btn_a
                .split_once(',')
                .ok_or_else(|| eyre!("Invalid btn_a"))?;

            Point {
                x: x[2..].trim().parse()?,
                y: y[3..].trim().parse()?,
            }
        };

        let btn_b = {
            let (x, y) = btn_b
                .split_once(',')
                .ok_or_else(|| eyre!("Invalid btn_b"))?;

            Point {
                x: x[2..].trim().parse()?,
                y: y[3..].trim().parse()?,
            }
        };

        machines.push(Machine {
            prize,
            btn_a,
            btn_b,
        });
    }

    Ok(machines)
}

fn part1(machines: &[Machine]) -> u32 {
    let mut sum = 0u32;
    let mut dists = HashMap::new();
    let mut heap = BinaryHeap::new();

    for machine in machines {
        dists.clear();
        heap.clear();

        dists.insert(Point { x: 0, y: 0 }, Cost {
            a_count: 0,
            b_count: 0,
        });

        heap.push(State {
            pos: Point { x: 0, y: 0 },
            cost: Cost {
                a_count: 0,
                b_count: 0,
            },
        });

        while let Some(cur_pos) = heap.pop() {
            if cur_pos.pos == machine.prize {
                sum += cur_pos.cost.get_cost() as u32;
                break;
            }

            if cur_pos.cost > dists.get(&cur_pos.pos).copied().unwrap_or(Cost::MAX) {
                continue;
            }

            let next_a_pos = cur_pos.pos + machine.btn_a;
            let next_a_cost = Cost {
                a_count: cur_pos.cost.a_count + 1,
                b_count: cur_pos.cost.b_count,
            };
            let next_b_pos = cur_pos.pos + machine.btn_b;
            let next_b_cost = Cost {
                a_count: cur_pos.cost.a_count,
                b_count: cur_pos.cost.b_count + 1,
            };

            if next_a_cost.a_count <= 100
                && next_a_cost < dists.get(&next_a_pos).copied().unwrap_or(Cost::MAX)
            {
                heap.push(State {
                    pos: next_a_pos,
                    cost: next_a_cost,
                });
                dists.insert(next_a_pos, next_a_cost);
            }

            if next_b_cost.b_count <= 100
                && next_b_cost < dists.get(&next_b_pos).copied().unwrap_or(Cost::MAX)
            {
                heap.push(State {
                    pos: next_b_pos,
                    cost: next_b_cost,
                });
                dists.insert(next_b_pos, next_b_cost);
            }
        }
    }

    sum
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
        let expected = 480;
        let actual = part1(&parsed);

        assert_eq!(expected, actual);
    }
}
