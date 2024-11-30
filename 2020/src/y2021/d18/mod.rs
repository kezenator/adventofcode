use crate::support::*;
use std::str::FromStr;
use std::ops::{Add, Range};
use itertools::Itertools;

const EXAMPLE: &str = include_str!("example.txt");

#[derive(Clone, Copy, Debug)]
struct SnailfishEntry
{
    pub num_pushes: usize,
    pub digit: u64,
    pub num_pops: usize,
}

#[derive(Clone)]
struct SnailfishNumber
{
    entries: Vec<SnailfishEntry>,
}

impl SnailfishNumber
{
    pub fn magnitude(&self) -> u64
    {
        self.magnitude_range(0..self.entries.len(), 1)
    }

    fn magnitude_range(&self, range: Range<usize>, level: usize) -> u64
    {
        if range.start + 1 == range.end
        {
            // It's just one number
            return self.entries[range.start].digit;
        }

        let (left, right) = self.find_halves(range, level);

        3 * self.magnitude_range(left, level + 1) + 2 * self.magnitude_range(right, level + 1)
    }

    fn find_halves(&self, range: Range<usize>, level: usize) -> (Range<usize>, Range<usize>)
    {
        let start = range.start;
        let end = range.end;

        let mut depth = 0;
        for i in 0..self.entries.len()
        {
            depth += self.entries[i].num_pushes;
            depth -= self.entries[i].num_pops;

            if (depth == level) && (i >= start)
            {
                assert!(i < end);

                return (start..(i + 1), (i + 1)..end);
            }
        }
        unreachable!();
    }

    fn reduce(&mut self)
    {
        loop
        {
            if self.try_explode_step()
            {
                continue;
            }
            else if self.try_split_step()
            {
                continue;
            }
            else
            {
                return;
            }
        }
    }

    fn try_explode_step(&mut self) -> bool
    {
        let mut depth = 0;
        for i in 0..self.entries.len()
        {
            depth += self.entries[i].num_pushes;

            if (depth >= 5)
                && ((i + 1) < self.entries.len())
                && (self.entries[i].num_pops == 0)
                && (self.entries[i + 1].num_pushes == 0)
            {
                if i > 0
                {
                    self.entries[i - 1].digit += self.entries[i].digit;
                }

                if (i + 2) < self.entries.len()
                {
                    self.entries[i + 2].digit += self.entries[i + 1].digit;
                }

                self.entries[i].digit = 0;
                self.entries[i].num_pushes -= 1;
                self.entries[i].num_pops = self.entries[i + 1].num_pops - 1;
                self.entries.remove(i + 1);

                return true;
            }

            depth -= self.entries[i].num_pops;
        }
        false
    }

    fn try_split_step(&mut self) -> bool
    {
        if let Some(i) = self.entries.iter().position(|e| e.digit >= 10)
        {
            let left = self.entries[i].digit / 2;
            let right = self.entries[i].digit - left;
            let orig_pops = self.entries[i].num_pops;

            self.entries[i].num_pushes += 1;
            self.entries[i].digit = left;
            self.entries[i].num_pops = 0;

            self.entries.insert(
                i + 1,
                SnailfishEntry
                {
                    num_pushes: 0,
                    digit: right,
                    num_pops: orig_pops + 1,
                });

            return true;
        }
        false
    }
}

impl FromStr for SnailfishNumber
{
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err>
    {
        let mut entries = Vec::new();

        let mut num_pushes = 0;
        let chars: Vec<char> = input.chars().collect();

        let mut index = 0;
        let len = chars.len();
        while index < len
        {
            let ch = chars[index];
            if ch == '['
            {
                num_pushes += 1;
                index += 1;
            }
            else if (ch >= '0') && (ch <= '9')
            {
                let digit = (ch as u64) - ('0' as u64);
                index += 1;

                let mut num_pops = 0;
                while (index < len) && (chars[index] == ']')
                {
                    num_pops += 1;
                    index += 1;
                }

                entries.push(SnailfishEntry{ num_pushes, digit, num_pops });
                
                num_pushes = 0;
            }
            else if ch == ','
            {
                // Ignore
                index += 1;
            }
            else
            {
                unreachable!();
            }
        }

        Ok(SnailfishNumber{ entries })
    }
}

impl ToString for SnailfishNumber
{
    fn to_string(&self) -> String
    {
        let mut result = String::new();

        for (i, entry) in self.entries.iter().enumerate()
        {
            if i != 0
            {
                result.push(',');
            }

            for _ in 0..entry.num_pushes
            {
                result.push('[');
            }

            result.push_str(&entry.digit.to_string());

            for _ in 0..entry.num_pops
            {
                result.push(']');
            }
        }

        result
    }
}

impl Add for SnailfishNumber
{
    type Output = SnailfishNumber;

    fn add(self, other: SnailfishNumber) -> SnailfishNumber
    {
        let new_len = self.entries.len() + other.entries.len();

        let mut entries = Vec::with_capacity(new_len);
        entries.extend_from_slice(&self.entries);
        entries.extend_from_slice(&other.entries);

        entries[0].num_pushes += 1;
        entries[new_len - 1].num_pops += 1;

        let mut result = SnailfishNumber{ entries };
        result.reduce();
        result
    }
}

fn part_1(input: &str) -> u64
{
    input_to_lines_parsed::<SnailfishNumber>(input)
        .drain(..)
        .reduce(|a, b| a + b)
        .unwrap()
        .magnitude()
}

fn part_2(input: &str) -> u64
{
    input_to_lines_parsed::<SnailfishNumber>(input)
        .iter()
        .combinations(2)
        .map(|pairs|
        {
            u64::max(
                (pairs[0].clone() + pairs[1].clone()).magnitude(),
                (pairs[1].clone() + pairs[0].clone()).magnitude())
        })
        .max()
        .unwrap()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(18)
        .example(|| Answer {
            calculated: "[[1,2],[[3,4],5]]".parse::<SnailfishNumber>().unwrap().magnitude(),
            expected: 143,
        })
        .example(|| Answer {
            calculated: "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse::<SnailfishNumber>().unwrap().magnitude(),
            expected: 1384,
        })
        .example(|| Answer {
            calculated: "[[[[1,1],[2,2]],[3,3]],[4,4]]".parse::<SnailfishNumber>().unwrap().magnitude(),
            expected: 445,
        })
        .example(|| Answer {
            calculated: "[[[[3,0],[5,3]],[4,4]],[5,5]]".parse::<SnailfishNumber>().unwrap().magnitude(),
            expected: 791,
        })
        .example(|| Answer {
            calculated: "[[[[5,0],[7,4]],[5,5]],[6,6]]".parse::<SnailfishNumber>().unwrap().magnitude(),
            expected: 1137,
        })
        .example(|| Answer {
            calculated: "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".parse::<SnailfishNumber>().unwrap().magnitude(),
            expected: 3488,
        })
        .example(|| Answer {
            calculated: ("[[[[4,3],4],4],[7,[[8,4],9]]]".parse::<SnailfishNumber>().unwrap()
                            + "[1,1]".parse::<SnailfishNumber>().unwrap()).to_string(),
            expected: "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        })
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 4140,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 3763,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 3993,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 4664,
        })
}
