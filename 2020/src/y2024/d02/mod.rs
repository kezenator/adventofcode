
use itertools::*;
use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");

fn is_safe(report: &Vec<usize>) -> bool
{
    let any_increasing = report.iter().tuple_windows().filter(|(&a, &b)| a <= b).next().is_some();
    let any_decreasing = report.iter().tuple_windows().filter(|(&a, &b)| a >= b).next().is_some();

    if any_increasing && any_decreasing
    {
        return false;
    }

    report.iter().tuple_windows()
        .filter(|(&a, &b)| a.abs_diff(b) > 3)
        .next().is_none()
}

fn is_safe_with_problem_dampener(report: &Vec<usize>) -> bool
{
    if is_safe(report)
    {
        return true;
    }
    for i in 0..report.len()
    {
        let mut removed = report.clone();
        removed.remove(i);
        if is_safe(&removed)
        {
            return true;
        }
    }
    return false;
}

fn part_1(input: &str) -> usize
{
    let reports = input_to_lines_mapped(
        input,
        |l| scan(l).remaining().parse_vec::<usize>(" ").0);

    reports.into_iter()
        .filter(|r| is_safe(r))
        .count()
}

fn part_2(input: &str) -> usize
{
    let reports = input_to_lines_mapped(
        input,
        |l| scan(l).remaining().parse_vec::<usize>(" ").0);

    reports.into_iter()
        .filter(|r| is_safe_with_problem_dampener(r))
        .count()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(2)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 2,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 670,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 4,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 700,
        })
}
