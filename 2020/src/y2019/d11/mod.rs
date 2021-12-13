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
                    dir = dir.rotate_90_right();
                }
                else
                {
                    dir = dir.rotate_90_left();
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

    CharGrid::new_from_points(points).to_string()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(11)
        .part_1(|| Answer { calculated: part_1(), expected: 2184, })
        .part_2(|| Answer { calculated: part_2(), expected: PART_2_ANSWER, })
}
