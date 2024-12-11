
use itertools::*;
use crate::support::*;

const EXAMPLE: &str = "125 17";

fn num_stones_after_blinks(input: &str, blinks: usize) -> usize
{
    let stones = input
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();

    let memorize = Memorized::new(
        &|(stone, after_splits), func|
        {
            if *after_splits == 0
            {
                // Have a single stone after no more splits
                return 1;
            }

            if *stone == 0
            {
                // 0 turns into 1
                return func.get(&(1, after_splits - 1));
            }
            let as_str = stone.to_string();
            if as_str.len() % 2 == 0
            {
                // Even - split
                let half_len = as_str.len() / 2;
                let left = as_str[0..half_len].parse().unwrap();
                let right = as_str[half_len..].parse().unwrap();

                return func.get(&(left, after_splits - 1))
                    + func.get(&(right, after_splits - 1));
            }

            // Else - multiply
            func.get(&(2024 * stone, after_splits - 1))
        });

    stones.into_iter()
        .map(|s| memorize.get(&(s, blinks)))
        .sum()
}

fn part_1(input: &str) -> usize
{
    num_stones_after_blinks(input, 25)
}

fn part_2(input: &str) -> usize
{
    num_stones_after_blinks(input, 75)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(11)
        .example(|| Answer {
            calculated: num_stones_after_blinks("0 1 10 99 999", 1),
            expected: 7,
        })
        .example(|| Answer {
            calculated: num_stones_after_blinks(EXAMPLE, 6),
            expected: 22,
        })
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 55312,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 228668,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 270673834779359usize,
        })
}
