use crate::support::*;

const INPUT: &str = include_str!("input.txt");

fn seat_id(s: &str) -> u64
{
    s.chars().map(
        |c| match c
        {
            'F' => 0, 'L' => 0,
            'B' => 1, 'R' => 1,
            _ => unreachable!(),
        }
    ).fold(0, |a, b| 2 * a + b)
}

pub fn part_1(input: &str) -> u64
{
    input_to_lines(input).iter()
        .map(|line| seat_id(line))
        .max()
        .expect("Must be some input....")
}

pub fn part_2(input: &str) -> u64
{
    let mut seats = input_to_lines(input).iter()
        .map(|line| seat_id(line))
        .collect::<Vec<_>>();

    seats.sort();

    for a in seats.windows(2)
    {
        if a[1] == (a[0] + 2)
        {
            return a[0] + 1;
        }
    }
    
    unreachable!();
}

pub fn puzzles(puzzles: &mut PuzzleSet)
{
    puzzles.register("y2020.d05.e1", || Answer {
        calculated: seat_id("BFFFBBFRRR"),
        expected: 567,
    });

    puzzles.register("y2020.d05.e2", || Answer {
        calculated: seat_id("FFFBBBFRRR"),
        expected: 119,
    });

    puzzles.register("y2020.d05.e3", || Answer {
        calculated: seat_id("BBFFBBFRLL"),
        expected: 820,
    });

    puzzles.register("y2020.d05.p1", || Answer {
        calculated: part_1(INPUT),
        expected: 978,
    });

    puzzles.register("y2020.d05.p2", || Answer {
        calculated: part_2(INPUT),
        expected: 727,
    });
}
