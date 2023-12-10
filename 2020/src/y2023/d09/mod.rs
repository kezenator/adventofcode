use crate::support::*;
use itertools::*;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

fn calc_differences(seq: Vec<i64>) -> Vec<Vec<i64>>
{
    let mut diffs = vec![seq];

    while diffs[diffs.len() - 1].iter().any(|v| *v != 0)
    {
        let diff_seq = diffs[diffs.len() - 1]
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect_vec();
        diffs.push(diff_seq);
    }

    diffs
}

fn extrapolate<F1: Fn(&Vec<i64>) -> i64, F2: Fn(i64, i64) -> i64>(seq: Vec<i64>, f_entry: F1, f_apply: F2) -> i64
{
    let diffs = calc_differences(seq);
    let num_seq = diffs.len();
    let mut prev_diff = 0;

    for i in 1..num_seq
    {
        let last_val = f_entry(&diffs[num_seq - i - 1]);
        prev_diff = f_apply(last_val, prev_diff);
    }

    prev_diff
}

fn next_value(seq: Vec<i64>) -> i64
{
    extrapolate(seq, |seq| seq[seq.len() - 1], |a, b| a + b)
}

fn prev_value(seq: Vec<i64>) -> i64
{
    extrapolate(seq, |seq| seq[0], |a, b| a - b)
}

fn part_1(input: &str) -> i64
{
    input_to_lines(input).into_iter()
        .map(|l| l.split_ascii_whitespace().map(|p| p.parse::<i64>().unwrap()).collect_vec())
        .map(|seq| next_value(seq))
        .sum()
}

fn part_2(input: &str) -> i64
{
    input_to_lines(input).into_iter()
        .map(|l| l.split_ascii_whitespace().map(|p| p.parse::<i64>().unwrap()).collect_vec())
        .map(|seq| prev_value(seq))
        .sum()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(9)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 114,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 2008960228,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 2,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 1097,
        })
}
