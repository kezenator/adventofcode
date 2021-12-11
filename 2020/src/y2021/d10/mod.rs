use crate::support::*;
use std::collections::VecDeque;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

enum Parse
{
    Incomplete(String),
    Corrupt(char),
}

fn close(c: char) -> Option<char>
{
    match c
    {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        _ => None
    }
}

fn parse(input: &str) -> Parse
{
    let mut stack = VecDeque::new();

    for c in input.chars()
    {
        if let Some(close) = close(c)
        {
            stack.push_back(close);
        }
        else if c == *stack.back().unwrap()
        {
            stack.pop_back();
        }
        else
        {
            return Parse::Corrupt(c);
        }
    }

    assert!(!stack.is_empty());

    Parse::Incomplete(stack.iter().copied().rev().collect())
}

fn illegal_score(c: char) -> usize
{
    match c
    {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn incomplete_score(s: &str) -> usize
{
    s.chars()
        .map(|c|
        {
            match c
            {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => unreachable!(),
            }
        })
        .fold(0, |a, b| (a * 5) + b)
}

fn part_1(input: &str) -> usize
{
    input_to_lines(input).iter()
        .map(|s| parse(s))
        .filter_map(|p| if let Parse::Corrupt(c) = p { Some(illegal_score(c)) } else { None })
        .sum()
}

fn part_2(input: &str) -> usize
{
    let mut scores = input_to_lines(input).iter()
        .map(|s| parse(s))
        .filter_map(|p| if let Parse::Incomplete(s) = p { Some(incomplete_score(&s)) } else { None })
        .collect::<Vec<_>>();

    assert!((scores.len() % 2) == 1);

    scores.sort();

    scores[scores.len() / 2]
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(10)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 26397,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 311895,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 288957,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 2904180541usize,
        })
}
