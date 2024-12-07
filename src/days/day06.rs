use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};

pub const DAY: Day = Day {
    day: 6,
    name: "Guard Gallivant",
    part_1: run_part1,
    part_2: None,
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let (map, point) = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1(&map, point)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: u8,
    y: u8,
}

impl Point {
    fn step(self, dir: Direction) -> Self {
        match dir {
            Direction::Up => Self {
                x: self.x,
                y: self.y.wrapping_sub(1),
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y.wrapping_add(1),
            },
            Direction::Left => Self {
                x: self.x.wrapping_sub(1),
                y: self.y,
            },
            Direction::Right => Self {
                x: self.x.wrapping_add(1),
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
}

struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Map {
    fn contains(&self, p: Point) -> bool {
        (p.x as usize) < self.width && (p.y as usize) < self.height
    }

    fn index(&self, p: Point) -> usize {
        p.y as usize * self.width + p.x as usize
    }

    fn tile(&self, p: Point) -> Option<Tile> {
        self.tiles.get(self.index(p)).copied()
    }
}

fn parse(input: &str) -> Result<(Map, Point)> {
    let mut tiles = Vec::new();
    let mut start = Point { x: 0, y: 0 };
    let mut height = 0;

    for (y, row) in input.trim().lines().enumerate() {
        for (x, tile) in row.trim().bytes().enumerate() {
            match tile {
                b'#' => tiles.push(Tile::Wall),
                _ => tiles.push(Tile::Empty),
            }

            if tile == b'^' {
                start.x = x as u8;
                start.y = y as u8;
            }
        }

        height += 1;
    }

    Ok((
        Map {
            width: tiles.len() / height,
            height,
            tiles,
        },
        start,
    ))
}

fn part1(map: &Map, start: Point) -> usize {
    let mut visited = vec![false; map.tiles.len()];
    let mut cur_pos = start;
    let mut cur_dir = Direction::Up;

    while map.contains(cur_pos) {
        visited[map.index(cur_pos)] = true;
        let mut next = cur_pos.step(cur_dir);
        while let Some(Tile::Wall) = map.tile(next) {
            cur_dir = cur_dir.turn();
            next = cur_pos.step(cur_dir);
        }
        cur_pos = next;
    }

    visited.into_iter().filter(|l| *l).count()
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

        let (map, point) = parse(&data).unwrap();
        dbg!(&map.height, map.width);
        let expected = 41;
        let actual = part1(&map, point);

        assert_eq!(expected, actual);
    }
}
