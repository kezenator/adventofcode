use std::str::FromStr;
use crate::support::*;

const EXAMPLE_1: &str = "R8,U5,L5,D3\nU7,R6,D4,L4";
const EXAMPLE_2: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
const EXAMPLE_3: &str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Direction
{
    point: Point
}

impl FromStr for Direction
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let (dir, len) = scan(s)
            .take(1).parse::<char>()
            .remaining().parse::<i64>();

        let point = match dir
        {
            'U' => Point::new(0, len),
            'D' => Point::new(0, -len),
            'R' => Point::new(len, 0),
            'L' => Point::new(-len, 0),
            _ => unreachable!(),
        };

        Ok(Direction { point })
    }
}

fn parse_line(line: &str) -> Vec<Line>
{
    let (directions,) = scan(line)
        .remaining().parse_vec::<Direction>(",");

    let mut pos = Point::new(0, 0);
    let mut result = Vec::new();

    for dir in directions
    {
        let end = pos + dir.point;
        result.push(Line::new(pos, end));
        pos = end;
    }

    result
}

fn parse_lines(input: &str) -> (Vec<Line>, Vec<Line>)
{
    let vec = input_to_lines(input);

    (parse_line(&vec[0]), parse_line(&vec[1]))
}

fn part_1(input: &str) -> i64
{
    let (path1, path2) = parse_lines(input);

    let mut distances = Vec::new();

    for line1 in path1
    {
        for line2 in path2.iter()
        {
            if let Some(intersection) = line1.intersection(line2)
            {
                distances.push(intersection.manhatten_size());
            }
        }
    }

    distances.sort();

    distances[0]
}

fn part_2(input: &str) -> i64
{
    let (path1, path2) = parse_lines(input);

    let mut steps = Vec::new();

    let mut steps1 = 0;
    for line1 in path1
    {
        let mut steps2 = 0;
        for line2 in path2.iter()
        {
            if let Some(intersection) = line1.intersection(line2)
            {
                let total_steps1 = steps1 + (intersection - line1.start).manhatten_size();
                let total_steps2 = steps2 + (intersection - line2.start).manhatten_size();

                steps.push(total_steps1 + total_steps2);
            }

            steps2 += line2.manhatten_len();
        }

        steps1 += line1.manhatten_len();
    }

    steps.sort();

    steps[0]
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(3)
        .example(|| Answer { calculated: part_1(EXAMPLE_1), expected: 6, })
        .example(|| Answer { calculated: part_1(EXAMPLE_2), expected: 159, })
        .example(|| Answer { calculated: part_1(EXAMPLE_3), expected: 135, })
        .part_1(|| Answer { calculated: part_1(INPUT), expected: 651, })
        .example(|| Answer { calculated: part_2(EXAMPLE_1), expected: 30, })
        .example(|| Answer { calculated: part_2(EXAMPLE_2), expected: 610, })
        .example(|| Answer { calculated: part_2(EXAMPLE_3), expected: 410, })
        .part_2(|| Answer { calculated: part_2(INPUT), expected: 7534, })
}
