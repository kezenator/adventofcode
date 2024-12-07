
use regex::Regex;
use crate::support::*;

const EXAMPLE1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const EXAMPLE2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn run_programme(input: &str, use_conditions: bool) -> usize
{
    let r = Regex::new("(mul\\([0-9]+,[0-9]+\\))|(do\\(\\))|(don\\'t\\(\\))").unwrap();

    let mut enabled = true;
    let mut result = 0;
    for m in r.find_iter(input)
    {
        match m.as_str()
        {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ =>
            {
                let (a, b): (usize, usize) = scan(m.as_str())
                    .skip_str("mul(")
                    .take_digits().parse()
                    .skip_str(",")
                    .take_digits().parse()
                    .remaining().ignore();
                if enabled || !use_conditions
                {
                    result += a * b;
                }
            },
        }
    }
    result
}

fn part_1(input: &str) -> usize
{
    run_programme(input, false)
}

fn part_2(input: &str) -> usize
{
    run_programme(input, true)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(3)
        .example(|| Answer {
            calculated: part_1(EXAMPLE1),
            expected: 161,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 160672468,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE2),
            expected: 48,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 84893551,
        })
}
