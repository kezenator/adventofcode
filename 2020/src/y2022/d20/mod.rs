use crate::support::*;
use itertools::*;
use std::collections::HashSet;
use pathfinding::directed::bfs::*;

const EXAMPLE: &str = include_str!("example.txt");

struct EncryptedFile
{
    orig_numbers: Vec<i64>,
    shuffled_indexes: Vec<usize>,
}

impl EncryptedFile
{
    fn new(input: &str) -> Self
    {
        let orig_numbers = input_to_lines_parsed(input);

        let mut shuffled_indexes = (0..orig_numbers.len()).collect_vec();

        EncryptedFile { orig_numbers, shuffled_indexes }
    }

    fn decrypted_data(&self, key: i64, rounds: usize) -> Vec<i64>
    {
        let mut indexes = self.shuffled_indexes.clone();
        let len = self.orig_numbers.len();

        for r in 0..rounds
        {
            for i in 0..len
            {
                let cur_index_of_ith = indexes.iter()
                    .enumerate()
                    .filter(|(_,&v)| v == i)
                    .map(|(i,_)| i)
                    .next()
                    .unwrap();

                let orig_num = key * self.orig_numbers[i];

                let mut new_index_i64 = ((cur_index_of_ith as i64) + orig_num) % ((len - 1) as i64);
                if new_index_i64 < 0 { new_index_i64 += (len - 1) as i64; }

                let new_index = new_index_i64 as usize;

                let v = indexes[cur_index_of_ith];
                indexes.remove(cur_index_of_ith);
                indexes.insert(new_index, v);
            }
        }

        indexes.iter()
            .map(|&i| key * self.orig_numbers[i])
            .collect_vec()
    }

    fn sum_of_three_grove_coordinates(&self, key: i64, rounds: usize) -> i64
    {
        let data = self.decrypted_data(key, rounds);
        let len = data.len();

        let index_of_0 = data.iter()
            .enumerate()
            .filter(|(_,&v)| v == 0)
            .map(|(i,_)| i)
            .next()
            .unwrap();

        data[(index_of_0 + 1000) % len]
            + data[(index_of_0 + 2000) % len]
            + data[(index_of_0 + 3000) % len]
    }
}

fn part_1(input: &str) -> i64
{
    EncryptedFile::new(input)
        .sum_of_three_grove_coordinates(1, 1)
}

fn part_2(input: &str) -> i64
{
    EncryptedFile::new(input)
        .sum_of_three_grove_coordinates(811589153, 10)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(20)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 3,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 3466,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 1623178306i64,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 9995532008348i64,
        })
}
