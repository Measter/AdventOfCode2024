use aoc_lib::{Bench, BenchResult, Day, NoError};
use logos::{Lexer, Logos};

pub const DAY: Day = Day {
    day: 3,
    name: "Mull it Over",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    b.bench(|| Ok::<_, NoError>(part1(input)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    b.bench(|| Ok::<_, NoError>(part2(input)))
}

#[derive(Debug)]
struct Mul {
    l: u32,
    r: u32,
}

impl Mul {
    fn parse(lexer: &mut Lexer<'_, Token>) -> Self {
        let (left, rest) = lexer.slice()[4..].split_once(',').unwrap();
        let (right, _) = rest.split_once(')').unwrap();
        Mul {
            l: left.parse().unwrap(),
            r: right.parse().unwrap(),
        }
    }
}

#[derive(Debug, Logos)]
enum Token {
    #[regex(r#"mul\(\d{1,3},\d{1,3}\)"#, Mul::parse)]
    Mul(Mul),

    #[token("do()")]
    Do,

    #[token("don't()")]
    Dont,

    #[regex(".")]
    Other,
}

fn part1(data: &str) -> u32 {
    Token::lexer(data)
        .filter_map(|t| {
            let Ok(Token::Mul(Mul { l, r })) = t else {
                return None;
            };
            Some(l * r)
        })
        .sum()
}

fn part2(data: &str) -> u32 {
    let mut enabled = true;
    let mut sum = 0;

    for tk in Token::lexer(data) {
        match tk {
            Ok(Token::Mul(Mul { l, r })) if enabled => {
                sum += l * r;
            }
            Ok(Token::Do) => enabled = true,
            Ok(Token::Dont) => enabled = false,
            _ => continue,
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

        let expected = 161;
        let actual = part1(&data);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part2, 1)
            .open()
            .unwrap();

        let expected = 48;
        let actual = part2(&data);

        assert_eq!(expected, actual);
    }
}
