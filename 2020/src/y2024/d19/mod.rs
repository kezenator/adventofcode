
use rayon::prelude::*;
use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");

fn count_ways_to_make_pattern(pattern: &str, towels: &Vec<String>) -> usize
{
    let memorized = Memorized::new(&|input: &(&str, &Vec<String>), memorized| -> usize
    {
        if input.0.len() == 0
        {
            return 1;
        }
        let mut result = 0;
        for t in input.1
        {
            if input.0.starts_with(t)
            {
                result += memorized.get(&(&input.0[t.len()..], input.1));
            }
        }
        result
    });

    memorized.get(&(pattern, towels))
}

fn part_1(input: &str) -> usize
{
    let groups = input_to_groups(input);
    let towels = scan(&groups[0][0]).remaining().parse_vec::<String>(", ").0;
    let patterns = groups[1].clone();

    patterns.into_iter()
        .filter(|pattern| count_ways_to_make_pattern(&pattern, &towels) != 0)
        .count()
}


fn part_2(input: &str) -> usize
{
    let groups = input_to_groups(input);
    let towels = scan(&groups[0][0]).remaining().parse_vec::<String>(", ").0;
    let patterns = groups[1].clone();

    patterns.par_iter()
        .map(|pattern| count_ways_to_make_pattern(&pattern, &towels))
        .sum()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(19)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 6,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 315,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 16,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 625108891232249usize,
        })
}
