use std::{
    collections::HashSet,
    ops::{Add, Mul, Sub},
};

use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};

pub const DAY: Day = Day {
    day: 8,
    name: "Resonant Collinearity",
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: u8,
    y: u8,
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.wrapping_sub(rhs.x),
            y: self.y.wrapping_sub(rhs.y),
        }
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

impl Mul<u8> for Point {
    type Output = Self;
    fn mul(self, rhs: u8) -> Self::Output {
        Self {
            x: self.x.wrapping_mul(rhs),
            y: self.y.wrapping_mul(rhs),
        }
    }
}

impl Point {
    fn antinodes(self, rhs: Self) -> impl Iterator<Item = [Self; 2]> {
        let delta = rhs - self;

        // [self - delta, rhs + delta]
        (0..).map(move |i| {
            let d = delta * i;
            [self - d, rhs + d]
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Antenna {
    frequency: u8,
    pos: Point,
}

#[derive(Debug)]
struct Map {
    antennae: Vec<Antenna>,
    width: usize,
    height: usize,
}

impl Map {
    fn in_bounds(&self, p: Point) -> bool {
        ((p.x as usize) < self.width) & ((p.y as usize) < self.height)
    }
}

fn parse(input: &str) -> Result<Map> {
    let mut antennae = Vec::new();
    let mut width = 0;
    let mut height = 0;

    for (y, row) in input.trim().lines().enumerate() {
        for (x, tile) in row.trim().bytes().enumerate() {
            if matches!(tile, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9') {
                antennae.push(Antenna {
                    frequency: tile,
                    pos: Point {
                        x: x as u8,
                        y: y as u8,
                    },
                });
            }

            width = width.max(x);
        }

        height += 1;
    }

    Ok(Map {
        antennae,
        width: width + 1,
        height,
    })
}

fn part<const P1: bool>(map: &Map) -> usize {
    let mut antinodes: HashSet<Point> = HashSet::new();

    for (i, a) in map.antennae.iter().enumerate() {
        for b in &map.antennae[i + 1..] {
            if a.frequency != b.frequency {
                continue;
            }

            let mut ans = a.pos.antinodes(b.pos);
            if P1 {
                let [an_a, an_b] = ans.nth(1).unwrap();

                if map.in_bounds(an_a) {
                    antinodes.insert(an_a);
                }

                if map.in_bounds(an_b) {
                    antinodes.insert(an_b);
                }
            } else {
                for [an_a, an_b] in ans {
                    if !map.in_bounds(an_a) && !map.in_bounds(an_b) {
                        break;
                    }

                    if map.in_bounds(an_a) {
                        antinodes.insert(an_a);
                    }

                    if map.in_bounds(an_b) {
                        antinodes.insert(an_b);
                    }
                }
            }
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn antinode_test() {
        let a = Point { x: 4, y: 3 };
        let b = Point { x: 5, y: 5 };

        let expected = [Point { x: 3, y: 1 }, Point { x: 6, y: 7 }];
        let actual = a.antinodes(b).nth(1).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn part1_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = 14;
        let actual = part::<true>(&parsed);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test1() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part2, 1)
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = 9;
        let actual = part::<false>(&parsed);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test2() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = 34;
        let actual = part::<false>(&parsed);

        assert_eq!(expected, actual);
    }
}
