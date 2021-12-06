use crate::support::*;
use std::collections::{HashMap, HashSet};
use std::cmp::{min, max};

const INPUT: &str = include_str!("input.txt");
const EXAMPLE1: &str = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2\n";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point
{
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Rectangle
{
    location: Point,
    size: Point,
}

impl Rectangle
{
    fn overlap(&self, other: &Rectangle) -> Option<Rectangle>
    {
        let min_x = max(self.location.x, other.location.x);
        let min_y = max(self.location.y, other.location.y);

        let max_x = min(self.location.x + self.size.x, other.location.x + other.size.x);
        let max_y = min(self.location.y + self.size.y, other.location.y + other.size.y);

        if (min_x < max_x) && (min_y < max_y)
        {
            return Some(Rectangle{
                location: Point{ x: min_x, y: min_y },
                size: Point{ x: max_x - min_x, y: max_y - min_y },
            })
        }
        return None;
    }

    fn points(&self) -> HashSet<Point>
    {
        let mut result = HashSet::new();
        for x in 0..self.size.x
        {
            for y in 0..self.size.y
            {
                result.insert(Point{x: self.location.x + x, y: self.location.y + y});
            }
        }
        return result;
    }
}

fn parse_point(input: &str, split: &str) -> Point
{
    let parts = input.split(split).collect::<Vec<_>>();

    let x = parts[0].parse::<u32>().unwrap();
    let y = parts[1].parse::<u32>().unwrap();

    Point{x, y}
}

fn load(input: &str) -> HashMap<u32, Rectangle>
{
    let mut result = HashMap::new();

    for line in input.lines().filter(|x| !x.is_empty())
    {
        let parts = line.split(" ").collect::<Vec<_>>();

        let id = parts[0][1..].parse::<u32>().unwrap();
        let location = parse_point(&parts[2][..(parts[2].len()-1)], ",");
        let size = parse_point(&parts[3], "x");

        result.insert(id, Rectangle{location, size});
    }

    return result;
}

fn part_1(input: &str) -> usize
{
    let rects = load(input);
    let mut overlapping = HashSet::new();

    for (ida, recta) in rects.iter()
    {
        for (idb, rectb) in rects.iter()
        {
            if ida != idb
            {
                if let Some(overlap) = recta.overlap(&rectb)
                {
                    for point in overlap.points()
                    {
                        overlapping.insert(point);
                    }
                }
            }
        }
    }
    
    return overlapping.len();
}

fn part_2(input: &str) -> u32
{
    let rects = load(input);

    for (ida, recta) in rects.iter()
    {
        let mut overlapped = false;

        for (idb, rectb) in rects.iter()
        {
            if ida != idb
            {
                if recta.overlap(&rectb).is_some()
                {
                    overlapped = true;
                    break;
                }
            }
        }

        if !overlapped
        {
            return *ida;
        }
    }

    assert!(false);
    unreachable!();
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(3)
        .example(|| Answer {
            calculated: part_1(EXAMPLE1),
            expected: 4, })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 104712, })
        .example(|| Answer {
            calculated: part_2(EXAMPLE1),
            expected: 3, })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 840, })
}