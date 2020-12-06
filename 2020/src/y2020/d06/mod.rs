use std::collections::HashSet;
use crate::support::*;

const EXAMPLE: &str = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
const INPUT: &str = include_str!("input.txt");

fn part_1(input: &str) -> usize
{
    let mut result = 0;

    for group in input_to_groups(input)
    {
        let mut set = HashSet::new();

        for line in group
        {
            for ch in line.chars()
            {
                set.insert(ch);
            }
        }

        result += set.len();
    }

    result
}

fn part_2(input: &str) -> usize
{
    let mut result = 0;

    for group in input_to_groups(input)
    {
        let first = group.iter().nth(0).unwrap().clone();

        for ch in first.chars()
        {
            if group.iter()
                .filter(|l| l.chars().find(|c| *c == ch).is_some())
                .count() == group.len()
            {
                result += 1;
            }
        }
    }

    result
}

pub fn puzzles(puzzles: &mut PuzzleSet)
{
    puzzles.register("y2020.d06.e1", || Answer {
        calculated: part_1(EXAMPLE),
        expected: 11,
    });

    puzzles.register("y2020.d06.e2", || Answer {
        calculated: part_2(EXAMPLE),
        expected: 6,
    });

    puzzles.register("y2020.d06.p1", || Answer {
        calculated: part_1(INPUT),
        expected: 6809,
    });

    puzzles.register("y2020.d06.p2", || Answer {
        calculated: part_2(INPUT),
        expected: 3394,
    });
}
