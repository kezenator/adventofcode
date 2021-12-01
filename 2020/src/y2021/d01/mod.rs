use crate::support::*;
use itertools::Itertools;

const EXAMPLE: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";
const INPUT: &str = include_str!("input.txt");

fn part_1(input: &str) -> usize
{
    input_to_lines_parsed::<u64>(input)
        .windows(2)
        .filter(|s| s[1] > s[0])
        .count()
}

fn part_2(input: &str) -> usize
{
    input_to_lines_parsed::<u64>(input)
        .windows(3)
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|s| (*s[1]).iter().sum::<u64>() > (*s[0]).iter().sum::<u64>())
        .count()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(1)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 7,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 1446,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 5,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 1486,
        })
}
