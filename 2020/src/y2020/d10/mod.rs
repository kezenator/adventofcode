use std::collections::HashMap;
use crate::support::*;

const EXAMPLE_1: &str = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";
const EXAMPLE_2: &str = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3";
const INPUT: &str = include_str!("input.txt");

fn parse_input(input: &str) -> Vec<u64>
{
    let mut nums = input_to_lines_parsed::<u64>(input);

    nums.sort();

    nums.insert(0, 0);
    nums.push(nums.last().unwrap() + 3);

    nums
}

fn part_1(input: &str) -> u64
{
    let mut num_1_diff = 0;
    let mut num_3_diff = 0;

    let nums = parse_input(input);

    for pair in nums.windows(2)
    {
        match pair[1] - pair[0]
        {
            1 => num_1_diff += 1,
            2 => (),
            3 => num_3_diff += 1,
            _ => unreachable!(),
        }
    }

    num_1_diff * num_3_diff
}

fn part_2_top_down(input: &str) -> u64
{
    let vals = parse_input(input);
    let target = *vals.last().unwrap();

    // Top down solution - from the final target,
    // try and reach the starting point (0 jolts).
    //
    // Calculate the number of ways to reach a target jolts,
    // using a Memorized Function to speed things up.
    //
    // There's one way to reach 0 jolts - plug into the seat.
    //
    // For each larger target:
    // 1) Filter out the previous targets that can possibly reach this target in 1-2 jolt increments
    // 2) Map each previous target to the number of ways to reach it
    // 3) Sum all of the ways to reach the previous targets

    let func = move |target: &u64, ways_to_reach: &Memorized<u64, u64>| -> u64
    {
        if *target == 0
        {
            1
        }
        else
        {
            vals.iter()
                .filter(|&i| (*i < *target) && ((i + 3) >= *target))
                .map(|i| ways_to_reach.get(i))
                .sum()
        }
    };

    Memorized::new(&func)
        .get(&target)
}

fn part_2_bottom_up(input: &str) -> u64
{
    let vals = parse_input(input);
    let target = *vals.last().unwrap();

    // Bottom up solution - from the seat (0 jolts),
    // summing up the ways to reach the final target.
    //
    // 1) Start with 0 ways to reach each target, except 1
    //    way to reach zero jolts.
    // 2) For each sub-target in order, add it's number of
    //    ways onto all larget targets it can reach.
    // 3) Return the final sum of the final target.
    //
    // Could inprove this to only use 3 words of temorary
    // memory storage - but I can't be bothered.

    let mut sums = HashMap::<u64, u64>::new();
    for v in vals.iter()
    {
        sums.insert(*v, 0);
    }
    sums.insert(0, 1);

    for sub in vals.iter()
    {
        let sub_sum = *sums.get(sub).unwrap();

        for target in (sub + 1)..(sub + 4)
        {
            if let Some(target_sum) = sums.get_mut(&target)
            {
                *target_sum += sub_sum;
            }
        }
    }

    *sums.get(&target).unwrap()
}

fn part_2(input: &str) -> u64
{
    let top_down = part_2_top_down(input);
    let bottom_up = part_2_bottom_up(input);

    assert_eq!(top_down, bottom_up);

    top_down
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(10)
        .example(|| Answer { calculated: part_1(EXAMPLE_1), expected: 35, })
        .example(|| Answer { calculated: part_1(EXAMPLE_2), expected: 220, })
        .part_1(|| Answer { calculated: part_1(INPUT), expected: 2482, })
        .example(|| Answer { calculated: part_2(EXAMPLE_1), expected: 8, })
        .example(|| Answer { calculated: part_2(EXAMPLE_2), expected: 19208, })
        .part_2(|| Answer { calculated: part_2(INPUT), expected: 96717311574016u64, })
}
