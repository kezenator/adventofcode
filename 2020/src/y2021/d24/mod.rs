use crate::support::*;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Round
{
    round_num: usize,
    div: i64,
    cx: i64,
    cy: i64,
}

#[derive(Debug)]
struct Pair
{
    i1: usize,
    i2: usize,
    diff: i64,
}

fn parse_rounds(input: &str) -> Vec<Round>
{
    let lines = input_to_lines(input);
    assert!(lines.len() == (14 * 18));

    let mut rounds = Vec::new();

    let mut div = 0;
    let mut cx = 0;
    let mut cy = 0;

    for round_num in 0..14
    {
        for j in 0..18
        {
            let i = 18 * round_num + j;
            let line = &lines[i];

            match j
            {
                0 => assert_eq!(line, "inp w"),
                1 => assert_eq!(line, "mul x 0"),
                2 => assert_eq!(line, "add x z"),
                3 => assert_eq!(line, "mod x 26"),
                4 =>
                {
                    assert_eq!(line[0..6], *"div z ");
                    div = line[6..].parse::<i64>().unwrap();
                },
                5 =>
                {
                    assert_eq!(line[0..6], *"add x ");
                    cx = line[6..].parse::<i64>().unwrap();
                },
                6 => assert_eq!(line, "eql x w"),
                7 => assert_eq!(line, "eql x 0"),
                8 => assert_eq!(line, "mul y 0"),
                9 => assert_eq!(line, "add y 25"),
                10 => assert_eq!(line, "mul y x"),
                11 => assert_eq!(line, "add y 1"),
                12 => assert_eq!(line, "mul z y"),
                13 => assert_eq!(line, "mul y 0"),
                14 => assert_eq!(line, "add y w"),
                15 =>
                {
                    assert_eq!(line[0..6], *"add y ");
                    cy = line[6..].parse::<i64>().unwrap();

                    rounds.push(Round{ round_num, div, cx, cy });
                },
                16 => assert_eq!(line, "mul y x"),
                17 => assert_eq!(line, "add z y"),
                _ => unreachable!(),
            }
        }
    }

    rounds
}

fn parse_pairs(input: &str) -> Vec<Pair>
{
    let rounds = parse_rounds(input);
    assert_eq!(rounds.len(), 14);

    let mut pairs = Vec::new();
    let mut stack = Vec::new();

    for round in rounds
    {
        if round.div == 1
        {
            // When div == 1, check that cx is
            // larger than 9. This means that
            // (z % 26 + cx) can never be equal
            // to w. This means we always "push"
            // w + cy into z - i.e.
            // z = (z * 26) + (w + cy)

            assert!(round.cx > 9);

            stack.push(round);
        }
        else if round.div == 26
        {
            // When div == 26, we need to "pop" an
            // entry off the stack. This only occurs
            // when z % 26 == w + cx. But z % 26 is
            // w + cy from the matching "pushed" entry.
            // So (w1 + cy1) == (w2 + cx2)
            // or w2 = w1 + cy1 - cx2.
            // So we need to store this difference
            // (cy1 + cx2).

            assert!(!stack.is_empty());
            let matching_round = stack.pop().unwrap();

            let diff = matching_round.cy + round.cx;

            pairs.push(Pair
            {
                i1: matching_round.round_num,
                i2: round.round_num,
                diff: diff,
            });
        }
        else // round.div is not 1 or 26
        {
            unreachable!();
        }
    }
    assert!(stack.is_empty());

    pairs
}

fn num_to_char(num: i64) -> char
{
    assert!((num >= 1) && (num <= 9));

    ((num as u8) + ('0' as u8)) as char
}

fn solve(input: &str, lowest: bool) -> u64
{
    let mut chars = vec![' '; 14];

    for pair in parse_pairs(input)
    {
        // Remember: w2 = w1 + diff

        if lowest
        {
            // Try for w1 = 1, unless diff
            // is negative, in which case
            // keep w1 as small as possible
            // by setting w2 = 1.

            if pair.diff < 0
            {
                chars[pair.i1] = num_to_char(1 - pair.diff);
                chars[pair.i2] = num_to_char(1);
            }
            else
            {
                chars[pair.i1] = num_to_char(1);
                chars[pair.i2] = num_to_char(1 + pair.diff);
            }
        }
        else // highest
        {
            // Try for w1 = 9, unless diff
            // is positive, in which case
            // keep w1 as large as possible
            // by setting w2 = 9.

            if pair.diff > 0
            {
                chars[pair.i1] = num_to_char(9 - pair.diff);
                chars[pair.i2] = num_to_char(9);
            }
            else
            {
                chars[pair.i1] = num_to_char(9);
                chars[pair.i2] = num_to_char(9 + pair.diff);
            }
        }
    }

    chars.iter().copied().collect::<String>().parse::<u64>().unwrap()
}

fn part_1(input: &str) -> u64
{
    solve(input, false)
}

fn part_2(input: &str) -> u64
{
    solve(input, true)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(24)
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 92967699949891u64,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 91411143612181u64,
        })
}
