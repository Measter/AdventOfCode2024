use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError, misc::ArrChunks};
use color_eyre::{Report, Result, eyre::eyre};

pub const DAY: Day = Day {
    day: 9,
    name: "Disk Fragmenter",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse P1", run_parse_p1), ("Parse P2", run_parse_p2)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse_p1(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1(&data)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let data = parse_p2(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part2(&data)))
}

fn run_parse_p1(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse_p1(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

fn run_parse_p2(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse_p2(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

fn parse_p1(input: &str) -> Result<Vec<u16>> {
    let mut filemap = Vec::new();
    let [first, rest @ ..] = input.trim().as_bytes() else {
        return Err(eyre!("Invaild input"));
    };

    let mut add_sectors = |len: u8, id: Option<u16>| {
        let id = id.unwrap_or(u16::MAX);
        filemap.extend(std::iter::repeat_n(id, len as usize));
    };

    add_sectors(first - b'0', Some(0));

    for (&[gap, file], file_id) in ArrChunks::new(rest).zip(1..) {
        add_sectors(gap - b'0', None);
        add_sectors(file - b'0', Some(file_id));
    }

    Ok(filemap)
}

fn part1(file_map: &[u16]) -> usize {
    let mut file_map = file_map.to_owned();

    let mut gap_idx = 0;

    loop {
        while gap_idx < file_map.len() {
            if file_map[gap_idx] == u16::MAX {
                break;
            }
            gap_idx += 1;
        }

        if gap_idx == file_map.len() {
            break;
        }

        file_map.swap_remove(gap_idx);

        while file_map.last() == Some(&u16::MAX) {
            file_map.pop();
        }
    }

    file_map
        .into_iter()
        .enumerate()
        .map(|(id, val)| id * (val as usize))
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Chunk {
    File { len: u8, id: u16 },
    Space { len: u8 },
}

fn parse_p2(input: &str) -> Result<Vec<Chunk>> {
    let mut filemap = Vec::new();
    let [first, rest @ ..] = input.trim().as_bytes() else {
        return Err(eyre!("Invaild input"));
    };

    filemap.push(Chunk::File {
        len: first - b'0',
        id: 0,
    });

    for (&[gap, file], file_id) in ArrChunks::new(rest).zip(1..) {
        let gap_len = gap - b'0';
        if gap_len != 0 {
            filemap.push(Chunk::Space { len: gap_len });
        }

        let file_len = file - b'0';
        if file_len != 0 {
            filemap.push(Chunk::File {
                len: file_len,
                id: file_id,
            });
        }
    }

    Ok(filemap)
}

fn part2(file_map: &[Chunk]) -> usize {
    let mut file_map = file_map.to_owned();

    let mut file_idx = file_map.len() - 1;
    while file_idx > 0 {
        let file @ Chunk::File { len: file_len, .. } = file_map[file_idx] else {
            file_idx -= 1;
            continue;
        };

        let gap_idx = file_map[..file_idx]
            .iter()
            .position(|c| matches!(c, Chunk::Space { len } if *len >= file_len));

        if let Some(gap_idx) = gap_idx {
            // We found a gap the file can fit in.
            file_map[file_idx] = Chunk::Space { len: file_len };

            let Chunk::Space { len: gap_len } = &mut file_map[gap_idx] else {
                unreachable!()
            };

            if *gap_len == file_len {
                // The simple case where we can just replace the gap entirely.
                file_map[gap_idx] = file;
            } else {
                // The more complex case, where have to split the gap.
                *gap_len -= file_len;
                file_map.insert(gap_idx, file);
                // We've moved everything up by one, so we need to increment the file index so we don't miss anything.
                file_idx += 1;
            }
        }

        file_idx -= 1;
    }

    // print_map(&file_map);

    let mut block_id = 0;
    let mut sum = 0;

    for chunk in file_map {
        match chunk {
            Chunk::File { len, id } => {
                for i in 0..len as usize {
                    sum += id as usize * (i + block_id);
                }
                block_id += len as usize;
            }
            Chunk::Space { len } => block_id += len as usize,
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn parse_p1_test() {
        let expected: Vec<u16> = vec![
            0,
            u16::MAX,
            u16::MAX,
            1,
            1,
            1,
            u16::MAX,
            u16::MAX,
            u16::MAX,
            u16::MAX,
            2,
            2,
            2,
            2,
            2,
        ];
        let actual = parse_p1("12345").unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn part1_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let parsed = parse_p1(&data).unwrap();
        let expected = 1928;
        let actual = part1(&parsed);

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_p2_test() {
        let expected = vec![
            Chunk::File { len: 1, id: 0 },
            Chunk::Space { len: 2 },
            Chunk::File { len: 3, id: 1 },
            Chunk::Space { len: 4 },
            Chunk::File { len: 5, id: 2 },
        ];
        let actual = parse_p2("12345").unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let parsed = parse_p2(&data).unwrap();
        let expected = 2858;
        let actual = part2(&parsed);

        assert_eq!(expected, actual);
    }
}
