
use itertools::Itertools;
use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");

struct Schematics
{
    locks: Vec<Vec<i64>>,
    keys: Vec<Vec<i64>>,
}

impl Schematics
{
    fn new(input: &str) -> Self
    {
        let mut locks = Vec::new();
        let mut keys = Vec::new();

        for g in input_to_groups(input)
        {
            let grid = CharGrid::new_from_input(
                &g.into_iter().join("\n"),
                '.');
            assert!(grid.get_width() == 5);
            assert!(grid.get_height() == 7);

            if grid.get_char(&Point::new(0, 0)) == '#'
            {
                // lock
                locks.push((0..5).map(|x|
                    {
                        (0..7).filter(|y|
                        {
                            grid.get_char(&Point::new(x, *y)) == '.'
                        })
                        .next()
                        .unwrap()
                        - 1
                    })
                    .collect());
            }
            else // key
            {
                keys.push((0..5).map(|x|
                    {
                        5 -
                        (0..7).filter(|y|
                        {
                            grid.get_char(&Point::new(x, *y)) == '.'
                        })
                        .last()
                        .unwrap()
                    })
                    .collect());
            }
        }

        Schematics { locks, keys }
    }
}

fn key_fits_in_lock(key: &Vec<i64>, lock: &Vec<i64>) -> bool
{
    assert!(key.len() == lock.len());
    for (key_pin, lock_pin) in key.iter().zip(lock.iter())
    {
        let sum = *key_pin + *lock_pin;
        if sum > 5
        {
            return false;
        }
    }
    true
}

fn part_1(input: &str) -> usize
{
    let schematics = Schematics::new(input);

    let mut count = 0;

    for lock in schematics.locks.iter()
    {
        for key in schematics.keys.iter()
        {
            if key_fits_in_lock(key, lock)
            {
                count += 1;
            }
        }
    }

    count
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(25)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 3,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 3483,
        })
        .final_gift()
}
