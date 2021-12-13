use crate::support::*;
use std::collections::HashSet;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

// A square box
const PART_2_ANSWER_EXAMPLE: &str = "#####\n#...#\n#...#\n#...#\n#####";
// "HKUJGAJZ"
const PART_2_ANSWER_INPUT: &str = "#..#.#..#.#..#...##..##...##....##.####\n#..#.#.#..#..#....#.#..#.#..#....#....#\n####.##...#..#....#.#....#..#....#...#.\n#..#.#.#..#..#....#.#.##.####....#..#..\n#..#.#.#..#..#.#..#.#..#.#..#.#..#.#...\n#..#.#..#..##...##...###.#..#..##..####";

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
            Fold::X(num) => if p.x > *num { Point::new(2 * num - p.x, p.y) } else { p.clone() },
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

    let minx = set.iter().map(|p| p.x).min().unwrap();
    let miny = set.iter().map(|p| p.y).min().unwrap();
    let width = set.iter().map(|p| p.x).max().unwrap() + 1 - minx;
    let height = set.iter().map(|p| p.y).max().unwrap() + 1 - miny;

    let mut grid = CharGrid::new_from_fill(width as usize, height as usize, '.');

    for p in set
    {
        grid.put_char(&Point::new(p.x - minx, p.y - miny), '#');
    }

    grid.to_string()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(13)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 17,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 621,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: PART_2_ANSWER_EXAMPLE,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: PART_2_ANSWER_INPUT,
        })
}
