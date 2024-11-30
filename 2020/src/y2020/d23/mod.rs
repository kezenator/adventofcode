use itertools::Itertools;
use crate::support::*;

const EXAMPLE: &str = "389125467";

fn simulate(input: &str, num_cups: usize, num_moves: usize) -> Vec<usize>
{
    // First - create the list of cups in the initial order.
    // This has no real purpose, except for creating the initial next
    // linked list in the correct initial order.
    //
    // Start with the digits from the input, then
    // append additional numbers until we reach the required number
    // of cups.

    let mut cups = input.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>();

    cups.reserve(num_cups);

    for i in 10..(num_cups + 1)
    {
        cups.push(i);
    }

    // Now, use this list to create the next linked list.
    // If cup is a cup number, next[cup] is the cup next in the ring.

    let mut next = Vec::<usize>::new();

    next.resize(num_cups + 1, 0);

    for i in 0..num_cups
    {
        let cur_cup = cups[i];
        let next_cup = cups[(i + 1) % num_cups];

        next[cur_cup] = next_cup;
    }

    let mut cur_cup = cups[0];

    // Now - apply the moves.
    // For each move:
    // 1) Collect 5 cups using the next list:
    //    cur, a, b, c, next_cur
    // 2) Subtract 1 until we find the dest cup -
    //    that can't be a, b or c.
    // 3) The next list gives us dest_next
    // 3) Shuffle - moving a/b/c from between cur/next_cur to dest/dest_next:
    //    cur => next_cur
    //    dest => a
    //    c => dest_next

    for _ in 0..num_moves
    {
        let a = next[cur_cup];
        let b = next[a];
        let c = next[b];
        let next_cur = next[c];

        let mut dest = ((cur_cup + num_cups - 2) % num_cups) + 1;

        while dest == a || dest == b || dest == c
        {
            dest = ((dest + num_cups - 2) % num_cups) + 1            
        }

        let dest_next = next[dest];

        next[cur_cup] = next_cur;
        next[dest] = a;
        next[c] = dest_next;

        cur_cup = next_cur;
    }

    // Finally, return the ring of cups
    // starting from cup 1

    let mut result = Vec::<usize>::new();
    result.reserve(num_cups);

    let mut cur_cup = 1;

    for _ in 0..num_cups
    {
        result.push(cur_cup);
        cur_cup = next[cur_cup];
    }

    result
}

fn part_1(input: &str) -> String
{
    simulate(input, 9, 100)
        .iter()
        .skip(1)
        .map(|c| c.to_string())
        .join("")
}

fn part_2(input: &str) -> usize
{
    simulate(input, 1000000, 10000000)
        .iter()
        .skip(1)
        .take(2)
        .product()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(23)
        .example(|| Answer { calculated: simulate(EXAMPLE, 9, 10).iter().join(""), expected: "192658374", })
        .example(|| Answer { calculated: part_1(EXAMPLE), expected: "67384529", })
        .part_1(|input| Answer { calculated: part_1(input), expected: "75893264", })
        .example(|| Answer { calculated: part_2(EXAMPLE), expected: 149245887792u64, })
        .part_2(|input| Answer { calculated: part_2(input), expected: 38162588308u64, })
}
