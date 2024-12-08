
use itertools::*;
use crate::support::*;
use std::collections::{HashMap, HashSet};

const EXAMPLE: &str = include_str!("example.txt");

fn find_antinodes(grid: &CharGrid, pa: &Point, pb: &Point, use_harmonics: bool) -> Vec<Point>
{
    let mut result = Vec::new();

    let diff = *pb - *pa;
    let max_spacings =
        (grid.get_width().max(grid.get_height()) + 1)
        / (diff.x.abs().min(diff.y.abs()));

    for offset in (-max_spacings)..(max_spacings + 1)
    {
        let consider = if use_harmonics
        {
            true
        }
        else
        {
            (offset == -1) || (offset == 2)
        };

        let p = *pa + offset * diff;
        if consider && grid.is_point_in_bounds(&p)
        {
            result.push(p);
        }
    }

    result
}

fn num_antinodes(input: &str, use_harmonics: bool) -> usize
{
    let grid = CharGrid::new_from_input(input, '.');

    let freq_to_points = grid.all_points().into_iter()
        .filter(|p| grid.get_char(p) != '.')
        .into_group_map_by(|p| grid.get_char(p));

    let mut antinode_locations = HashSet::new();

    for (_, points) in freq_to_points.into_iter()
    {
        for pair in points.iter().combinations(2)
        {
            for p in find_antinodes(&grid, pair[0], pair[1], use_harmonics)
            {
                antinode_locations.insert(p);
            }
        }
    }

    antinode_locations.len()
}

fn part_1(input: &str) -> usize
{
    num_antinodes(input, false)
}

fn part_2(input: &str) -> usize
{
    num_antinodes(input, true)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(8)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 14,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 354,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 34,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 1263,
        })
}
