
use rayon::prelude::*;
use std::collections::HashSet;
use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");

enum RouteOutcome
{
    Escape(HashSet<Point>),
    Loop,
}

fn test_route(grid: &CharGrid) -> RouteOutcome
{
    let mut cur_pos = grid.all_points().into_iter()
        .filter(|p| grid.get_char(p) == '^')
        .next().unwrap();
    let mut cur_dir = Point::new(0, -1);

    let mut visited_points = HashSet::new();
    let mut pos_and_dirs = HashSet::new();

    // For my input, the path is about 4000 steps long -
    // pre-reserving space here prevents copies.

    visited_points.reserve(8192);
    pos_and_dirs.reserve(8192);

    while grid.is_point_in_bounds(&cur_pos)
    {
        if !pos_and_dirs.insert((cur_pos.clone(), cur_dir.clone()))
        {
            return RouteOutcome::Loop;
        }
        visited_points.insert(cur_pos.clone());

        let next_pos = cur_pos + cur_dir;
        if grid.get_char(&next_pos) == '#'
        {
            cur_dir = cur_dir.rotate_90_left();
        }
        else
        {
            cur_pos = next_pos;
        }
    }

    RouteOutcome::Escape(visited_points)
}

fn part_1(input: &str) -> usize
{
    let grid = CharGrid::new_from_input(input, '.');

    match test_route(&grid)
    {
        RouteOutcome::Escape(visited_points) => visited_points.len(),
        RouteOutcome::Loop => unreachable!(),
    }
}

fn part_2(input: &str) -> usize
{
    let grid = CharGrid::new_from_input(input, '.');

    // Escape from the original grid, and use that
    // as the set of points to try for obstructions

    let to_try =  match test_route(&grid)
    {
        RouteOutcome::Loop => unreachable!(),
        RouteOutcome::Escape(visited_points) => visited_points,
    };

    to_try.par_iter()
        .filter(|to_try|
            {
                if grid.get_char(to_try) == '.'
                {
                    let mut modified_grid = grid.clone();
                    modified_grid.put_char(to_try, '#');
                    if let RouteOutcome::Loop = test_route(&modified_grid)
                    {
                        return true;
                    }
                }
                false
            })
        .count()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(6)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 41,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 4454,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 6,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 1503,
        })
}
