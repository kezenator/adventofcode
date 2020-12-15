use crate::support::*;
use std::str::FromStr;
use itertools::Itertools;

const EXAMPLE_1: &str = "deal with increment 7\ndeal into new stack\ndeal into new stack";
const EXAMPLE_2: &str = "cut 6\ndeal with increment 7\ndeal into new stack";
const EXAMPLE_3: &str = "deal with increment 7\ndeal with increment 9\ncut -2";
const EXAMPLE_4: &str = "deal into new stack\ncut -2\ndeal with increment 7\ncut 8\ncut -4\ndeal with increment 7\ncut 3\ndeal with increment 9\ndeal with increment 3\ncut -1";
const INPUT: &str = include_str!("input.txt");

enum Step
{
    Deal,
    Cut(i64),
    DealInc(u64),
}

impl FromStr for Step
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        if s == "deal into new stack"
        {
            Ok(Step::Deal)
        }
        else if s.starts_with("cut ")
        {
            let (num,) = scan(s)
                .skip_str("cut ")
                .remaining().parse::<i64>();

            Ok(Step::Cut(num))
        }
        else
        {
            let (num,) = scan(s)
                .skip_str("deal with increment ")
                .remaining().parse::<u64>();

            Ok(Step::DealInc(num))
        }
    }
}

fn card_in_position(input: &str, pos: u64, len: u64) -> u64
{
    let mut pos = pos;

    for step in input_to_lines_parsed::<Step>(input).iter().rev()
    {
        match &step
        {
            Step::Deal =>
            {
                pos = len - 1 - pos;
            },
            Step::Cut(num) =>
            {
                assert_ne!(*num, 0);
                assert!((num.abs() as u64) < len);

                let sum = ((pos as i64) + num) % (len as i64);

                if sum >= 0
                {
                    pos = sum as u64;
                }
                else
                {
                    pos = ((len as i64) + sum) as u64;
                }
            },
            Step::DealInc(num) =>
            {
                assert!(*num < len);
                assert_eq!(gcd(*num, len), 1);

                // So deal step (num) and total length (len) are relatively
                // prime, and step (num) is smaller.
                //
                // The card on top is *always* 0.
                // If we know the next card, we an calculate
                // any position by (next * pos) % len.
                //
                // To find the next pos:
                // Work out how many num/s fit into len,
                // Then

                let pos_1: u64 = (||
                {
                    let mut p: u64 = 0;
                    let mut result: u64 = 0;
                    loop
                    {
                        let rem = len - p;
                        let times = rem / num;

                        result += times + 1;
                        p = (p + num * (times + 1)) % len;

                        if p == 1
                        {
                            return result;
                        }
                    }
                })();

                pos = (pos * pos_1) % len;
            },
        }
    }

    pos
}

fn example(input: &str) -> String
{
    let len = 10;

    (0..len)
        .map(|i| card_in_position(input, i, len).to_string())
        .join(" ")
}

fn part_1(input: &str) -> u64
{
    let len = 10007;

    for i in 0..len
    {
        if card_in_position(input, i, len) == 2019
        {
            return i;
        }
    }
    unreachable!();
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(22)
        .example(|| Answer { calculated: example(""), expected: "0 1 2 3 4 5 6 7 8 9", })
        .example(|| Answer { calculated: example("deal into new stack"), expected: "9 8 7 6 5 4 3 2 1 0", })
        .example(|| Answer { calculated: example("deal with increment 7"), expected: "0 3 6 9 2 5 8 1 4 7", })
        .example(|| Answer { calculated: example("deal with increment 3"), expected: "0 7 4 1 8 5 2 9 6 3", })
        .example(|| Answer { calculated: example("deal with increment 9"), expected: "0 9 8 7 6 5 4 3 2 1", })
        .example(|| Answer { calculated: example("cut 3"), expected: "3 4 5 6 7 8 9 0 1 2", })
        .example(|| Answer { calculated: example("cut -4"), expected: "6 7 8 9 0 1 2 3 4 5", })
        .example(|| Answer { calculated: example(EXAMPLE_1), expected: "0 3 6 9 2 5 8 1 4 7", })
        .example(|| Answer { calculated: example(EXAMPLE_2), expected: "3 0 7 4 1 8 5 2 9 6", })
        .example(|| Answer { calculated: example(EXAMPLE_3), expected: "6 3 0 7 4 1 8 5 2 9", })
        .example(|| Answer { calculated: example(EXAMPLE_4), expected: "9 2 5 8 1 4 7 0 3 6", })
        .part_1(|| Answer { calculated: part_1(INPUT), expected: 1498, })
        .part_2(|| Answer { calculated: "unknown", expected: "unknown", })
}
