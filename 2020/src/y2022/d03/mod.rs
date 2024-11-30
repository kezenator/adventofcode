use crate::support::*;
use itertools::*;

const EXAMPLE: &str = include_str!("example.txt");

fn to_priority(item: char) -> usize
{
    if item.is_lowercase() { 1 + (item as usize) - ('a' as usize) }
    else { 27 + (item as usize) - ('A' as usize) }
}

fn to_parts(line: &str) -> (Vec<char>, Vec<char>)
{
    let len = line.chars().count();
    let each = len / 2;
    return (line.chars().take(each).collect(),
        line.chars().skip(each).collect());
}

fn common_chars(a: &Vec<char>, b: &Vec<char>) -> Vec<char>
{
    a.iter().filter(|c| b.contains(c)).copied().unique().collect()
}

fn part_1(input: &str) -> usize
{
    input_to_lines(input)
        .iter()
        .map(|l| to_parts(l))
        .map(|(a, b)| common_chars(&a, &b))
        .map(|c| c.iter().map(|c| to_priority(*c)).sum::<usize>())
        .sum::<usize>()
}

fn part_2(input: &str) -> usize
{
    input_to_lines(input)
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
        .chunks(3)
        .into_iter()
        .map(|g| common_chars(&g[0], &common_chars(&g[1], &g[2])))
        .flatten()
        .map(|g| to_priority(g))
        .sum::<usize>()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(3)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 157,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 7848,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 70,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 2616,
        })
}
