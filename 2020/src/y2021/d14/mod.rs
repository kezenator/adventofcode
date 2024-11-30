use crate::support::*;
use std::collections::HashMap;

const EXAMPLE: &str = include_str!("example.txt");

struct Input
{
    start: String,
    replacements: HashMap<String, char>,
}

fn parse(input: &str) -> Input
{
    let lines = input_to_lines(input);

    let start = lines[0].clone();

    let replacements = lines.iter()
        .skip(2)
        .map(|l|
        {
            scan(l)
                .until(" -> ").parse::<String>()
                .remaining().parse::<char>()
        })
        .collect::<HashMap<String, char>>();

    Input
    {
        start,
        replacements,
    }
}

fn solve(input: &str, iterations: usize) -> usize
{
    let input = parse(input);

    // First, count all pairs

    let mut pair_counts: HashMap<String, usize> = HashMap::new();
    
    for pair in input.start.chars().collect::<Vec<_>>().windows(2)
    {
        *pair_counts.entry(pair.iter().cloned().collect::<String>()).or_insert(0) += 1;
    }

    // Now, remove each pair (a, b) and replace with (a, c) and (c, a) where
    // c is the replacement char for "ab"

    for _ in 0..iterations
    {
        let mut new_pair_counts: HashMap<String, usize> = HashMap::new();

        for (pair, count) in pair_counts
        {
            let a = pair.chars().next().unwrap();
            let b = pair.chars().skip(1).next().unwrap();
            let c = *input.replacements.get(&pair).unwrap();

            let ac = format!("{}{}", a, c);
            let cb = format!("{}{}", c, b);

            *new_pair_counts.entry(ac).or_insert(0) += count;
            *new_pair_counts.entry(cb).or_insert(0) += count;
        }

        pair_counts = new_pair_counts;
    }

    // Convert pair counts to letter counts

    let mut letter_counts: HashMap<char, usize> = HashMap::new();

    for (pair, count) in pair_counts
    {
        let a = pair.chars().next().unwrap();
        let b = pair.chars().skip(1).next().unwrap();

        *letter_counts.entry(a).or_insert(0) += count;
        *letter_counts.entry(b).or_insert(0) += count;
    }

    // Divide each char count by 2 (rounding up)
    // as we've counted each letter twice (start and end of pair)
    // except for the two letters at the start and end (which is why we round up)

    for (_, count) in letter_counts.iter_mut()
    {
        *count = (*count + 1) / 2;
    }

    // Sort by count

    let mut letter_counts = letter_counts.drain().collect::<Vec<(char, usize)>>();
    letter_counts.sort_by(|a, b| a.1.cmp(&b.1));

    // Difference between most frequent and least frequent

    letter_counts[letter_counts.len() - 1].1 - letter_counts[0].1
}

fn part_1(input: &str) -> usize
{
    solve(input, 10)
}

fn part_2(input: &str) -> usize
{
    solve(input, 40)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(14)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 1588,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 3247,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 2188189693529usize,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 4110568157153usize,
        })
}
