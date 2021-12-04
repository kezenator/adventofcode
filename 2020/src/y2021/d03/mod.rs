use crate::support::*;

const EXAMPLE: &str = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";
const INPUT: &str = include_str!("input.txt");

fn filter_and_collect<F>(input: &str, keep_non_matching: bool, filter: F) -> usize
    where F: Fn(usize, usize) -> char
{
    let mut lines = input_to_lines(input).iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let num_digits = lines[0].len();

    let mut result = String::new();

    for index in 0..num_digits
    {
        let digits = lines.iter()
            .map(|l| l[index])
            .collect::<Vec<char>>();

        let count_0 = digits.iter().filter(|&d| *d == '0').count();
        let count_1 = digits.iter().filter(|&d| *d == '1').count();

        let next_digit = filter(count_0, count_1);

        result.push(next_digit);

        if !keep_non_matching
        {
            lines = lines.drain(..)
                .filter(|s| s[index] == next_digit)
                .collect::<Vec<_>>();

            if lines.len() == 1
            {
                // Only one left - use this value
                result = lines[0].drain(..).collect::<String>();
                break;
            }
        }
    }

    return usize::from_str_radix(&result, 2).unwrap();
}

fn find_gamma(input: &str) -> usize
{
    filter_and_collect(
        input,
        true,
        |count_0: usize, count_1: usize| if count_0 > count_1 { '0' } else { '1' })
}

fn find_epsilon(input: &str) -> usize
{
    filter_and_collect(
        input,
        true,
        |count_0: usize, count_1: usize| if count_0 > count_1 { '1' } else { '0' })
}

fn find_oxygen_generator_rating(input: &str) -> usize
{
    filter_and_collect(
        input,
        false,
        |count_0: usize, count_1: usize| if count_0 > count_1 { '0' } else { '1' })
}

fn find_co2_scrubber_rating(input: &str) -> usize
{
    filter_and_collect(
        input,
        false,
        |count_0: usize, count_1: usize| if count_0 > count_1 { '1' } else { '0' })
}

fn part_1(input: &str) -> usize
{
    find_gamma(input) * find_epsilon(input)
}

fn part_2(input: &str) -> usize
{
    find_oxygen_generator_rating(input) * find_co2_scrubber_rating(input)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(3)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 198,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 4138664,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 230,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 4273224,
        })
}
