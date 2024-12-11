
use itertools::*;
use crate::support::*;
use std::{collections::BTreeMap, str::FromStr};

const EXAMPLE: &str = include_str!("example.txt");

#[derive(Clone)]
struct BlockRun
{
    file_id: usize,
    length: usize,
}

struct DiskMap
{
    blocks: BTreeMap<usize, BlockRun>,
}

impl FromStr for DiskMap
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let mut blocks = BTreeMap::new();
        let mut index = 0;
        let mut file_id = 0;

        for chunk in s.chars().chunks(2).into_iter().map(|c| c.collect_vec())
        {
            let length = chunk[0].to_string().parse().unwrap();
            
            blocks.insert(index, BlockRun{ file_id, length });
            index += length;
            file_id += 1;

            if chunk.len() > 1
            {
                index += chunk[1].to_string().parse::<usize>().unwrap();
            }
        }

        Ok(DiskMap{ blocks })
    }
}


impl DiskMap
{
    fn first_free_range(&self, min_size: usize) -> Option<(usize, usize)>
    {
        if let Some((first_index, _)) = self.blocks.first_key_value()
        {
            if *first_index >= min_size
            {
                return Some((0, *first_index));
            }
        }
        for ((a_index, a_block), (b_index, _b_block)) in self.blocks.iter().tuple_windows()
        {
            let a_upper = a_index + a_block.length;
            let free_len = b_index - a_upper;
            if free_len >= min_size
            {
                return Some((a_upper, free_len));
            }
        }
        return None;
    }

    fn defragment(&mut self, split_files: bool)
    {
        if split_files // Part 1
        {
            loop
            {
                match self.first_free_range(1)
                {
                    Some((free_index, free_len)) =>
                    {
                        let (last_index, last_block) = self.blocks.pop_last().unwrap();

                        if last_block.length <= free_len
                        {
                            // Move to the start
                            self.blocks.insert(free_index, last_block);
                        }
                        else
                        {
                            // Split the block
                            self.blocks.insert(free_index, BlockRun { file_id: last_block.file_id, length: free_len});
                            self.blocks.insert(last_index, BlockRun{ file_id: last_block.file_id, length: last_block.length - free_len});
                        }
                    },
                    None =>
                    {
                        // No more free space
                        return;
                    }
                }
            }
        }
        else // Don't split files - Part 2
        {
            for file_entry in self.blocks.clone().into_iter().rev()
            {
                if let Some((free_index, _)) = self.first_free_range(file_entry.1.length)
                {
                    if free_index < file_entry.0
                    {
                        self.blocks.remove(&file_entry.0);
                        self.blocks.insert(free_index, file_entry.1);
                    }
                }
            }
        }
    }

    fn checksum(&self) -> usize
    {
        let mut result = 0;

        for (index, block) in self.blocks.iter()
        {
            for i in 0..block.length
            {
                result += (index + i) * block.file_id;
            }
        }

        result
    }
}

fn part_1(input: &str) -> usize
{
    let mut map: DiskMap = input.parse().unwrap();
    map.defragment(true);
    map.checksum()
}

fn part_2(input: &str) -> usize
{
    let mut map: DiskMap = input.parse().unwrap();
    map.defragment(false);
    map.checksum()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(9)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 1928,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 6432869891895usize,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 2858,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 6467290479134usize,
        })
}
