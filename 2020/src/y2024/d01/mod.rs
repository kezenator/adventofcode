
use itertools::*;
use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");

fn part_1(input: &str) -> usize
{
    let pairs: Vec<(usize, usize)> = input_to_lines(input).iter()
        .map(|l| scan(l).until_whitespace().parse().skip_ws().remaining().parse())
        .collect();

    let l = pairs.iter()
        .map(|p| p.0)
        .sorted()
        .collect_vec();

    let r = pairs.iter()
        .map(|p| p.1)
        .sorted()
        .collect_vec();

    let pairs = l.iter().cloned()
        .zip(r.iter().cloned())
        .collect_vec();

    pairs.into_iter()
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

fn part_2(input: &str) -> usize
{
    let pairs: Vec<(usize, usize)> = input_to_lines(input).iter()
        .map(|l| scan(l).until_whitespace().parse().skip_ws().remaining().parse())
        .collect();

    let l = pairs.iter()
        .map(|p| p.0)
        .collect_vec();

    let r = pairs.iter()
        .map(|p| p.1)
        .collect_vec();

    l.into_iter()
        .map(|ln| ln * r.iter().cloned().filter(|rn| *rn == ln).count())
        .sum()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(1)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 11,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 1506483,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 31,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 23126924,
        })
}
