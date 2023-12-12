use std::collections::HashSet;
use crate::support::*;
use itertools::*;

const EXAMPLE1: &str = include_str!("example1.txt");
const EXAMPLE2: &str = include_str!("example2.txt");
const INPUT: &str = include_str!("input.txt");

fn junctions_to_char(ch: char) -> Vec<Point>
{
    match ch
    {
        '|' => vec![Point::new(0, -1), Point::new(0, 1)],
        '-' => vec![Point::new(-1, 0), Point::new(1, 0)],
        'L' => vec![Point::new(0, -1), Point::new(1, 0)],
        'J' => vec![Point::new(0, -1), Point::new(-1, 0)],
        '7' => vec![Point::new(0, 1), Point::new(-1, 0)],
        'F' => vec![Point::new(0, 1), Point::new(1, 0)],
        'S' => vec![Point::new(0, 1), Point::new(1, 0), Point::new(0, -1), Point::new(-1, 0)],
        '.' => Vec::new(),
        _ => unreachable!(),
    }
}

fn find_path(map: &CharGrid) -> Vec<Point>
{
    let starting_point = map.all_points().into_iter()
        .filter(|p| map.get_char(p) == 'S')
        .next().unwrap();

    let mut already_visited = HashSet::new();
    already_visited.insert(starting_point.clone());

    let mut result = Vec::new();
    result.push(starting_point.clone());

    let mut cur = starting_point.clone();

    'main_loop: loop
    {
        let cur_ch = map.get_char(&cur);
        let cur_dirs = junctions_to_char(cur_ch);

        for cur_dir in cur_dirs
        {
            let next_pos = cur + cur_dir;

            if !already_visited.contains(&next_pos)
            {
                let next_ch = map.get_char(&next_pos);
                let next_dirs = junctions_to_char(next_ch);

                for d in next_dirs
                {
                    if (d.x == -cur_dir.x) && (d.y == -cur_dir.y)
                    {
                        // These pipes connect
                        result.push(next_pos.clone());
                        already_visited.insert(next_pos.clone());
                        cur = next_pos;
                        continue 'main_loop;
                    }
                }
            }
        }

        // Found the complete path
        return result;
    }
}

fn count_crossings<FExtract: Fn(&Point) -> i64, FBuild: Fn(i64) -> Point>(map: &CharGrid, path_points: &HashSet<Point>, point: &Point, extract: FExtract, build: FBuild) -> i64
{
    let mut inc_count = 0;
    let mut dec_count = 0;
    let mut sum = 0;

    for i in 0..extract(&point)
    {
        let p = build(i);

        if path_points.contains(&p)
        {
            let ch = map.get_char(&p);
            let junction_dirs = junctions_to_char(ch);

            for dir in junction_dirs
            {
                let dir = extract(&Point::new(dir.y, dir.x));
                sum += dir;
                if dir > 0 { inc_count += 1; }
                if dir < 0 { dec_count += 1; }
            }
        }
    }

    inc_count.min(dec_count)
}

fn part_1(input: &str) -> usize
{
    let map = CharGrid::new_from_input(input, '.');
    find_path(&map).len() / 2
}

fn part_2(input: &str) -> usize
{
    let map = CharGrid::new_from_input(input, '.');
    let path = find_path(&map);
    let starting_point = path[0].clone();
    let path_points = path.into_iter().collect::<HashSet<_>>();

    // Flood fill a map with all points "outside"

    let mut inside_map = CharGrid::new_from_fill(map.get_width() as usize, map.get_height() as usize, 'O');

    // Now - process each point

    for point in inside_map.all_points()
    {
        if path_points.contains(&point)
        {
            inside_map.put_char(&point, map.get_char(&point));
        }
        else
        {
            // To find points inside:
            // Send a 'ray' to infinity - points inside
            // will 'cross' the path an odd number of times.
            // Because the starting point doesn't have
            // a known piece of pipe - we need to test
            // in both directions to ensure we don't pass
            // through the starting point

            let crossings = if point.y != starting_point.y
            {
                count_crossings(&map, &path_points, &point, |p| p.x, |x| Point::new(x, point.y))
            }
            else
            {
                count_crossings(&map, &path_points, &point, |p| p.y, |y| Point::new(point.x, y))
            };

            if (crossings % 2) == 1
            {
                inside_map.put_char(&point, 'I')
            }
        }
    }

    //println!("{}", inside_map.to_string());
    
    inside_map.all_chars()
        .into_iter()
        .filter(|ch| *ch == 'I')
        .count()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(10)
        .example(|| Answer {
            calculated: part_1(EXAMPLE1),
            expected: 8,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 7102,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE2),
            expected: 10,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 363,
        })
}
