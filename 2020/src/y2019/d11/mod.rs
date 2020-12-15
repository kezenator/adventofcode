use std::collections::HashMap;
use crate::support::*;
use crate::y2019::intcode::*;

const INPUT: &str = include_str!("input.txt");

// .##..#..#..##..#..#.####.####.###..#..#
// #..#.#..#.#..#.#..#....#.#....#..#.#.#.
// #..#.####.#....####...#..###..#..#.##..
// ####.#..#.#....#..#..#...#....###..#.#.
// #..#.#..#.#..#.#..#.#....#....#....#.#.
// #..#.#..#..##..#..#.####.####.#....#..#
//
// i.e. AHCHZEPK

const PART_2_ANSWER: &str = ".##..#..#..##..#..#.####.####.###..#..#\n#..#.#..#.#..#.#..#....#.#....#..#.#.#.\n#..#.####.#....####...#..###..#..#.##..\n####.#..#.#....#..#..#...#....###..#.#.\n#..#.#..#.#..#.#..#.#....#....#....#.#.\n#..#.#..#..##..#..#.####.####.#....#..#";


fn run(start_on_white: bool) -> HashMap::<Point, bool>
{
    let mut comp = Intcode::new_from_input(INPUT);
    let mut points: HashMap::<Point, bool> = HashMap::new();

    let mut pos = Point::new(0, 0);
    let mut dir = Point::new(0, 1);

    if start_on_white
    {
        points.insert(pos.clone(), true);
    }

    loop
    {
        if *points.get(&pos).unwrap_or(&false)
        {
            comp.input(1);
        }
        else
        {
            comp.input(0);
        }

        match comp.run_until_halt_or_input_required()
        {
            IntcodePause::Halted =>
            {
                return points;
            },
            IntcodePause::MoreInputRequired =>
            {
                assert_eq!(comp.output_len(), 2);

                let col = comp.pop_output() != 0;
                let right = comp.pop_output() != 0;

                points.insert(pos.clone(), col);

                if right
                {
                    dir = dir.rotate_right();
                }
                else
                {
                    dir = dir.rotate_left();
                }

                pos += dir;
            },
        }
    }
}

fn part_1() -> usize
{
    run(false).len()
}

fn part_2() -> String
{
    let points = run(true).iter()
        .filter(|(_, &white)| white)
        .map(|(pos, _)| Point::new(pos.x, -pos.y))
        .collect::<Vec<Point>>();

    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();

    let mut image = CharGrid::new_from_fill((max_x - min_x + 1) as usize, (max_y - min_y + 1) as usize, '.');

    for p in points
    {
        image.put_char(
            &Point::new(p.x - min_x, p.y - min_y),
            '#');
    }

    image.to_string()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(11)
        .part_1(|| Answer { calculated: part_1(), expected: 2184, })
        .part_2(|| Answer { calculated: part_2(), expected: PART_2_ANSWER, })
}
