use crate::support::*;

fn required_fuel(mass: u64) -> u64
{
    (mass / 3).saturating_sub(2)
}

fn total_fuel(mass: u64) -> u64
{
    let mut total = 0;
    let mut fuel = mass;

    while fuel > 0
    {
        fuel = required_fuel(fuel);
        total += fuel;
    }

    total
}

fn part_1(input: &str) -> u64
{
    input_to_lines_parsed::<u64>(input)
        .drain(..)
        .map(|m| required_fuel(m))
        .sum()
}

fn part_2(input: &str) -> u64
{
    input_to_lines_parsed::<u64>(input)
        .drain(..)
        .map(|m| total_fuel(m))
        .sum()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(1)
        .example(|| Answer { calculated: required_fuel(12), expected: 2, })
        .example(|| Answer { calculated: required_fuel(14), expected: 2, })
        .example(|| Answer { calculated: required_fuel(1969), expected: 654, })
        .example(|| Answer { calculated: required_fuel(100756), expected: 33583, })
        .part_1(|input| Answer { calculated: part_1(input), expected: 3479429, })
        .example(|| Answer { calculated: total_fuel(1969), expected: 966, })
        .part_2(|input| Answer { calculated: part_2(input), expected: 5216273, })
}
