use crate::support::*;
use std::collections::HashMap;

const EXAMPLE_1: &str = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
const EXAMPLE_2: &str = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";

fn parse_orbits(input: &str) -> HashMap<String, String>
{
    input_to_lines(input)
        .iter()
        .map(|s| {
            scan(s)
                .until(")").parse::<String>()
                .remaining().parse::<String>()
        })
        .map(|(a, b)| (b, a))
        .collect()
}

fn path_to_com(o: &String, orbits: &HashMap<String, String>) -> Vec<String>
{
    let mut result = Vec::new();
    let mut cur = o;

    while *cur != "COM"
    {
        cur = orbits.get(cur).unwrap();
        result.push(cur.clone());
    }

    result
}

fn part_1(input: &str) -> usize
{
    let orbits = parse_orbits(input);

    let objects = orbits
        .iter()
        .map(|(a, _b)| a.to_owned())
        .collect::<Vec<String>>();

    objects.iter()
        .map(|o| path_to_com(o, &orbits).len())
        .sum()
}

fn part_2(input: &str) -> usize
{
    let orbits = parse_orbits(input);

    let mut p1 = path_to_com(&"YOU".to_owned(), &orbits);
    let mut p2 = path_to_com(&"SAN".to_owned(), &orbits);

    while p1[p1.len() - 1] == p2[p2.len() - 1]
    {
        p1.pop();
        p2.pop();
    }

    p1.len() + p2.len()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(6)
        .example(|| Answer { calculated: part_1(EXAMPLE_1), expected: 42, })
        .part_1(|input| Answer { calculated: part_1(input), expected: 150150, })
        .example(|| Answer { calculated: part_2(EXAMPLE_2), expected: 4, })
        .part_2(|input| Answer { calculated: part_2(input), expected: 352, })
}
