use std::collections::HashSet;
use crate::support::*;
use itertools::*;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

fn expand_space(input: &str, factor: i64) -> Vec<Point>
{
    let image = CharGrid::new_from_input(input, '.');

    let orig_galaxy_points = image.all_points().into_iter()
        .filter(|p| image.get_char(p) == '#')
        .collect_vec();

    let empty_rows = (0..image.get_height())
        .filter(|r| !orig_galaxy_points.iter().any(|p| p.y == *r))
        .collect_vec();

    let empty_columns = (0..image.get_width())
        .filter(|c| !orig_galaxy_points.iter().any(|p| p.x == *c))
        .collect_vec();

    let expand_coord = |coord: i64, empties: &Vec<i64>| -> i64
    {
        coord + (factor - 1) * (empties.iter().filter(|e| **e < coord).count() as i64)
    };

    orig_galaxy_points.into_iter()
        .map(|p| Point::new(expand_coord(p.x, &empty_columns), expand_coord(p.y, &empty_rows)))
        .collect_vec()
}

fn shortest_pair_paths(galaxy_locations: Vec<Point>) -> i64
{
    let mut sum = 0;

    for i in 0..(galaxy_locations.len() - 1)
    {
        for j in (i + 1)..galaxy_locations.len()
        {
            let dist = (galaxy_locations[i] - galaxy_locations[j]).manhatten_size();

            sum += dist;
        }
    }

    sum
}

fn solve(input: &str, factor: i64) -> i64
{
    shortest_pair_paths(expand_space(input, factor))
}

fn part_1(input: &str) -> i64
{
    solve(input, 2)
}

fn part_2(input: &str) -> i64
{
    solve(input, 1000000)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(11)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 374,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 10033566,
        })
        .example(|| Answer {
            calculated: solve(EXAMPLE, 10),
            expected: 1030,
        })
        .example(|| Answer {
            calculated: solve(EXAMPLE, 100),
            expected: 8410,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 560822911938i64,
        })
}
