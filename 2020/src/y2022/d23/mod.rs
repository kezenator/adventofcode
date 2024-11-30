use crate::support::*;
use std::collections::{HashMap, HashSet};

const EXAMPLE: &str = include_str!("example.txt");

fn parse(input: &str) -> Vec<Point>
{
    let grid = CharGrid::new_from_input(input, '.');

    grid.all_points()
        .into_iter()
        .filter(|p| grid.get_char(p) == '#')
        .collect()
}

fn one_round(input: Vec<Point>, round_num: usize) -> (Vec<Point>, bool)
{
    const DIRS: [[Point;3];4] =
    [
        [ Point{x:-1,y:-1}, Point{x:0,y:-1}, Point{x:1,y:-1}],
        [ Point{x:-1,y:1}, Point{x:0,y:1}, Point{x:1,y:1}],
        [ Point{x:-1,y:-1}, Point{x:-1,y:0}, Point{x:-1,y:1}],
        [ Point{x:1,y:-1}, Point{x:1,y:0}, Point{x:1,y:1}],
    ];

    let input: HashSet<Point> = input.into_iter().collect();
    let mut dests: HashMap<Point, Vec<Point>> = HashMap::new();

    for elf_pos in input.iter()
    {
        let mut dest = elf_pos.clone();

        let any_neighbour_occupied = elf_pos.neighbours_8()
            .any(|n| input.contains(&n));

        if any_neighbour_occupied
        {
            for offset in 0..4
            {
                let dirs = &DIRS[(round_num + offset) % 4];

                let all_free_in_dir = dirs.iter()
                    .map(|dir| *elf_pos + *dir)
                    .all(|n| !input.contains(&n));

                if all_free_in_dir
                {
                    dest = *elf_pos + dirs[1];
                    break;
                }
            }
        }

        dests.entry(dest).or_default().push(elf_pos.clone());
    }

    let mut any_moved = false;
    let mut output = Vec::new();
    output.reserve(input.len());

    for (dest, sources) in dests
    {
        if sources.len() == 1
        {
            if sources[0] != dest { any_moved = true; }
            output.push(dest);
        }
        else // Multiple going to the same spot - none move
        {
            output.extend(sources);
        }
    }

    (output, any_moved)
}

fn part_1(input: &str) -> usize
{
    let mut elfs = parse(input);

    for round in 0..10
    {
        (elfs, _) = one_round(elfs, round);
    }

    let grid = CharGrid::new_from_points(elfs);

    grid.all_chars()
        .into_iter()
        .filter(|c| *c == '.')
        .count()
}

fn part_2(input: &str) -> usize
{
    let mut elfs = parse(input);

    for round in 0..usize::max_value()
    {
        let any_moved;
        (elfs, any_moved) = one_round(elfs, round);

        if !any_moved
        {
            return round + 1;
        }
    }
    unreachable!();
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(23)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 110,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 3780,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 20,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 930,
        })
}
