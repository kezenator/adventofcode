use crate::support::*;
use std::collections::HashSet;

const EXAMPLE1: &str = include_str!("example_1.txt");
const EXAMPLE2: &str = include_str!("example_2.txt");

fn solve_maze(input: &str) -> (HashSet<Point>, i64)
{
    let grid = CharGrid::new_from_input(input, '#');
    let start = grid.all_points().into_iter()
        .filter(|p| grid.get_char(p) == 'S')
        .next().unwrap();
    let end = grid.all_points().into_iter()
        .filter(|p| grid.get_char(p) == 'E')
        .next().unwrap();

    let (paths, cost) = pathfinding::directed::astar::astar_bag_collect(
        &(start, Point::new(1, 0)),
        |(cur, dir)|
        {
            let mut successors = Vec::new();
            let forward = *cur + *dir;
            if grid.get_char(&forward) != '#'
            {
                successors.push(((forward, *dir), 1));
            }
            successors.push(((*cur, dir.rotate_90_left()), 1000));
            successors.push(((*cur, dir.rotate_90_right()), 1000));
            successors
        },
        |(cur, _dir)|
        {
            (*cur - end).manhatten_size()
        },
        |(cur, _dir)| *cur == end).unwrap();
    
    let points = paths.into_iter()
        .flatten()
        .map(|(cur, _dir)| cur)
        .collect();

    (points, cost)
}

fn part_1(input: &str) -> i64
{
    let (_points, cost) = solve_maze(input);
    cost
}

fn part_2(input: &str) -> usize
{
    let (points, _cost) = solve_maze(input);
    points.len()
}

pub fn puzzles() -> PuzzleDay {
    puzzle_day(16)
        .example(|| Answer {
            calculated: part_1(EXAMPLE1),
            expected: 7036,
        })
        .example(|| Answer {
            calculated: part_1(EXAMPLE2),
            expected: 11048,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 101492,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE1),
            expected: 45,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE2),
            expected: 64,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 543,
        })
}
