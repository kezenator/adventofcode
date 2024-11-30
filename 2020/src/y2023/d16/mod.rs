use crate::support::*;
use std::collections::HashSet;

const EXAMPLE: &str = include_str!("example.txt");

fn split(beam: (Point, Point), splitter: char) -> Vec<(Point, Point)>
{
    let splitter_vertical = splitter == '|';
    let beam_vertical = beam.1.x == 0;

    if splitter_vertical == beam_vertical
    {
        // Pass straight through
        vec![beam]
    }
    else // split
    {
        vec![
            (beam.0, beam.1.rotate_90_left()),
            (beam.0, beam.1.rotate_90_right()),
        ]
    }
}

fn reflect(beam: (Point, Point), mirror: char) -> (Point, Point)
{
    if mirror == '/'
    {
        if beam.1.x != 0
        {
            (beam.0, beam.1.rotate_90_right())
        }
        else
        {
            (beam.0, beam.1.rotate_90_left())
        }
    }
    else // '\\'
    {
        if beam.1.x != 0
        {
            (beam.0, beam.1.rotate_90_left())
        }
        else
        {
            (beam.0, beam.1.rotate_90_right())
        }
    }
}

fn energized_count(grid: &CharGrid, beam_start_pos: Point, beam_start_dir: Point) -> usize
{
    let mut result = CharGrid::new_from_fill(grid.get_width() as usize, grid.get_height() as usize, '.');
    let mut already_done = HashSet::<(Point, Point)>::new();

    let mut beams = vec![(beam_start_pos, beam_start_dir)];

    while !beams.is_empty()
    {
        let mut new_beams = Vec::new();
        for beam in beams.into_iter()
        {
            // Move the beam
            let beam = (beam.0 + beam.1, beam.1);
            if grid.is_point_in_bounds(&beam.0)
                && already_done.insert(beam.clone())
            {
                // It's still on the grid and we've not already
                // processed this beam into a loop

                // Mark the spot as energized
                result.put_char(&beam.0, '#');

                // Work out how the beam moves
                let ch = grid.get_char(&beam.0);
                match ch
                {
                    '.' =>
                    {
                        // Just move along
                        new_beams.push(beam);
                    },
                    '-' | '|' =>
                    {
                        new_beams.append(&mut split(beam, ch));
                    },
                    '/' | '\\' =>
                    {
                        new_beams.push(reflect(beam, ch));
                    },
                    _ => unreachable!(),
                }
            }
        }
        beams = new_beams;
    }
    
    result.all_chars().into_iter().filter(|ch| *ch == '#').count()
}

fn part_1(input: &str) -> usize
{
    let grid = CharGrid::new_from_input(input, '.');
    energized_count(&grid, Point::new(-1, 0), Point::new(1, 0))
}

fn part_2(input: &str) -> usize
{
    let grid = CharGrid::new_from_input(input, '.');
    let mut all_counts = vec![];

    let width = grid.get_width();
    let height = grid.get_height();

    for x in 0..width
    {
        all_counts.push(energized_count(&grid, Point::new(x, -1), Point::new(0, 1)));
        all_counts.push(energized_count(&grid, Point::new(x, height), Point::new(0, -1)));
    }

    for y in 0..height
    {
        all_counts.push(energized_count(&grid, Point::new(-1, y), Point::new(1, 0)));
        all_counts.push(energized_count(&grid, Point::new(width, y), Point::new(-1, 0)));
    }

    all_counts.into_iter().max().unwrap()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(16)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 46,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 7623,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 51,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 8244,
        })
}
