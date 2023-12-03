use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

fn from_snafu_digit(digit: char) -> i64
{
    match digit
    {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => unreachable!(),
    }
}

fn to_snafu_digit(digit: i64) -> char
{
    match digit
    {
        2 => '2',
        1 => '1',
        0 => '0',
        -1 => '-',
        -2 => '=',
        _ => unreachable!(),
    }
}

fn from_snafu(snafu: &str) -> i64
{
    let mut sum = 0;
    let mut radix = 1;

    for ch in snafu.chars().rev()
    {
        sum += radix * from_snafu_digit(ch);
        radix *=5;
    }
    //println!("Snafu {} => {}", snafu, sum);
    sum
}

fn to_snafu(val: i64) -> String
{
    let mut digit_vals = Vec::new();
    let mut remaining = val;
    while remaining != 0
    {
        let mut digit_val = remaining % 5;
        remaining /= 5;

        if digit_val > 2
        {
            digit_val -= 5;
            remaining += 1;
        }
        digit_vals.push(digit_val);
    }
    let result = digit_vals.into_iter()
        .rev()
        .map(|d| to_snafu_digit(d))
        .collect::<String>();

    //println!("Base10 {} => Snafu {}", val, result);
    result
}

fn part_1(input: &str) -> String
{
    to_snafu(input_to_lines(input)
        .into_iter()
        .map(|l| from_snafu(&l))
        .sum())
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(25)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: "2=-1=0",
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: "2==221=-002=0-02-000",
        })
        .final_gift()
}
