use std::ops::{Add, Sub};

use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};

pub const DAY: Day = Day {
    day: 10,
    name: "Hool It",
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

impl Point {
    fn neighbours(self) -> [Self; 4] {
        [
            self - Point { x: 0, y: 1 },
            self + Point { x: 0, y: 1 },
            self - Point { x: 1, y: 0 },
            self + Point { x: 1, y: 0 },
        ]
    }
}

struct Map {
    tiles: Vec<u8>,
    width: usize,
    height: usize,
    starts: Vec<Point>,
}

impl Map {
    fn contains(&self, p: Point) -> bool {
        (p.x as usize) < self.width && (p.y as usize) < self.height
    }

    fn index(&self, p: Point) -> usize {
        p.y as usize * self.width + p.x as usize
    }

    fn tile(&self, p: Point) -> Option<u8> {
        self.tiles.get(self.index(p)).copied()
    }
}

fn parse(input: &str) -> Result<Map> {
    let mut tiles = Vec::new();
    let mut starts = Vec::new();
    let mut height = 0;

    for (y, row) in input.trim().lines().enumerate() {
        for (x, mut tile) in row.trim().bytes().enumerate() {
            tile -= b'0';
            tiles.push(tile);

            if tile == 0 {
                starts.push(Point {
                    x: x as u8,
                    y: y as u8,
                })
            }
        }

        height += 1;
    }

    Ok(Map {
        width: tiles.len() / height,
        height,
        tiles,
        starts,
    })
}

fn part1(map: &Map) -> usize {
    let mut queue = Vec::new();
    let mut visited = vec![false; map.tiles.len()];
    let mut total = 0;

    for start in &map.starts {
        queue.clear();
        queue.push(*start);
        visited.fill(false);
        let mut reachable = 0;

        while let Some(next) = queue.pop() {
            let Some(next_tile) = map.tile(next) else {
                continue;
            };

            if next_tile == 9 && !visited[map.index(next)] {
                reachable += 1;
            } else {
                for nb in next.neighbours() {
                    if !map.contains(nb) {
                        continue;
                    }
                    let Some(nb_tile) = map.tile(nb) else {
                        continue;
                    };
                    if nb_tile.wrapping_sub(next_tile) == 1 && !visited[map.index(nb)] {
                        queue.push(nb);
                    }
                }
            }

            visited[map.index(next)] = true;
        }

        total += reachable
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

        for (id, case) in data.split("---").enumerate() {
            let Some((test, result)) = case.split_once("\n\n") else {
                panic!("bad test input");
            };

            let parsed = parse(test).unwrap();
            let expected: usize = result.trim().parse().unwrap();
            let actual = part1(&parsed);

            assert_eq!(expected, actual, "{id}");
        }
    }
}
