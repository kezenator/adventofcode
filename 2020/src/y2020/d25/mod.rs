use crate::support::*;

const INPUT: &str = "10943862\n12721030";

fn public_key_to_loop_size(pub_key: u64) -> u64
{
    let mut result = 1;
    let subject_num = 7;

    for loop_size in 1..u64::MAX
    {
        result = (result * subject_num) % 20201227;

        if result == pub_key
        {
            return loop_size;
        }
    }
    unreachable!();
}

fn transform(subject: u64, loop_size: u64) -> u64
{
    let mut result = 1;

    for _ in 0..loop_size
    {
        result = (result * subject) % 20201227;
    }

    result
}

fn part_1(input: &str) -> u64
{
    let input = input_to_lines_parsed::<u64>(input);
    let pub1 = input[0];
    let pub2 = input[1];

    let loop_size1 = public_key_to_loop_size(pub1);
    transform(pub2, loop_size1)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(25)
        .example(|| Answer { calculated: public_key_to_loop_size(5764801), expected: 8, })
        .example(|| Answer { calculated: public_key_to_loop_size(17807724), expected: 11, })
        .part_1(|| Answer { calculated: part_1(INPUT), expected: 5025281, })
        .final_gift()
}
