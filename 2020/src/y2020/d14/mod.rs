use std::collections::{HashMap, HashSet};
use crate::support::*;

const EXAMPLE_1: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0";
const EXAMPLE_2: &str = "mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1";

pub fn part_1(input: &str) -> u64
{
    let mut mem: HashMap::<u64, u64> = HashMap::new();
    let mut and_mask: u64 = (1 << 36) - 1;
    let mut or_mask: u64 = 0;

    for line in input_to_lines(input)
    {
        if line[0..7].to_owned() == "mask = "
        {
            and_mask = 0;
            or_mask = 0;

            for i in 0..36
            {
                let ch = line.chars().nth(7 + i).unwrap();
                let mask = 1 << (35 - i);

                match ch
                {
                    '0' =>
                    {
                        // Ignore
                    },
                    '1' =>
                    {
                        or_mask |= mask;
                    },
                    'X' =>
                    {
                        and_mask |= mask;
                    },
                    _ =>
                    {
                        unreachable!();
                    },
                }
            }
        }
        else
        {
            let (addr, val) = scan(&line)
                .skip(4)
                .until("] = ").parse::<u64>()
                .remaining().parse::<u64>();
            
            let store = (val & and_mask) | or_mask;

            mem.insert(addr, store);
        }
    }

    mem.iter()
        .map(|(_addr, val)| val)
        .sum()
}

fn get_part_2_addrs(addr: u64, mask: &str) -> HashSet<u64>
{
    let mut result = HashSet::new();
    result.insert(addr);

    for index in 0..36
    {
        let bit_mask = 1u64 << (35 - index);

        match mask.chars().nth(index).unwrap()
        {
            '0' =>
            {
                // Unchanged
            },
            '1' =>
            {
                // Set

                let mut new = HashSet::new();
                for a in result
                {
                    new.insert(a | bit_mask);
                }
                result = new;
            },
            'X' =>
            {
                // Floating - set and unset

                let mut new = HashSet::new();
                for a in result
                {
                    new.insert(a | bit_mask);
                    new.insert(a & (!bit_mask & 0xFFFFFFFFF));
                }
                result = new;
            },
            _ =>
            {
                unreachable!();
            },
        }
    }

    result
}

fn part_2(input: &str) -> u64
{
    let mut mem: HashMap::<u64, u64> = HashMap::new();
    let mut mask = "".to_owned();

    for line in input_to_lines(input)
    {
        if line[0..7].to_owned() == "mask = "
        {
            mask = line[7..].to_owned();
        }
        else
        {
            let (addr, val) = scan(&line)
                .skip(4)
                .until("] = ").parse::<u64>()
                .remaining().parse::<u64>();

            for addr in get_part_2_addrs(addr, &mask)
            {
                mem.insert(addr, val);
            }
        }
    }

    mem.iter()
        .map(|(_addr, val)| val)
        .sum()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(14)
        .example(|| Answer { calculated: part_1(EXAMPLE_1), expected: 165, })
        .part_1(|input| Answer { calculated: part_1(input), expected: 17481577045893u64, })
        .example(|| Answer { calculated: part_2(EXAMPLE_2), expected: 208, })
        .part_2(|input| Answer { calculated: part_2(input), expected: 4160009892257u64, })
}
