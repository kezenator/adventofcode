use crate::support::*;
use std::collections::HashMap;

const EXAMPLE: &str = include_str!("example.txt");

fn solve(input: &str, diagonals: bool) -> usize
{
    let lines = input_to_lines(input);

    let mut point_to_count: HashMap<Point, usize> = HashMap::new();

    for line in lines.iter()
    {
        let (x1, y1, x2, y2) = scan(line)
            .until(",").parse::<i64>()
            .until(" -> ").parse::<i64>()
            .until(",").parse::<i64>()
            .remaining().parse::<i64>();

        let line = Line::new(Point::new(x1, y1), Point::new(x2, y2));

        // Always consider horizontal / vertical lines.
        // Only consider diagonals if requested

        if (x1 == x2) || (y1 == y2) || diagonals
        {
            for point in line.points_exactly_on_line_inclusive()
            {
                if let Some(count) = point_to_count.get_mut(&point)
                {
                    *count += 1;
                }
                else
                {
                    point_to_count.insert(point, 1);
                }
            }
        }
    }

    point_to_count.iter()
        .filter(|entry| *entry.1 >= 2)
        .count()
}

fn part_1(input: &str) -> usize
{
    solve(input, false)
}

fn part_2(input: &str) -> usize
{
    solve(input, true)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(5)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 5,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 6007,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 12,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 19349,
        })
}
