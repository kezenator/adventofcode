
use crate::support::*;
use std::collections::HashSet;

const EXAMPLE: &str = include_str!("example.txt");

fn fencing_price<F>(input: &str, linear_measurement: F) -> usize
    where F: Fn(&GridArea) -> usize
{
    let grid = CharGrid::new_from_input(input, '.');
    let mut remaining_points = grid.all_points().into_iter().collect::<HashSet<Point>>();

    let mut result = 0;

    while let Some(next_point) = remaining_points.iter().next()
    {
        let grid_area = grid.find_flood_fill_points(next_point);

        remaining_points = remaining_points.into_iter()
            .filter(|p| !grid_area.points.contains(p))
            .collect();

        let area = grid_area.area();
        let linear = linear_measurement(&grid_area);

        //println!("{} => {} x {}", field_char, area, linear);

        result += area * linear;
    }

    result
}

fn part_1(input: &str) -> usize
{
    fencing_price(input, |ga| ga.perimeter())
}

fn part_2(input: &str) -> usize
{
    fencing_price(input, |ga| ga.num_edges())
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(12)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 1930,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 1452678,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 1206,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 873584,
        })
}
