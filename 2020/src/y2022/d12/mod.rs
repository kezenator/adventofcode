use crate::support::*;
use itertools::*;
use pathfinding::directed::astar::*;

const EXAMPLE: &str = include_str!("example.txt");

fn height(ch: char) -> i64
{
    match ch
    {
        '.' => -100,
        'S' => 0,
        'E' => 26,
        _ => (ch as i64) - ('a' as i64),
    }
}

fn start(area: &CharGrid) -> Point
{
    area.all_points().iter()
        .filter(|p| area.get_char(p) == 'S')
        .next().unwrap().clone()
}

fn end(area: &CharGrid) -> Point
{
    area.all_points().iter()
        .filter(|p| area.get_char(p) == 'E')
        .next().unwrap().clone()
}

fn steps_to<F>(area: &CharGrid, success: F) -> usize
    where F: Fn(&Point) -> bool
{
    let end_point = end(&area);

    let search_result = astar(
        &end_point,
        |p|
        {
            let p_height = height(area.get_char(p));
            p.neighbours_4()
                .filter(|n| height(area.get_char(n)) >= (p_height - 1))
                .map(|n| (n, 1))
                .collect_vec()
        },
        |p|
        {
            height(area.get_char(p))
        },
        success);

    return search_result.unwrap().0.len() - 1;
}

fn part_1(input: &str) -> usize
{
    let area = CharGrid::new_from_input(input, '.');
    let start_point = start(&area);

    steps_to(&area, |p| *p == start_point)
}

fn part_2(input: &str) -> usize
{
    let area = CharGrid::new_from_input(input, '.');

    steps_to(&area, |p| height(area.get_char(p)) == 0)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(12)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 31,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 408,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 29,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 399,
        })
}
