use crate::support::*;

const EXAMPLE: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
const INPUT: &str = include_str!("input.txt");

fn part_1(input: &str) -> usize
{
    let mut horiz = 0;
    let mut depth = 0;

    for line in input_to_lines(input)
    {
        let (dir, num) = scan(&line)
            .until(" ").parse::<String>()
            .remaining().parse::<usize>();

        match dir.as_ref()
        {
            "forward" => horiz += num,
            "up" => depth -= num,
            "down" => depth += num,
            _ => unreachable!(),
        };
    }

    horiz * depth
}

fn part_2(input: &str) -> usize
{
    let mut horiz = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in input_to_lines(input)
    {
        let (dir, num) = scan(&line)
            .until(" ").parse::<String>()
            .remaining().parse::<usize>();

        match dir.as_ref()
        {
            "forward" =>
            {
                horiz += num;
                depth += aim * num;
            },
            "up" => aim -= num,
            "down" => aim += num,
            _ => unreachable!(),
        };
    }

    horiz * depth
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(2)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 150,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 1924923,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 900,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 1982495697,
        })
}
