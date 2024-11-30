use crate::support::*;
use itertools::*;

const EXAMPLE: &str = include_str!("example.txt");

fn sum_of_max_n_elves(input: &str, num_elves: usize) -> usize
{
    input_to_groups(input)
        .iter()
        .map(|v| v.iter().map(|s| s.parse::<usize>().unwrap()).sum::<usize>())
        .sorted()
        .rev()
        .take(num_elves)
        .sum()
}

fn part_1(input: &str) -> usize
{
    sum_of_max_n_elves(input, 1)
}

fn part_2(input: &str) -> usize
{
    sum_of_max_n_elves(input, 3)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(1)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 24000,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 68923,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 45000,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 200044,
        })
}
