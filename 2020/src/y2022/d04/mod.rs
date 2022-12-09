use crate::support::*;
use std::ops::Range;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> Vec<(Range<usize>, Range<usize>)>
{
    input_to_lines(input)
        .iter()
        .map(|l|
            {
                scan(l)
                .until("-").parse::<usize>()
                .until(",").parse::<usize>()
                .until("-").parse::<usize>()
                .remaining().parse::<usize>()
            })
        .map(|t| (t.0..(t.1+1), t.2..(t.3+1)))
        .collect()
}

fn contains(r1: &Range<usize>, r2: &Range<usize>) -> bool
{
    r1.start <= r2.start && r1.end >= r2.end
}

fn overlaps(r1: &Range<usize>, r2: &Range<usize>) -> bool
{
    r1.start < r2.end && r1.end > r2.start
}

fn part_1(input: &str) -> usize
{
    parse(input)
        .iter()
        .filter(|l| contains(&l.0, &l.1) || contains(&l.1, &l.0))
        .count()
}

fn part_2(input: &str) -> usize
{
    parse(input)
        .iter()
        .filter(|l| overlaps(&l.0, &l.1))
        .count()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(4)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 2,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 475,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 4,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 825,
        })
}
