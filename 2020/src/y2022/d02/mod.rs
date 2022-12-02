use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

fn strategy_to_response(_opponent: usize, input: char) -> usize
{
    // XYZ -> ABC
    match input
    {
        'X' => 0,
        'Y' => 1,
        'Z' => 2,
        _ => panic!(),
    }
}

fn strategy_to_winlose(opponent: usize, input: char) -> usize
{
    // XYZ -> win/draw/lose
    match input
    {
        'X' => (opponent + 2) % 3,
        'Y' => opponent,
        'Z' => (opponent + 1) % 3,
        _ => panic!(),
    }
}

fn calc_round_score(opponent: usize, you: char, strategy: fn(usize, char)->usize) -> usize
{
    let you = strategy(opponent, you);

    if opponent == you { you + 1 + 3 } // Draw
    else if (opponent + 1) % 3 == you { you + 1 + 6 }// Win
    else { you + 1 } // Lose
}

fn decode_line(line: &str) -> (usize, char)
{
    (
        (line.chars().next().unwrap() as usize) - ('A' as usize),
        line.chars().skip(2).next().unwrap()
    )
}

fn calc_total_score(input: &str, strategy: fn(usize, char)->usize) -> usize
{
    input_to_lines(input)
        .iter()
        .map(|l| decode_line(&l))
        .map(|(opponent, you)| calc_round_score(opponent, you, strategy))
        .sum::<usize>()
}

fn part_1(input: &str) -> usize
{
    calc_total_score(input, strategy_to_response)
}

fn part_2(input: &str) -> usize
{
    calc_total_score(input, strategy_to_winlose)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(2)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 15,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 12740,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 12,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 11980,
        })
}
