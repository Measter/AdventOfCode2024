use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Display,
    ops::Add,
};

use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result, eyre::eyre};

pub const DAY: Day = Day {
    day: 18,
    name: "RAM Run",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1(&data[..1024], 71, 71)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part2(&data, 1024, 71, 71)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: u8,
    y: u8,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.wrapping_add(rhs.x),
            y: self.y.wrapping_add(rhs.y),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    pos: Point,
    steps: u16,
    predicted: u16,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.steps + other.predicted)
            .cmp(&(self.steps + self.predicted))
            .then(self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Result<Vec<Point>> {
    input
        .trim()
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').ok_or_else(|| eyre!("Invalid input"))?;
            Ok::<_, Report>(Point {
                x: x.parse()?,
                y: y.parse()?,
            })
        })
        .collect()
}

fn part1(data: &[Point], width: usize, height: usize) -> u16 {
    let mut map = vec![false; width * height];
    let mut dists = HashMap::new();
    let mut heap = BinaryHeap::new();

    for point in data {
        map[(point.y as usize) * width + point.x as usize] = true;
    }

    dists.insert(Point { x: 0, y: 0 }, 0);
    heap.push(State {
        pos: Point { x: 0, y: 0 },
        steps: 0,
        predicted: width as u16 * height as u16,
    });

    let end = Point {
        x: width as u8 - 1,
        y: height as u8 - 1,
    };

    while let Some(cur_pos) = heap.pop() {
        if cur_pos.pos == end {
            return cur_pos.steps;
        }

        if cur_pos.steps > dists.get(&cur_pos.pos).copied().unwrap_or(u16::MAX) {
            continue;
        }

        let next_steps = cur_pos.steps + 1;

        for (dx, dy) in [(0, 255), (0, 1), (255, 0), (1, 0)] {
            let next_pos = Point { x: dx, y: dy } + cur_pos.pos;

            if next_pos.x >= width as u8 || next_pos.y >= height as u8 {
                continue;
            }

            if map[(next_pos.y as usize) * width + next_pos.x as usize] {
                continue;
            }

            let next_pred = ((end.x - next_pos.x) + (end.y - next_pos.y)) as u16;
            let next_cost = next_steps + next_pred;

            if next_cost < dists.get(&next_pos).copied().unwrap_or(u16::MAX) {
                heap.push(State {
                    pos: next_pos,
                    steps: next_steps,
                    predicted: next_pred,
                });
                dists.insert(next_pos, next_steps);
            }
        }
    }

    unreachable!()
}

fn part2(data: &[Point], init: usize, width: usize, height: usize) -> Point {
    let mut map = vec![false; width * height];

    for point in &data[..init] {
        map[(point.y as usize) * width + point.x as usize] = true;
    }

    let mut dists = HashMap::new();
    let mut heap = BinaryHeap::new();

    let end = Point {
        x: width as u8 - 1,
        y: height as u8 - 1,
    };

    'outer: for next in &data[init..] {
        dists.clear();
        heap.clear();

        dists.insert(Point { x: 0, y: 0 }, 0);
        heap.push(State {
            pos: Point { x: 0, y: 0 },
            steps: 0,
            predicted: width as u16 * height as u16,
        });

        map[(next.y as usize) * width + next.x as usize] = true;

        while let Some(cur_pos) = heap.pop() {
            if cur_pos.pos == end {
                continue 'outer;
            }

            if cur_pos.steps > dists.get(&cur_pos.pos).copied().unwrap_or(u16::MAX) {
                continue;
            }

            let next_steps = cur_pos.steps + 1;

            for (dx, dy) in [(0, 255), (0, 1), (255, 0), (1, 0)] {
                let next_pos = Point { x: dx, y: dy } + cur_pos.pos;

                if next_pos.x >= width as u8 || next_pos.y >= height as u8 {
                    continue;
                }

                if map[(next_pos.y as usize) * width + next_pos.x as usize] {
                    continue;
                }

                let next_pred = ((end.x - next_pos.x) + (end.y - next_pos.y)) as u16;
                let next_cost = next_steps + next_pred;

                if next_cost < dists.get(&next_pos).copied().unwrap_or(u16::MAX) {
                    heap.push(State {
                        pos: next_pos,
                        steps: next_steps,
                        predicted: next_pred,
                    });
                    dists.insert(next_pos, next_steps);
                }
            }
        }

        // If we got here, that means the end was not reached
        return *next;
    }

    unreachable!()
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
        let expected = 22;
        let actual = part1(&parsed[..12], 7, 7);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = Point { x: 6, y: 1 };
        let actual = part2(&parsed, 12, 7, 7);

        assert_eq!(expected, actual);
    }
}
