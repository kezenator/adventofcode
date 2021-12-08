use crate::support::*;

const EXAMPLE: &str = "F10\nN3\nF7\nR90\nF11";
const INPUT: &str = include_str!("input.txt");

fn part_1(input: &str) -> i64
{
    let mut dir = Point::new(1, 0);
    let mut pos = Point::new(0, 0);

    for line in input_to_lines(input)
    {
        let (code, dist) = scan(&line)
            .take(1).parse::<char>()
            .remaining().parse::<i64>();

        match code
        {
            'N' =>
            {
                pos += Point::new(0, dist);
            },
            'S' =>
            {
                pos += Point::new(0, -dist);
            },
            'E' =>
            {
                pos += Point::new(dist, 0);
            },
            'W' =>
            {
                pos += Point::new(-dist, 0);
            },
            'L' =>
            {
                match dist
                {
                    90 => dir = dir.rotate_90_left(),
                    180 => dir = dir.rotate_180(),
                    270 => dir = dir.rotate_90_right(),
                    _ => unreachable!(),
                }
            },
            'R' =>
            {
                match dist
                {
                    90 => dir = dir.rotate_90_right(),
                    180 => dir = dir.rotate_180(),
                    270 => dir = dir.rotate_90_left(),
                    _ => unreachable!(),
                }
            },
            'F' =>
            {
                pos += dist * dir;
            },
            _ => unreachable!(),
        }
    }
    
    pos.manhatten_size()
}

fn part_2(input: &str) -> i64
{
    let mut pos = Point::new(0, 0);
    let mut waypoint = Point::new(10, 1);

    for line in input_to_lines(input)
    {
        let (code, dist) = scan(&line)
            .take(1).parse::<char>()
            .remaining().parse::<i64>();

        match code
        {
            'N' =>
            {
                waypoint += Point::new(0, dist);
            },
            'S' =>
            {
                waypoint += Point::new(0, -dist);
            },
            'E' =>
            {
                waypoint += Point::new(dist, 0);
            },
            'W' =>
            {
                waypoint += Point::new(-dist, 0);
            },
            'L' =>
            {
                let mut move_dir = waypoint - pos;
                match dist
                {
                    90 => move_dir = move_dir.rotate_90_left(),
                    180 => move_dir = move_dir.rotate_180(),
                    270 => move_dir = move_dir.rotate_90_right(),
                    _ => unreachable!(),
                }
                waypoint = pos + move_dir;
            },
            'R' =>
            {
                let mut move_dir = waypoint - pos;
                match dist
                {
                    90 => move_dir = move_dir.rotate_90_right(),
                    180 => move_dir = move_dir.rotate_180(),
                    270 => move_dir = move_dir.rotate_90_left(),
                    _ => unreachable!(),
                }
                waypoint = pos + move_dir;
            },
            'F' =>
            {
                let move_dir = waypoint - pos;

                pos += dist * move_dir;
                waypoint += dist * move_dir;
            },
            _ => unreachable!(),
        }
    }
    
    pos.manhatten_size()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(12)
        .example(|| Answer { calculated: part_1(EXAMPLE), expected: 25, })
        .part_1(|| Answer { calculated: part_1(INPUT), expected: 2228, })
        .example(|| Answer { calculated: part_2(EXAMPLE), expected: 286, })
        .part_2(|| Answer { calculated: part_2(INPUT), expected: 42908, })
}
