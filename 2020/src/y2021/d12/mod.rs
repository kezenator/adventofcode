use crate::support::*;
use std::collections::{ HashMap, HashSet };

const EXAMPLE_1: &str = include_str!("example_1.txt");
const EXAMPLE_2: &str = include_str!("example_2.txt");
const EXAMPLE_3: &str = include_str!("example_3.txt");

fn parse_moves(input: &str) -> HashMap<String, HashSet<String>>
{
    let mut moves = HashMap::<String, HashSet<String>>::new();

    for line in input_to_lines(input)
    {
        let (a, b) = scan(&line)
            .until("-").parse::<String>()
            .remaining().parse::<String>();

        moves.entry(a.clone()).or_default().insert(b.clone());
        moves.entry(b).or_default().insert(a);
    }

    moves
}

fn count_paths<'a, 'b>(pos: &'a String, path: &'b mut Vec<&'a String>, allow_duplicate_small_cave: bool, moves: &'a HashMap<String, HashSet<String>>) -> usize
{
    let mut result = 0;

    for dest in moves.get(pos).unwrap()
    {
        if dest == "end"
        {
            // Found the end cave - count as one path and we're done
            result += 1;
        }
        else if dest != "start"
        {
            let already_visited = path.iter_mut().position(|p| *p == dest).is_some();
    
            path.push(dest);
    
            if dest.chars().next().unwrap().is_lowercase()
            {
                if already_visited
                {
                    if allow_duplicate_small_cave
                    {
                        // Visiting first cave twice - don't allow this for another cave!
                        result += count_paths(dest, path, false, moves);
                    }
                }
                else
                {
                    // Visiting new small cave
                    result += count_paths(dest, path, allow_duplicate_small_cave, moves);
                }
            }
            else
            {
                // Visiting a big cave
                result += count_paths(dest, path, allow_duplicate_small_cave, moves);
            }
    
            path.pop();
        }
        else
        {
            // Can't move back to start
        }
    }

    result
}

fn count_all_paths(input: &str, allow_duplicate_small_cave: bool) -> usize
{
    let start = "start".to_owned();
    let mut path = vec![&start];

    count_paths(
        &start,
        &mut path,
        allow_duplicate_small_cave,
        &parse_moves(input))
}

fn part_1(input: &str) -> usize
{
    count_all_paths(input, false)
}

fn part_2(input: &str) -> usize
{
    count_all_paths(input, true)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(12)
        .example(|| Answer {
            calculated: part_1(EXAMPLE_1),
            expected: 10,
        })
        .example(|| Answer {
            calculated: part_1(EXAMPLE_2),
            expected: 19,
        })
        .example(|| Answer {
            calculated: part_1(EXAMPLE_3),
            expected: 226,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 3000,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE_1),
            expected: 36,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE_2),
            expected: 103,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE_3),
            expected: 3509,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 74222,
        })
}
