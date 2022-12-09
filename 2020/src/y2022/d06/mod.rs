use crate::support::*;
use itertools::*;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

fn all_different(packet: &&[char]) -> bool
{
    for i in 0..packet.len()
    {
        for j in i + 1 .. packet.len()
        {
            if packet[i] == packet[j] { return false; }
        }
    }
    return true;
}

fn find_marker(input: &str, len: usize) -> usize
{
    input_to_lines(input)[0]
        .chars()
        .collect_vec()
        .windows(len)
        .find_position(all_different).unwrap().0 + len
}

fn part_1(input: &str) -> usize
{
    find_marker(input, 4)
}

fn part_2(input: &str) -> usize
{
    find_marker(input, 14)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(6)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 7,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 1816,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 19,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 2625,
        })
}
