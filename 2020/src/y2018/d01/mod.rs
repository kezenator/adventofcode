use crate::support::*;
use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

fn part_1(input: &str) -> i32
{
    input_to_lines(input).iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .sum::<i32>()
}

fn part_2(input: &str) -> i32
{
    let changes = input_to_lines(input).iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    
    let mut sum = 0;
    let mut found = HashSet::<i32>::new();
    found.insert(sum);

    loop
    {
        for change in changes.iter()
        {
            sum += change;
            if !found.insert(sum)
            {
                return sum;
            }
        }
    }
}


pub fn puzzles() -> PuzzleDay
{
    puzzle_day(1)
        .example(|| Answer {
            calculated: part_1("+1\n+1\n+1\n"),
            expected: 3, })
        .example(|| Answer {
            calculated: part_1("+1\n+1\n-2\n"),
            expected: 0, })
        .example(|| Answer {
            calculated: part_1("-1\n-2\n-3\n"),
            expected: -6, })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 522, })
        .example(|| Answer {
            calculated: part_2("+1\n-1\n"),
            expected: 0, })
        .example(|| Answer {
            calculated: part_2("+3\n+3\n+4\n-2\n-4\n"),
            expected: 10, })
        .example(|| Answer {
            calculated: part_2("-6\n+3\n+8\n+5\n-6\n"),
            expected: 5, })
        .example(|| Answer {
            calculated: part_2("+7\n+7\n-2\n-7\n-4\n"),
            expected: 14, })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 73364, })
}
