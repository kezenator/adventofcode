
use std::collections::HashSet;
use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");

fn parse_input(input: &str, size: usize) -> (CharGrid, Vec<Point>, Point, Point)
{
    let mut grid = CharGrid::new_from_fill(size + 1, size + 1, '.');
    grid.set_default('#');

    let bytes = input_to_lines_mapped(input,
        |l|
        {
            let (x, y) = scan(l).take_digits().parse().skip_str(",").remaining().parse();
            Point::new(x, y)
        });

    let start = Point::new(0, 0);
    let end = Point::new(size as i64, size as i64);

    (grid, bytes, start, end)
}

fn try_solve(grid: &CharGrid, start: &Point, end: &Point) -> Option<(HashSet<Point>, i64)>
{
    if let Some((path, cost)) = pathfinding::directed::astar::astar(
        start,
        |cur|
        {
            cur.neighbours_4()
                .filter(|n| grid.is_point_in_bounds(n) && grid.get_char(n) == '.')
                .map(|n| (n, 1))
        },
        |cur| (*cur - *end).manhatten_size(),
        |cur| *cur == *end)
    {
        return Some((path.into_iter().collect(), cost));
    }
    None
}

fn part_1(input: &str, size: usize, num_bytes: usize) -> i64
{
    let (mut grid, bytes, start, end) = parse_input(input, size);

    for b in bytes.into_iter().take(num_bytes)
    {
        grid.put_char(&b, '#');
    }

    try_solve(&grid, &start, &end).unwrap().1
}


fn part_2(input: &str, size: usize) -> String
{
    let (mut grid, all_points, start, end) = parse_input(input,size);
    let mut shortest_path = try_solve(&grid, &start, &end).unwrap().0;

    for byte in all_points
    {
        grid.put_char(&byte, '#');

        if !shortest_path.contains(&byte)
        {
            // Just filling in other random points that are not on our
            // current shortest path
        }
        else
        {
            // We've filled in a block on our current shortest path -
            // we need to see if there is a new shortest path...

            if let Some((new_path, _)) = try_solve(&grid, &start, &end)
            {
                shortest_path = new_path;
            }
            else
            {
                // Blocked!!!
                return format!("{},{}", byte.x, byte.y);
            }
        }
    }
    unreachable!();
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(18)
        .example(|| Answer {
            calculated: part_1(EXAMPLE, 6, 12),
            expected: 22,
        })
        .part_1(|input| Answer {
            calculated: part_1(input, 70, 1024),
            expected: 260,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE, 6),
            expected: "6,1",
        })
        .part_2(|input| Answer {
            calculated: part_2(input, 70),
            expected: "24,48",
        })
}
