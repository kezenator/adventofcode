use crate::support::*;
use itertools::*;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

type Round = [u64;3];
struct Game
{
    id: u64,
    rounds: Vec<Round>,
}

fn parse_round(round: &str) -> Round
{
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for part in round.split(",")
    {
        let (num, color) = scan(part)
            .skip_str(" ")
            .until(" ").parse::<u64>()
            .remaining().parse::<String>();

        match color.as_str()
        {
            "red" => red += num,
            "green" => green += num,
            "blue" => blue += num,
            _ => unreachable!(),
        }
    }
    
    [red, green, blue]
}

fn parse_game(line: &str) -> Game
{
    let (id, rounds) = scan(line).skip_str("Game ")
        .until(":").parse::<u64>()
        .remaining().parse::<String>();
    let rounds = rounds.split(";").collect_vec();
    let rounds = rounds.iter().map(|r| parse_round(r)).collect_vec();

    Game { id, rounds }
}

fn part_1(input: &str) -> u64
{
    let is_valid = |g: &Game|
    {
        g.rounds.iter().all(|r| r[0] <= 12 && r[1] <= 13 && r[2] <= 14)
    };

    input_to_lines(input).iter()
        .map(|l| parse_game(l))
        .filter(is_valid)
        .map(|g| g.id)
        .sum()
}

fn part_2(input: &str) -> u64
{
    let power_set = |g: Game|
    {
        let max_r = g.rounds.iter().map(|r| r[0]).max().unwrap();
        let max_g = g.rounds.iter().map(|r| r[1]).max().unwrap();
        let max_b = g.rounds.iter().map(|r| r[2]).max().unwrap();
    
        max_r * max_g * max_b
    };

    input_to_lines(input).iter()
        .map(|l| parse_game(l))
        .map(power_set)
        .sum()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(2)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 8,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 2101,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 2286,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 58269,
        })
}
