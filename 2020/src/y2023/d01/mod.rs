use crate::support::*;
use itertools::*;

const EXAMPLE1: &str = include_str!("example1.txt");
const EXAMPLE2: &str = include_str!("example2.txt");

fn find_needle(needle: &str, haystack: &Vec<char>, start_index: usize) -> bool
{
    if start_index + needle.chars().count() > haystack.len()
    {
        return false;
    }

    for (n_index, n) in needle.chars().enumerate()
    {
        let h_index = start_index + n_index;
        if haystack[h_index] != n
        {
            return false;
        }
    }
    true
}

fn line_to_digits(line: &str, include_words: bool) -> Vec<u64>
{
    let chars = line.chars().collect_vec();
    let mut index = 0;
    let mut digits = Vec::new();
    'main_loop: while index < chars.len()
    {
        if let Some(digit) = chars[index].to_digit(10)
        {
            digits.push(digit as u64);
            index += 1;
        }
        else if include_words
        {
            let words = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
            for (w_index, &w) in words.iter().enumerate()
            {
                if find_needle(w, &chars, index)
                {
                    digits.push((w_index + 1) as u64);
                    index += 1;
                    continue 'main_loop;
                }
            }
            index += 1;
        }
        else
        {
            index += 1;
        }
    }
    digits
}

fn digits_to_calibration_value(digits: Vec<u64>) -> u64
{
    assert!(digits.len() >= 1);
    let tens = digits[0];
    
    let units = digits[digits.len() - 1];
    (10 * tens) + units
}

fn part_1(input: &str) -> u64
{
    input_to_lines(input)
        .iter()
        .map(|l| digits_to_calibration_value(line_to_digits(l, false)))
        .sum()
}

fn part_2(input: &str) -> u64
{
    input_to_lines(input)
        .iter()
        .map(|l| digits_to_calibration_value(line_to_digits(l, true)))
        .sum()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(1)
        .example(|| Answer {
            calculated: part_1(EXAMPLE1),
            expected: 142,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 55607,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE2),
            expected: 281,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 55291,
        })
}
