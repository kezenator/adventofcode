use crate::support::*;
use itertools::*;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

fn search_dir(trees: &CharGrid, point: &Point, dir: Point) -> (bool, usize)
{
    let point_height = trees.get_char(point);
    let mut result = 0;
    let mut cur = *point + dir;
    while trees.is_point_in_bounds(&cur)
    {
        result += 1;
        if trees.get_char(&cur) >= point_height { return (false, result); }
        cur = cur + dir;
    }
    (true, result)
}

fn search_dirs<T>(trees: &CharGrid, point: &Point, part: fn((bool, usize)) -> T, combine: fn(T, T) -> T) -> T
{
    let result = part(search_dir(trees, point, Point::new(1, 0)));
    let result = combine(result, part(search_dir(trees, point, Point::new(-1, 0))));
    let result = combine(result, part(search_dir(trees, point, Point::new(0, 1))));
    let result = combine(result, part(search_dir(trees, point, Point::new(0, -1))));
    result
}

fn is_visible_from_outside(trees: &CharGrid, point: &Point) -> bool
{
    // If any (combine = ||) escapes (part = 0)
    search_dirs(trees, point, |p| p.0, |a, b| a || b)
}

fn scenic_score(trees: &CharGrid, point: &Point) -> usize
{
    // Product (combine = *) of distances (part = 1)
    search_dirs(trees, point, |p| p.1, |a, b| a * b)
}

fn part_1(input: &str) -> usize
{
    let trees = CharGrid::new_from_input(input, '0');

    trees.all_points().iter()
        .filter(|point| is_visible_from_outside(&trees, point))
        .count()
}

fn part_2(input: &str) -> usize
{
    let trees = CharGrid::new_from_input(input, '0');

    trees.all_points().iter()
        .map(|point| scenic_score(&trees, point))
        .sorted()
        .rev()
        .next()
        .unwrap()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(8)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 21,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 1717,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 8,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 321975,
        })
}
