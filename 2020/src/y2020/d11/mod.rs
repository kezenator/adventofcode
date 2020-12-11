use std::collections::HashSet;
use crate::support::*;

const EXAMPLE: &str = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL";
const INPUT: &str = include_str!("input.txt");

fn solve(input: &str, one_step: bool, occupied_limit: usize) -> usize
{
    let mut seen = HashSet::new();
    let mut cur = CharGrid::new_from_input(input, '.');

    let all_points = cur.all_points();
    let directions = Point::directions_8();

    loop
    {
        if !seen.insert(cur.clone())
        {
            // We've already seen this state - we're now stable

            return cur.all_chars().iter()
                .filter(|&c| *c == '#')
                .count()
        }

        let mut next = cur.clone();

        for p in all_points.iter()
        {
            let cur_char = cur.get_char(&p);

            let char_in_dir = |dir: Point| -> char
            {
                let mut cur_pos = *p + dir;

                loop
                {
                    let ch = cur.get_char(&cur_pos);

                    if ch != '.'
                    {
                        // Found the first seat
                        return ch;
                    }

                    if one_step || !cur.is_point_in_bounds(&cur_pos)
                    {
                        // Stepped too far - no seat found
                        return '.';
                    }

                    cur_pos = cur_pos + dir;
                }
            };

            let occupied = directions.iter()
                .map(|d| char_in_dir(*d))
                .filter(|&c| c == '#')
                .count();

            if cur_char == 'L' && occupied == 0
            {
                next.put_char(p, '#');
            }
            else if cur_char == '#' && occupied >= occupied_limit
            {
                next.put_char(p, 'L');
            }
        }

        cur = next;
    }
}

fn part_1(input: &str) -> usize
{
    solve(input, true, 4)
}

fn part_2(input: &str) -> usize
{
    solve(input, false, 5)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(11)
        .example(|| Answer { calculated: part_1(EXAMPLE), expected: 37, })
        .part_1(|| Answer { calculated: part_1(INPUT), expected: 2489, })
        .example(|| Answer { calculated: part_2(EXAMPLE), expected: 26, })
        .part_2(|| Answer { calculated: part_2(INPUT), expected: 2180, })
}
