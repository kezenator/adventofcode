use std::collections::HashSet;
use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");

fn parse_dirs(input: &str) -> Vec<Point>
{
    let mut result = Vec::new();
    let mut it = input.chars();

    loop
    {
        match it.next()
        {
            None => break,
            Some(ch) =>
            {
                match ch
                {
                    'e' => result.push(Point::new(2, 0)),
                    'w' => result.push(Point::new(-2, 0)),
                    'n' =>
                    {
                        match it.next().unwrap()
                        {
                            'e' => result.push(Point::new(1, 1)),
                            'w' => result.push(Point::new(-1, 1)),
                            _ => unreachable!(),
                        }
                    },
                    's' =>
                    {
                        match it.next().unwrap()
                        {
                            'e' => result.push(Point::new(1, -1)),
                            'w' => result.push(Point::new(-1, -1)),
                            _ => unreachable!(),
                        }
                    },
                    _ => unreachable!(),
                };
            },
        }
    }

    result
}

fn flip_tiles(input: &str) -> HashSet<Point>
{
    let mut result = HashSet::new();

    for line in input_to_lines(input)
    {
        let mut pos = Point::new(0, 0);
        for p in parse_dirs(&line)
        {
            pos += p;
        }

        if result.contains(&pos)
        {
            result.remove(&pos);
        }
        else
        {
            result.insert(pos);
        }
    }

    result
}

fn neighbours(pos: &Point) -> Vec<Point>
{
    let mut result = Vec::new();
    result.push(Point::new(pos.x + 2, pos.y));
    result.push(Point::new(pos.x - 2, pos.y));
    result.push(Point::new(pos.x + 1, pos.y + 1));
    result.push(Point::new(pos.x - 1, pos.y + 1));
    result.push(Point::new(pos.x + 1, pos.y - 1));
    result.push(Point::new(pos.x - 1, pos.y - 1));
    result
}

fn part_1(input: &str) -> usize
{
    flip_tiles(input).len()
}

fn part_2(input: &str) -> usize
{
    let mut state = flip_tiles(input);

    for _ in 0..100
    {
        let mut points_to_test = HashSet::new();
        for p in state.iter()
        {
            for n in neighbours(p)
            {
                points_to_test.insert(n);
            }
        }

        let mut new_state = HashSet::new();

        for p in points_to_test
        {
            let cur_black = state.contains(&p);
            let black_neighbours = neighbours(&p).into_iter()
                .filter(|n| state.contains(n))
                .count();

            let new_black = if cur_black
            {
                if black_neighbours == 0 || black_neighbours > 2
                {
                    false
                }
                else
                {
                    true
                }
            }
            else // currently white
            {
                if black_neighbours == 2
                {
                    true
                }
                else
                {
                    false
                }
            };

            if new_black
            {
                new_state.insert(p);
            }
        }

        state = new_state;
    }

    state.len()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(24)
        .example(|| Answer { calculated: part_1(EXAMPLE), expected: 10, })
        .part_1(|input| Answer { calculated: part_1(input), expected: 436, })
        .example(|| Answer { calculated: part_2(EXAMPLE), expected: 2208, })
        .part_2(|input| Answer { calculated: part_2(input), expected: 4133, })
}
