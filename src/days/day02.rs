use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report as EyreReport, Result};

pub const DAY: Day = Day {
    day: 2,
    name: "Red-Nosed Reports",
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
        Ok::<_, EyreReport>(ParseResult(data))
    })
}

#[derive(Debug, Clone)]
struct Report {
    levels: Vec<u8>,
}

impl Report {
    fn is_valid(&self) -> bool {
        let mut levels_iter = self.levels.array_windows();
        let Some([first, second]) = levels_iter.next() else {
            unreachable!()
        };

        let expected_order = first.cmp(second);
        let valid_range = 1..=3;
        let first_valid = valid_range.contains(&first.abs_diff(*second));

        levels_iter.fold(first_valid, |acc, [a, b]| {
            acc & valid_range.contains(&a.abs_diff(*b)) & (a.cmp(b) == expected_order)
        })
    }
}

fn parse(input: &str) -> Result<Vec<Report>> {
    let mut reports = Vec::new();

    for line in input.trim().lines().map(str::trim) {
        let levels = line
            .split_ascii_whitespace()
            .map(str::parse::<u8>)
            .collect::<Result<_, _>>()?;

        reports.push(Report { levels });
    }

    Ok(reports)
}

fn part1(data: &[Report]) -> u32 {
    let mut num_safe = 0;

    for report in data {
        num_safe += report.is_valid() as u32;
    }

    num_safe
}

fn part2(data: &[Report]) -> u32 {
    let mut num_safe = 0;

    let mut buffer_report = Report { levels: Vec::new() };
    'outer: for report in data {
        if report.is_valid() {
            num_safe += 1;
            continue;
        }

        for i in 0..report.levels.len() {
            buffer_report.levels.clear();
            buffer_report.levels.extend_from_slice(&report.levels);
            buffer_report.levels.remove(i);

            if buffer_report.is_valid() {
                num_safe += 1;
                continue 'outer;
            }
        }
    }

    num_safe
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
        let expected = 2;
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
        let expected = 4;
        let actual = part2(&parsed);

        assert_eq!(expected, actual);
    }
}
