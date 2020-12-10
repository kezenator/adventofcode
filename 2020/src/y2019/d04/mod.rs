use crate::support::*;

fn increasing(s: &str) -> bool
{
    s.chars()
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|w| w[0] > w[1])
        .next()
        .is_none()
}

fn has_run(s: &str, len: usize) -> bool
{
    string_split_into_runs(s)
        .iter()
        .map(|s| s.len())
        .filter(|l| *l == len)
        .next()
        .is_some()
}

fn is_valid_1(password: u64) -> bool
{
    let password = password.to_string();

    (password.len() == 6)
        && increasing(&password)
        && (has_run(&password, 2)
            || has_run(&password, 3)
            || has_run(&password, 4)
            || has_run(&password, 5)
            || has_run(&password, 6))
}

fn is_valid_2(password: u64) -> bool
{
    let password = password.to_string();

    (password.len() == 6)
        && increasing(&password)
        && has_run(&password, 2)
}

fn part_1() -> usize
{
    (236491..713788)
        .filter(|p| is_valid_1(*p))
        .count()
}

fn part_2() -> usize
{
    (236491..713788)
        .filter(|p| is_valid_2(*p))
        .count()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(4)
        .example(|| Answer { calculated: is_valid_1(111111), expected: true, })
        .example(|| Answer { calculated: is_valid_1(223450), expected: false, })
        .example(|| Answer { calculated: is_valid_1(123789), expected: false, })
        .part_1(|| Answer { calculated: part_1(), expected: 1169, })
        .example(|| Answer { calculated: is_valid_2(112233), expected: true, })
        .example(|| Answer { calculated: is_valid_2(123444), expected: false, })
        .example(|| Answer { calculated: is_valid_2(111122), expected: true, })
        .part_2(|| Answer { calculated: part_2(), expected: 757, })
}
