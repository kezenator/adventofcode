use std::collections::HashMap;
use crate::support::*;

const EXAMPLE_1: &str = "0,3,6";

pub fn after_turns(input: &str, total_turns: usize) -> usize
{
    assert!(total_turns > 0);

    let starting = input
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    if total_turns <= starting.len()
    {
        return starting[total_turns - 1];
    }

    let mut number_to_turn = starting.iter()
        .enumerate()
        .map(|(index, num)| (*num, index + 1))
        .collect::<HashMap<usize, usize>>();

    let mut turn = starting.len();
    let mut just_spoken = starting[starting.len() - 1];

    loop
    {
        let mut answer = 0;

        if let Some(last_turn_spoken) = number_to_turn.get_mut(&just_spoken)
        {
            // Has been spoken before - calculate the answer as the difference
            // in turns, and directly update the HashMap (saving an extra lookup)

            answer = turn - *last_turn_spoken;
            *last_turn_spoken = turn;
        }
        else
        {
            // Never spoken before - leave answer as zero
            // and manually insert into the HashMap

            number_to_turn.insert(just_spoken, turn);
        }

        turn += 1;
        just_spoken = answer;

        if turn == total_turns
        {
            return answer;
        }
    }
}

pub fn part_1(input: &str) -> usize
{
    after_turns(input, 2020)
}

fn part_2(input: &str) -> usize
{
    after_turns(input, 30000000)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(15)
        .example(|| Answer { calculated: after_turns(EXAMPLE_1, 1), expected: 0, })
        .example(|| Answer { calculated: after_turns(EXAMPLE_1, 2), expected: 3, })
        .example(|| Answer { calculated: after_turns(EXAMPLE_1, 3), expected: 6, })
        .example(|| Answer { calculated: after_turns(EXAMPLE_1, 4), expected: 0, })
        .example(|| Answer { calculated: after_turns(EXAMPLE_1, 5), expected: 3, })
        .example(|| Answer { calculated: after_turns(EXAMPLE_1, 6), expected: 3, })
        .example(|| Answer { calculated: after_turns(EXAMPLE_1, 7), expected: 1, })
        .example(|| Answer { calculated: after_turns(EXAMPLE_1, 8), expected: 0, })
        .example(|| Answer { calculated: after_turns(EXAMPLE_1, 9), expected: 4, })
        .example(|| Answer { calculated: after_turns(EXAMPLE_1, 10), expected: 0, })
        .example(|| Answer { calculated: part_1("1,3,2"), expected: 1, })
        .example(|| Answer { calculated: part_1("2,1,3"), expected: 10, })
        .example(|| Answer { calculated: part_1("1,2,3"), expected: 27, })
        .example(|| Answer { calculated: part_1("2,3,1"), expected: 78, })
        .example(|| Answer { calculated: part_1("3,2,1"), expected: 438, })
        .example(|| Answer { calculated: part_1("3,1,2"), expected: 1836, })
        .part_1(|input| Answer { calculated: part_1(input), expected: 1280, })
        .example(|| Answer { calculated: part_2("0,3,6"), expected: 175594, })
        .example(|| Answer { calculated: part_2("1,3,2"), expected: 2578, })
        .example(|| Answer { calculated: part_2("2,1,3"), expected: 3544142, })
        .example(|| Answer { calculated: part_2("1,2,3"), expected: 261214, })
        .example(|| Answer { calculated: part_2("2,3,1"), expected: 6895259, })
        .example(|| Answer { calculated: part_2("3,2,1"), expected: 18, })
        .example(|| Answer { calculated: part_2("3,1,2"), expected: 362, })
        .part_2(|input| Answer { calculated: part_2(input), expected: 651639, })
}
