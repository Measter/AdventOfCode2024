use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result, eyre::eyre};
use itertools::Itertools;

pub const DAY: Day = Day {
    day: 1,
    name: "Historian Histeria",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1(&data)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part2(&data)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

fn parse(input: &str) -> Result<(Vec<u32>, Vec<u32>)> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for s in input.trim().lines() {
        let Some((l, r)) = s.split_once(' ') else {
            return Err(eyre!("invalid input: `{s}`"));
        };

        left.push(l.trim().parse()?);
        right.push(r.trim().parse()?);
    }

    Ok((left, right))
}

fn part1((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut left = left.clone();
    left.sort_unstable();
    let mut right = right.clone();
    right.sort_unstable();

    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

fn part2((left, right): &(Vec<u32>, Vec<u32>)) -> usize {
    let mut left = left.clone();
    left.sort_unstable();
    let mut right = right.clone();
    right.sort_unstable();

    let mut right = right.as_slice();
    let mut sum = 0;

    for (l, ch) in &left.iter().chunk_by(|c| **c) {
        let Some(&r) = right.first() else {
            break;
        };

        if r < l {
            let Some(idx) = right.iter().position(|i| *i >= l) else {
                break;
            };

            right = &right[idx..];
        }

        if let Some(&r) = right.first()
            && r != l
        {
            continue;
        }

        if let Some(idx) = right.iter().position(|i| *i != l) {
            right = &right[idx..];
            sum += idx * (l as usize) * ch.count();
        } else {
            sum += (l as usize) * right.len() * ch.count();
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
        let expected = 11;
        let actual = part1(&parsed);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = 31;
        let actual = part2(&parsed);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test2() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part2, 1)
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = 31;
        let actual = part2(&parsed);

        assert_eq!(expected, actual);
    }
}
