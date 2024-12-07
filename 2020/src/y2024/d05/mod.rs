
use itertools::*;
use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");

fn index_of(update: &Vec<usize>, rule_part: usize) -> Option<usize>
{
    update.iter()
        .enumerate()
        .filter(|u| *u.1 == rule_part)
        .map(|(i, _)| i)
        .next()
}

fn is_in_order(update: &Vec<usize>, rules: &Vec<(usize, usize)>) -> bool
{
    for r in rules.iter()
    {
        if let (Some(ia), Some(ib)) = (index_of(update, r.0), index_of(update, r.1))
        {
            if ia > ib
            {
                return false;
            }
        }
    }
    return true;
}

fn part_1(input: &str) -> usize
{
    let groups = input_to_groups(input);
    let rules: Vec<(usize, usize)> = groups[0].iter()
        .map(|l| scan(l).until("|").parse().remaining().parse())
        .collect_vec();
    let updates: Vec<Vec<usize>> = groups[1].iter()
        .map(|l| scan(l).remaining().parse_vec(",").0)
        .collect_vec();
    
    updates.into_iter()
        .filter(|u| is_in_order(u, &rules))
        .map(|u| u[u.len() / 2])
        .sum()
}

fn part_2(input: &str) -> usize
{
    let groups = input_to_groups(input);
    let rules: Vec<(usize, usize)> = groups[0].iter()
        .map(|l| scan(l).until("|").parse().remaining().parse())
        .collect_vec();
    let updates: Vec<Vec<usize>> = groups[1].iter()
        .map(|l| scan(l).remaining().parse_vec(",").0)
        .collect_vec();

    let bad_updates = updates.into_iter()
        .filter(|u| !is_in_order(u, &rules))
        .collect_vec();

    let fixed_updates = bad_updates.into_iter()
        .map(|mut u|
        {
            u.sort_by(|a, b|
            {
                if rules.contains(&(*a, *b))
                {
                    return Ordering::Less;
                }
                if rules.contains(&(*b, *b))
                {
                    return Ordering::Greater;
                }
                return Ordering::Equal;
            });
            u
        })
        .collect_vec();

    fixed_updates.iter()
        .map(|u| u[u.len() / 2])
        .sum()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(5)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 143,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 4578,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 123,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 6179,
        })
}
