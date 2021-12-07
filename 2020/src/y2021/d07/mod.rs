use crate::support::*;

const EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";
const INPUT: &str = include_str!("input.txt");

fn cost_to_move_to<F>(nums: &Vec<i64>, pos: i64, move_cost: &F) -> i64
    where F: Fn(i64) -> i64
{
    // For each crab,
    // find the cost to move to the requested position
    // then sum across all crabs

    nums.iter()
        .map(|n| move_cost((n - pos).abs()))
        .sum()
}

fn solve<F>(input: &str, move_cost: F) -> i64
    where F: Fn(i64) -> i64
{
    let nums = scan(&input_to_lines(input)[0]).remaining().parse_vec::<i64>(",").0;

    let min: i64 = *nums.iter().min().unwrap();
    let max: i64 = *nums.iter().max().unwrap();

    // For each position,
    // find the cost to move all crabs there
    // and find the minimum of all these

    (min..(max+1))
        .map(|pos| cost_to_move_to(&nums, pos, &move_cost))
        .min().unwrap()
}

fn part_1(input: &str) -> i64
{
    solve(input, |a| a)
}

fn part_2(input: &str) -> i64
{
    // Cost to move is triangular numbers
    
    solve(input, |a| a * (a + 1) / 2)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(7)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 37,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 353800,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 168,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 98119739,
        })
}
