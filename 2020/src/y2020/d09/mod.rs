use itertools::Itertools;
use crate::support::*;

const EXAMPLE: &str = "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576\n";

fn part_1(input: &str, len: usize) -> usize
{
    for window in input_to_lines_parsed::<usize>(input).windows(len + 1)
    {
        let sum = window[len];

        if window[0..len]
            .iter()
            .combinations(2)
            .filter(|c| c[0] + c[1] == sum)
            .next()
            .is_none()
        {
            return sum;
        }
    }
    unreachable!();
}

fn part_2(input: &str, sum: usize) -> usize
{
    let nums = input_to_lines_parsed::<usize>(input);

    for len in 2..nums.len()
    {
        for range in nums.windows(len)
        {
            if range.iter().sum::<usize>() == sum
            {
                let mut range = range.iter().cloned().collect::<Vec<usize>>();

                range.sort();

                return range[0] + range[len - 1];
            }
        }
    }
    unreachable!();
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(9)
        .example(|| Answer { calculated: part_1(EXAMPLE, 5), expected: 127, })
        .part_1(|input| Answer { calculated: part_1(input, 25), expected: 1492208709, })
        .example(|| Answer { calculated: part_2(EXAMPLE, part_1(EXAMPLE, 5)), expected: 62, })
        .part_2(|input| Answer { calculated: part_2(input, part_1(input, 25)), expected: 238243506, })
}
