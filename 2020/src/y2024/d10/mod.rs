
use itertools::*;
use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");

fn trailheads(grid: &CharGrid) -> Vec<Point>
{
    grid.all_points().into_iter()
        .filter(|p| grid.get_char(p) == '0')
        .collect()
}

fn num_summits_reached(grid: &CharGrid, trailhead: Point) -> usize
{
    let (solutions, _cost) = pathfinding::directed::astar::astar_bag(
        &trailhead,
        |&p|
        {
            let ch_at_p = grid.get_char(&p);
            let next_step = ((ch_at_p as usize) + 1) as u8 as char;
            p.neighbours_4()
                .filter(move |n| grid.get_char(n) == next_step)
                .map(|p| (p, 1))
        },
        |p| ('9' as usize) - (grid.get_char(p) as usize),
        |p| grid.get_char(p) == '9').unwrap();

    solutions
        .into_iter()
        .map(|path| path.last().cloned().unwrap())
        .unique()
        .count()
}

fn num_paths_to_summits(grid: &CharGrid, trailhead: Point) -> usize
{
    let (solutions, _cost) = pathfinding::directed::astar::astar_bag(
        &trailhead,
        |&p|
        {
            let ch_at_p = grid.get_char(&p);
            let next_step = ((ch_at_p as usize) + 1) as u8 as char;
            p.neighbours_4()
                .filter(move |n| grid.get_char(n) == next_step)
                .map(|p| (p, 1))
        },
        |p| ('9' as usize) - (grid.get_char(p) as usize),
        |p| grid.get_char(p) == '9').unwrap();

    solutions
        .into_iter()
        .count()
}

fn part_1(input: &str) -> usize
{
    let grid = CharGrid::new_from_input(input, '.');

    trailheads(&grid)
        .into_iter()
        .map(|p| num_summits_reached(&grid, p))
        .sum()
}

fn part_2(input: &str) -> usize
{
    let grid = CharGrid::new_from_input(input, '.');

    trailheads(&grid)
        .into_iter()
        .map(|p| num_paths_to_summits(&grid, p))
        .sum()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(10)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 36,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 638,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 81,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 1289,
        })
}
