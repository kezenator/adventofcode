use crate::support::*;

const INPUT: &str = include_str!("input.txt");

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

fn part_1() -> u64
{
    input_to_lines_parsed::<u64>(INPUT)
        .drain(..)
        .map(|m| required_fuel(m))
        .sum()
}

fn part_2() -> u64
{
    input_to_lines_parsed::<u64>(INPUT)
        .drain(..)
        .map(|m| total_fuel(m))
        .sum()
}

pub fn puzzles(puzzles: &mut PuzzleSet)
{
    puzzles.register("y2019.d01.e1", || Answer {
        calculated: required_fuel(12),
        expected: 2,
    });

    puzzles.register("y2019.d01.e2", || Answer {
        calculated: required_fuel(14),
        expected: 2,
    });

    puzzles.register("y2019.d01.e3", || Answer {
        calculated: required_fuel(1969),
        expected: 654,
    });

    puzzles.register("y2019.d01.e4", || Answer {
        calculated: required_fuel(100756),
        expected: 33583,
    });

    puzzles.register("y2019.d01.e5", || Answer {
        calculated: total_fuel(1969),
        expected: 966,
    });

    puzzles.register("y2019.d01.e6", || Answer {
        calculated: total_fuel(100756),
        expected: 50346,
    });

    puzzles.register("y2019.d01.p1", || Answer {
        calculated: part_1(),
        expected: 3479429,
    });

    puzzles.register("y2019.d01.p2", || Answer {
        calculated: part_2(),
        expected: 5216273,
    });
}
