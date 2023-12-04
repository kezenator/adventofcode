use std::collections::HashSet;
use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

struct Card
{
    winning_numbers: HashSet<u64>,
    numbers_you_have: Vec<u64>,
}

impl std::str::FromStr for Card
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let (_id, list1, list2) = scan(s)
            .skip_str("Card")
            .skip_ws()
            .until(": ").parse::<u64>()
            .until(" | ").parse::<String>()
            .remaining().parse::<String>();

        let winning_numbers = list1.split(" ").filter(|p| !p.is_empty()).map(|p| p.parse::<u64>().unwrap()).collect();
        let numbers_you_have = list2.split(" ").filter(|p| !p.is_empty()).map(|p| p.parse::<u64>().unwrap()).collect();

        Ok(Card { winning_numbers, numbers_you_have })
    }
}

impl Card
{
    fn num_winning_numbers_you_have(&self) -> usize
    {
        self.numbers_you_have.iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count()
    }

    fn points(&self) -> u64
    {
        let num_winning_numbers_you_have = self.num_winning_numbers_you_have();

        if num_winning_numbers_you_have == 0
        {
            0
        }
        else
        {
            1 << (num_winning_numbers_you_have - 1)
        }
    }
}

fn part_1(input: &str) -> u64
{
    input_to_lines_parsed::<Card>(input).iter()
        .map(|c| c.points())
        .sum()
}

fn part_2(input: &str) -> u64
{
    let cards = input_to_lines_parsed::<Card>(input);
    let mut num_copies = vec![1; cards.len()];

    for i in 0..cards.len()
    {
        let num_winning = cards[i].num_winning_numbers_you_have();
        for c in 0..num_winning
        {
            num_copies[i + c + 1] += num_copies[i];
        }
    }

    num_copies.iter().sum()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(4)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 13,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 22488,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 30,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 7013204,
        })
}
