use crate::support::*;

const EXAMPLE: &str = "3,4,3,1,2";
const INPUT: &str = include_str!("input.txt");

fn simulate(input: &str, days: usize) -> u64
{
    let start = input_to_lines(input)[0]
        .split_terminator(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    // The key point is that all lanternfish with the same
    // internal timer value behave the same - so we only need
    // to simulate each state (0..8), and keep count of 
    // *how many* fish of each type there are.
    // This converts the problem from O(exp) to simply
    // linear in the number of days.

    let mut num_with_timer: Vec<u64> = vec![0; 9];

    // Count starting fish

    for s in start
    {
        num_with_timer[s] += 1;
    }

    // Simulate each day

    for _ in 0..days
    {
        // Count down each timer. Fish with timer zero
        // reset to 6 *and* create a new fish with timer 8

        num_with_timer = vec![
            num_with_timer[1],
            num_with_timer[2],
            num_with_timer[3],
            num_with_timer[4],
            num_with_timer[5],
            num_with_timer[6],
            num_with_timer[7] + num_with_timer[0],
            num_with_timer[8],
            num_with_timer[0],
        ];
    }

    // Return total number of fish

    num_with_timer.iter().sum()
}

fn part_1(input: &str) -> u64
{
    simulate(input, 80)
}

fn part_2(input: &str) -> u64
{
    simulate(input, 256)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(6)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 5934,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 380243,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 26984457539u64,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 1708791884591u64,
        })
}
