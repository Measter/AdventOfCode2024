use std::cmp::Ordering;

use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result, eyre::OptionExt};

pub const DAY: Day = Day {
    day: 5,
    name: "Print Queue",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let (db, orders) = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part::<true>(&db, &orders)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let (db, orders) = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part::<false>(&db, &orders)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug)]
struct ComesBeforeDB {
    db: [u128; 100],
}

impl ComesBeforeDB {
    fn set_comes_before(&mut self, a: u8, b: u8) {
        self.db[a as usize] |= u128::pow(2, b as u32);
    }

    fn comes_before(&self, a: u8, b: u8) -> bool {
        (self.db[a as usize] & u128::pow(2, b as u32)) != 0
    }
}

fn parse(input: &str) -> Result<(ComesBeforeDB, Vec<Vec<u8>>)> {
    let (cbs, order_list) = input.split_once("\n\n").ok_or_eyre("Invalid input")?;

    let mut db = ComesBeforeDB { db: [0; 100] };
    for cb in cbs.trim().lines() {
        let (l, r) = cb.split_once('|').ok_or_eyre("Invalid input")?;
        db.set_comes_before(l.parse()?, r.parse()?);
    }

    let mut orders = Vec::new();
    for order in order_list.trim().lines() {
        let list = order.split(',').map(str::parse::<u8>).try_fold(
            Vec::new(),
            |mut v, o| -> Result<Vec<u8>> {
                v.push(o?);
                Ok(v)
            },
        )?;

        orders.push(list);
    }

    Ok((db, orders))
}

fn part<const P1: bool>(db: &ComesBeforeDB, orders: &[Vec<u8>]) -> u32 {
    let mut sum = 0;

    let mut buf = Vec::new();
    for order in orders {
        let mut order_slice = order.as_slice();
        let mut is_ordered = true;
        while let [first, rest @ ..] = order_slice {
            is_ordered &= rest.iter().all(|r| db.comes_before(*first, *r));
            order_slice = rest;
        }

        if P1 && is_ordered {
            sum += order[order.len() / 2] as u32;
        } else if !P1 && !is_ordered {
            buf.clear();
            buf.extend_from_slice(order);
            buf.sort_unstable_by(|&a, &b| {
                if db.comes_before(a, b) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });

            sum += buf[buf.len() / 2] as u32;
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

        let (db, orders) = parse(&data).unwrap();
        let expected = 143;
        let actual = part::<true>(&db, &orders);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let (db, orders) = parse(&data).unwrap();
        let expected = 123;
        let actual = part::<false>(&db, &orders);

        assert_eq!(expected, actual);
    }
}
