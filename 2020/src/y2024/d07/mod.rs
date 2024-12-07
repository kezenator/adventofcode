
use rayon::prelude::*;
use std::{collections::HashSet, str::FromStr};
use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");

struct Equation
{
    test_value: usize,
    numbers: Vec<usize>,
}

impl FromStr for Equation
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (test_value, numbers) = scan(s).until(": ").parse().remaining().parse_vec(" ");
        Ok(Equation { test_value, numbers })
    }
}

impl Equation
{
    fn has_solution(&self, use_concatenation: bool) -> bool
    {
        let mut cur_values = HashSet::new();
        cur_values.insert(self.numbers[0]);

        for num in self.numbers.iter().skip(1)
        {
            // Work out the next values we can reach from
            // the current values and the next number

            let mut next_values = HashSet::new();
            next_values.reserve(3 * cur_values.len());
            for cur_val in cur_values.into_iter()
            {
                next_values.insert(cur_val + num);
                next_values.insert(cur_val * num);

                if use_concatenation
                {
                    let mut concat_shift = 10;
                    while concat_shift <= *num
                    {
                        concat_shift *= 10;
                    }
                    next_values.insert(cur_val * concat_shift + num);
                }
            }

            // Filter out only the ones that have
            // not exceeded the target value

            cur_values = HashSet::new();
            cur_values.reserve(next_values.len());
            for s in next_values
            {
                if s <= self.test_value
                {
                    cur_values.insert(s);
                }
            }
        }

        // Did we reach the target value after
        // processing all numbers??

        cur_values.contains(&self.test_value)
    }
}

fn part_1(input: &str) -> usize
{
    let equations = input_to_lines_parsed::<Equation>(input);

    equations.par_iter()
        .filter(|e| e.has_solution(false))
        .map(|e| e.test_value)
        .sum()
}

fn part_2(input: &str) -> usize
{
    let equations = input_to_lines_parsed::<Equation>(input);

    equations.par_iter()
        .filter(|e| e.has_solution(true))
        .map(|e| e.test_value)
        .sum()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(7)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 3749,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 12839601725877usize,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 11387,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 149956401519484usize,
        })
}
