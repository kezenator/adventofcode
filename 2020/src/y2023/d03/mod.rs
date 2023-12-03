use std::collections::HashMap;
use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

struct Schematic
{
    part_numbers: Vec<u64>,
    gears: HashMap<Point, Vec<u64>>,
}

fn parse(input: &str) -> Schematic
{
    let grid = CharGrid::new_from_input(input, '.');
    let mut part_numbers = Vec::new();
    let mut gears: HashMap<Point, Vec<u64>> = HashMap::new();

    for y in 0..grid.get_height()
    {
        let mut x = 0;
        while x < grid.get_width()
        {
            let ch = grid.get_char(&Point::new(x, y));

            if ch.is_digit(10)
            {
                let mut len = 1;
                while grid.get_char(&Point::new(x + len, y)).is_digit(10)
                {
                    len += 1;
                }

                let mut points = Vec::new();
                points.push(Point::new(x - 1, y - 1));
                points.push(Point::new(x - 1, y));
                points.push(Point::new(x - 1, y + 1));
                points.push(Point::new(x + len, y - 1));
                points.push(Point::new(x + len, y));
                points.push(Point::new(x + len, y + 1));

                let mut str = String::new();

                for i in 0..len
                {
                    points.push(Point::new(x + i, y - 1));
                    points.push(Point::new(x + i, y + 1));
                    str.push(grid.get_char(&Point::new(x + i, y)));
                }

                let is_part_number = points.iter()
                    .map(|p| grid.get_char(p))
                    .any(|ch| !ch.is_digit(10) && ch != '.');

                if is_part_number
                {
                    let part_number = str.parse().unwrap();

                    part_numbers.push(part_number);

                    for possible_gear in points.iter().filter(|p| grid.get_char(p) == '*')
                    {
                        gears.entry(*possible_gear).or_default().push(part_number);
                    }
                }

                x += len;
            }
            else
            {
                x += 1;
            }            
        }
    }

    let gears = gears
        .into_iter()
        .filter(|e| e.1.len() == 2)
        .collect();

    Schematic { part_numbers, gears }
}

fn part_1(input: &str) -> u64
{
    let schematic = parse(input);
    schematic.part_numbers.iter().sum()
}

fn part_2(input: &str) -> u64
{
    let schematic = parse(input);
    schematic.gears.iter()
        .map(|g| g.1.iter().product::<u64>())
        .sum()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(3)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 4361,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 535235,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 467835,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 79844424,
        })
}
