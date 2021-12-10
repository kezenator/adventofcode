use crate::support::*;
use std::collections::VecDeque;

const EXAMPLE: &str = "[({(<(())[]>[[{[]{<()<>>\n[(()[<>])]({[<{<<[]>>(\n{([(<{}[<>[]}>{[]{[(<()>\n(((({<>}<{<{<>}{[]{[]{}\n[[<[([]))<([[{}[[()]]]\n[{[{({}]{}}([{[{{{}}([]\n{<[[]]>}<{[{[{[]{()[[[]\n[<(<(<(<{}))><([]([]()\n<{([([[(<>()){}]>(<<{{\n<{([{{}}[<[[[<>{}]]]>[]]";
const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
enum ParseResult
{
    Valid,
    Corrupted(char),
    Incomplete(String),
}

fn closing(ch: char) -> Option<char>
{
    match ch
    {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        _ => None,
    }
}

fn corrupt_score(ch: char) -> usize
{
    match ch
    {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn incomplete_score(closing: &str) -> usize
{
    let mut result = 0;

    for ch in closing.chars()
    {
        let part = match ch
        {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => unreachable!(),
        };

        result = (5 * result) + part;
    }

    result
}

fn parse_chunk(chunk: &str) -> ParseResult
{
    let mut stack: VecDeque<char> = VecDeque::new();

    for ch in chunk.chars()
    {
        if let Some(close) = closing(ch)
        {
            stack.push_back(close);
        }
        else if *stack.back().unwrap() == ch
        {
            stack.pop_back();
        }
        else
        {
            return ParseResult::Corrupted(ch);
        }
    }

    if stack.is_empty()
    {
        ParseResult::Valid
    }
    else
    {
        ParseResult::Incomplete(stack.iter().rev().copied().collect())
    }
}

fn part_1(input: &str) -> usize
{
    input_to_lines(input)
        .iter()
        .map(|l| parse_chunk(l))
        .map(|r| if let ParseResult::Corrupted(ch) = r { corrupt_score(ch) } else { 0 })
        .sum()
}

fn part_2(input: &str) -> usize
{
    let mut scores = input_to_lines(input)
        .iter()
        .map(|l| parse_chunk(l))
        .filter(|r| if let ParseResult::Incomplete(_) = r { true } else { false })
        .map(|r| if let ParseResult::Incomplete(close) = r { close.clone() } else { String::new() })
        .map(|close| incomplete_score(&close))
        .collect::<Vec<usize>>();

    scores.sort();

    assert!((scores.len() % 2) == 1);

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
