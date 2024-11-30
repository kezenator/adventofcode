use crate::support::*;
use itertools::*;
use std::collections::{HashMap, HashSet};
use pathfinding::directed::astar::*;

const EXAMPLE: &str = include_str!("example.txt");

struct Input
{
    blizzards: Vec<(Point, Point)>,
}

fn parse(input: &str) -> Input
{
    let grid = CharGrid::new_from_input(input, '.');

    let mut blizzards = Vec::new();

    for p in grid.all_points()
    {
        match grid.get_char(&p)
        {
            '>' => { blizzards.push((p, Point::new(1, 0))); },
            'v' => { blizzards.push((p, Point::new(0, 1))); },
            '<' => { blizzards.push((p, Point::new(-1, 0))); },
            '^' => { blizzards.push((p, Point::new(0, -1))); },
            _ => {},
        }
    }

    Input { blizzards }
}

fn start_end(input: &str) -> (Point, Point)
{
    let grid = CharGrid::new_from_input(input, '#');
    let width = grid.get_width();
    let height = grid.get_height();
    let start = Point::new(1, 0);
    let end = Point::new(width - 2, height - 1);

    (start, end)
}

fn shortest_time(input: &str, from: &Point, to: &Point, start_minute: i64) -> i64
{
    let grid = CharGrid::new_from_input(input, '#');
    let width = grid.get_width();
    let height = grid.get_height();
    let input = parse(input);
    let mut bpoints = HashMap::new();

    let points_at_minutes = move |step: i64| -> HashSet<Point>
    {
        let mut result = HashSet::new();
        for (b_start, b_dir) in input.blizzards.iter()
        {
            let (mut x, mut y) = (b_start.x - 1, b_start.y - 1);
            x += step * b_dir.x;
            y += step * b_dir.y;
            x = x % (width - 2);
            y = y % (height - 2);
            if x < 0 { x += width - 2; }
            if y < 0 { y += height - 2; }
            result.insert(Point::new(x + 1, y + 1));
        }
        result
    };

    let search_result = astar(
        &(*from, start_minute),
        move |(p, minutes)|
        {
            if !bpoints.contains_key(&(minutes + 1))
            {
                bpoints.insert(minutes + 1, points_at_minutes(minutes + 1));
            }
            let cur_bliz = bpoints.get(&(minutes + 1)).unwrap();
            let mut next_possible_positions = p.neighbours_4().collect_vec();
            next_possible_positions.push(p.clone());
            next_possible_positions.into_iter()
                .filter(|n| !cur_bliz.contains(n) && (grid.get_char(n) != '#'))
                .map(|n| ((n, minutes + 1), 1))
                .collect_vec()
        },
        |(p, _)|
        {
            (*to - *p).manhatten_size()
        },
        |(p, _)|
        {
            *p == *to
        });

    search_result.unwrap().1
}

fn part_1(input: &str) -> i64
{
    let (start, end) = start_end(input);
    shortest_time(input, &start, &end, 0)
}

fn part_2(input: &str) -> i64
{
    let (start, end) = start_end(input);
    let there = shortest_time(input, &start, &end, 0);
    let back = shortest_time(input, &end, &start, there);
    let there_again = shortest_time(input, &start, &end, there + back);

    there + back + there_again
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(24)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 18,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 305,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 54,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 905,
        })
}
