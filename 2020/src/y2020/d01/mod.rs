use crate::support::*;
use itertools::Itertools;

const EXAMPLE: &str = "1721\n979\n366\n299\n675\n1456\n";

fn product_of_terms_that_sum_to_2020(input: &str, num_terms: usize) -> u64
{
    input_to_lines_parsed::<u64>(input)
        .drain(..)
        .combinations(num_terms)
        .filter(|s| s.iter().sum::<u64>() == 2020)
        .next().unwrap()
        .drain(..)
        .product()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(1)
        .example(|| Answer {
            calculated: product_of_terms_that_sum_to_2020(EXAMPLE, 2),
            expected: 514579,
        })
        .part_1(|input| Answer {
            calculated: product_of_terms_that_sum_to_2020(input, 2),
            expected: 357504,
        })
        .example(|| Answer {
            calculated: product_of_terms_that_sum_to_2020(EXAMPLE, 3),
            expected: 241861950,
        })
        .part_2(|input| Answer {
            calculated: product_of_terms_that_sum_to_2020(input, 3),
            expected: 12747392,
        })
}
