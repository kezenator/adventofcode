use crate::support::*;
use std::collections::HashSet;

const EXAMPLE: &str = include_str!("example.txt");

// A square box
const PART_2_ANSWER_FOR_EXAMPLE: &str = "#####\n#...#\n#...#\n#...#\n#####";
// "HKUJGAJZ"
const PART_2_ANSWER_FOR_INPUT: &str = "#..#.#..#.#..#...##..##...##....##.####\n#..#.#.#..#..#....#.#..#.#..#....#....#\n####.##...#..#....#.#....#..#....#...#.\n#..#.#.#..#..#....#.#.##.####....#..#..\n#..#.#.#..#..#.#..#.#..#.#..#.#..#.#...\n#..#.#..#..##...##...###.#..#..##..####";

enum Fold
{
    X(i64),
    Y(i64),
}

impl Fold
{
    fn apply(&self, p: Point) -> Point
    {
        match self
        {
            // Fold right
            Fold::X(num) => if p.x > *num { Point::new(2 * num - p.x, p.y) } else { p.clone() },
            // Fold up - but remember "up" means "down" in our coordinates
            Fold::Y(num) => if p.y > *num { Point::new(p.x, 2 * num - p.y) } else { p.clone() },
        }
    }
}

struct Input
{
    points: Vec<Point>,
    folds: Vec<Fold>,
}

fn parse(input: &str) -> Input
{
    let lines = input_to_lines(input);

    let empty_index = lines.iter().position(|l| l.is_empty()).unwrap();

    let points = lines.iter()
        .take(empty_index)
        .map(|l|
        {
            let (x, y) = scan(l).until(",").parse::<i64>().remaining().parse::<i64>();
            Point::new(x, y)
        })
        .collect();

    let folds = lines.iter()
        .skip(empty_index + 1)
        .map(|l|
        {
            let (ch, num) = scan(l).skip_str("fold along ").until("=").parse::<char>().remaining().parse::<i64>();

            match ch
            {
                'x' => Fold::X(num),
                'y' => Fold::Y(num),
                _ => unreachable!(),
            }
        })
        .collect();

    Input { points, folds }
}

fn part_1(input: &str) -> usize
{
    let input = parse(input);

    let mut set = HashSet::new();

    for p in input.points
    {
        set.insert(input.folds[0].apply(p));
    }

    set.len()
}

fn part_2(input: &str) -> String
{
    let input = parse(input);

    let mut set = HashSet::new();

    for p in input.points
    {
        set.insert(
            input.folds.iter()
                .fold(p, |p, f| f.apply(p)));
    }

    CharGrid::new_from_points(set.drain().collect()).to_string()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(13)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 17,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 621,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: PART_2_ANSWER_FOR_EXAMPLE,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: PART_2_ANSWER_FOR_INPUT,
        })
}
