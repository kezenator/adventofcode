use crate::support::*;
use itertools::*;
use std::collections::HashSet;

const EXAMPLE1: &str = include_str!("example1.txt");
const EXAMPLE2: &str = include_str!("example2.txt");

type Rope = Vec<Point>;

fn follow_dir(new_head: Point, old_tail: Point) -> Point
{
    let offset = new_head - old_tail;

    if offset.x.abs() <= 1 && offset.y.abs() <= 1
    {
        Point::new(0, 0)
    }
    else
    {
        Point::new(offset.x.signum(), offset.y.signum())
    }
}

fn step(rope: &mut Rope, dir: Point)
{
    let mut dir = dir;

    for i in 0..(rope.len() - 1)
    {
        rope[i] += dir;
        dir = follow_dir(rope[i], rope[i+1]);
    }
    *rope.last_mut().unwrap() += dir;
}

fn to_steps(input: &str) -> Vec<Point>
{
    let parts = input.split(' ').collect_vec();
    assert!(parts.len() == 2);

    let dist = parts[1].parse::<usize>().unwrap();
    let dir = match parts[0].chars().next().unwrap()
    {
        'U' => Point::new(0, -1),
        'D' => Point::new(0, 1),
        'L' => Point::new(-1, 0),
        'R' => Point::new(1, 0),
        _ => unreachable!(),
    };

    let mut result = Vec::new();
    result.resize(dist, dir);
    result
}

fn num_points_visited(input: &str, num_knots: usize) -> usize
{
    let mut rope = Vec::new();
    rope.resize(num_knots, Point::new(0, 0));

    let mut visited = HashSet::new();
    visited.insert(*rope.last().unwrap());

    for dir in input_to_lines(input).iter().map(|l| to_steps(l)).flatten()
    {
        step(&mut rope, dir);
        visited.insert(*rope.last().unwrap());
    }
    visited.len()
}

fn part_1(input: &str) -> usize
{
    num_points_visited(input, 2)
}

fn part_2(input: &str) -> usize
{
    num_points_visited(input, 10)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(9)
        .example(|| Answer {
            calculated: part_1(EXAMPLE1),
            expected: 13,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 5883,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE1),
            expected: 1,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE2),
            expected: 36,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 2367,
        })
}
