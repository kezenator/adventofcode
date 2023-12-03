use crate::support::*;
use itertools::*;
use std::collections::HashSet;
use pathfinding::directed::bfs::*;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

fn range<F>(points: &HashSet<Point3>, func: F) -> (i64, i64)
    where F: Fn(&Point3) -> i64
{
    match points.iter().map(func).minmax()
    {
        MinMaxResult::NoElements => unreachable!(),
        MinMaxResult::OneElement(_) => unreachable!(),
        MinMaxResult::MinMax(min, max) => (min, max),
    }
}

fn is_inside(point_to_test: &Point3, points: &HashSet<Point3>, p_min: &Point3, p_max: &Point3) -> bool
{
    if points.contains(point_to_test)
    {
        // Part of the lava set - it's not inside
        return false;
    }

    let path_to_outside = bfs(
        point_to_test,
        |p|
        {
            p.neighbours_6()
                .filter(|n| !points.contains(n))
        },
        |p|
        {
            p.x < p_min.x || p.x > p_max.x
                || p.y < p_min.y || p.y > p_max.y
                || p.z < p_min.z || p.z > p_max.z
        });

    path_to_outside.is_none()
}

fn surface_area(input: &str, count_inside: bool) -> usize
{
    let points = input_to_lines(input)
        .into_iter()
        .map(|l| l.split(",").map(|p| p.parse::<i64>().unwrap()).collect_vec())
        .map(|ps| Point3::new(ps[0], ps[1], ps[2]))
        .collect::<HashSet<_>>();

    let mut lava_or_inside_points = points.clone();

    if (!count_inside)
    {
        // For all points - work out if they are inside
        // and if so - also include them in the "not outside" points

        let x_range = range(&points, |p| p.x);
        let y_range = range(&points, |p| p.y);
        let z_range = range(&points, |p| p.z);

        let p_min = Point3::new(x_range.0, y_range.0, z_range.0);
        let p_max = Point3::new(x_range.1, y_range.1, z_range.1);

        for x in x_range.0..=x_range.1
        {
            for y in y_range.0..=y_range.1
            {
                for z in z_range.0..=z_range.1
                {
                    let p = Point3::new(x, y, z);

                    if is_inside(&p, &points, &p_min, &p_max)
                    {
                        lava_or_inside_points.insert(p);
                    }
                }
            }
        }
    }

    points
        .iter()
        .map(|p|
        {
            p.neighbours_6()
                .filter(|n| !lava_or_inside_points.contains(n))
                .count()
        })
        .sum::<usize>()
}

fn part_1(input: &str) -> usize
{
    surface_area(input, true)
}

fn part_2(input: &str) -> usize
{
    surface_area(input, false)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(18)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 64,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 3530,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 58,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 2000,
        })
}
