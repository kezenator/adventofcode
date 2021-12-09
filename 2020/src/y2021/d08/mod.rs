use crate::support::*;
use std::str::FromStr;
use std::collections::HashSet;

const SHORT_EXAMPLE: &str = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
const LONG_EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct SignalSet
{
    bits: u64,
}

impl FromStr for SignalSet
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        // Convert letters ('a', 'b', 'c', ...) into bits (0x01, 0x02, 0x04, ...)
        // and or them all together

        let bits = s.chars()
            .map(|c| 1u64 << ((c as u8) - ('a' as u8)))
            .fold(0, |a, b| a | b);

        Ok(SignalSet { bits })
    }
}

impl SignalSet
{
    fn zero() -> Self
    {
        SignalSet { bits: 0 }
    }

    fn len(&self) -> usize
    {
        self.bits.count_ones() as usize
    }

    fn num_signals_not_in(&self, other: SignalSet) -> usize
    {
        (self.bits & !other.bits).count_ones() as usize
    }
}

struct Entry
{
    obs: Vec<SignalSet>,
    values: Vec<SignalSet>,
}

impl FromStr for Entry
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let (obs, values) = scan(s)
            .until(" | ").parse_vec::<SignalSet>(" ")
            .remaining().parse_vec::<SignalSet>(" ");

        Ok(Entry { obs, values })
    }
}

impl Entry
{
    fn num_easy_values(&self) -> usize
    {
        // Number of values that are "easy" to decode
        // (i.e. they have a number of signals (2, 3, 4, 7)
        // which map to a unqiue digit (1, 7, 4, 8)

        self.values.iter()
            .filter(|&v|
            {
                match v.len()
                {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                }
            })
            .count()
    }

    fn decode_value(&self) -> usize
    {
        // We need a set of remaining observations,
        // and will build up the mapping from each digit to it's observation

        let mut remaining_obs = self.obs.iter().copied().collect::<HashSet<SignalSet>>();

        let mut digit_to_obs = vec![SignalSet::zero(); 10];

        // Solve for the "easy" (i.e. unique) lengths

        {
            let mut solve_easy = |digit, num_signals|
            {
                let result = remaining_obs.iter()
                    .filter(|ob| ob.len() == num_signals)
                    .next()
                    .unwrap().clone();

                remaining_obs.remove(&result);
                digit_to_obs[digit] = result;
            };

            solve_easy(1, 2); // '1' has 2 signals
            solve_easy(7, 3);
            solve_easy(4, 4);
            solve_easy(8, 7);
        }

        // Now, for the rest, we can find them
        // by searching for an observation with a known
        // length that is missing a signal from a previously
        // known digit

        {
            let mut solve_hard = |digit: usize, num_signals: usize, num_other_signals: usize, other_signals_not_used_by: usize|
            {
                let result = remaining_obs.iter()
                    .filter(|ob| ob.len() == num_signals)
                    .filter(|ob| num_other_signals == ob.num_signals_not_in(digit_to_obs[other_signals_not_used_by]))
                    .next()
                    .unwrap().clone();

                remaining_obs.remove(&result);
                digit_to_obs[digit] = result;
            };

            solve_hard(9, 6, 2, 4); // '9' has 6 signals, and has 2 not used by '4'
            solve_hard(0, 6, 4, 1); // '0' has 6 signals, and has 4 not used by '1'
            solve_hard(6, 6, 5, 1); // '6' has 6 signals, and has 5 not used by '1'

            solve_hard(3, 5, 3, 1);
            solve_hard(2, 5, 3, 4);
            solve_hard(5, 5, 4, 1);
        }

        // Finally, convert each value to a digit
        // (i.e. position of an observation with the same signals in the digit_to_obs array)
        // and return all of them as a 4 digit value

        self.values.iter()
            .map(|&value| digit_to_obs.iter().position(|&ob| value == ob).unwrap())
            .fold(0, |value, digit| (10 * value) + digit)
    }
}

fn part_1(input: &str) -> usize
{
    input_to_lines_parsed::<Entry>(input).drain(..)
        .map(|e| e.num_easy_values())
        .sum()
}

fn part_2(input: &str) -> usize
{
    input_to_lines_parsed::<Entry>(input).drain(..)
        .map(|e| e.decode_value())
        .sum()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(8)
        .example(|| Answer {
            calculated: part_1(LONG_EXAMPLE),
            expected: 26,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 397,
        })
        .example(|| Answer {
            calculated: SHORT_EXAMPLE.parse::<Entry>().unwrap().decode_value(),
            expected: 5353,
        })
        .example(|| Answer {
            calculated: part_2(LONG_EXAMPLE),
            expected: 61229,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 1027422,
        })
}
