use std::collections::HashSet;
use crate::support::*;

const INPUT: &str = include_str!("input.txt");

#[derive(PartialEq, Eq, Hash, Clone)]
struct Bugs
{
    image: CharGrid,
}

impl Bugs
{
    pub fn new() -> Self
    {
        Bugs
        {
            image: CharGrid::new_from_input(INPUT, '.'),
        }
    }

    pub fn next(&self) -> Self
    {
        let mut image = self.image.clone();

        for pos in image.all_points()
        {
            let num_bugs = Point::directions_4().iter()
                .map(|&d| self.image.get_char(&(pos + d)))
                .filter(|&c| c == '#')
                .count();

            if self.image.get_char(&pos) == '#'
            {
                if num_bugs != 1
                {
                    image.put_char(&pos, '.');
                }
            }
            else // cur empty
            {
                if num_bugs == 1 || num_bugs == 2
                {
                    image.put_char(&pos, '#');
                }
            }
        }

        Bugs { image }
    }

    pub fn biodiversity(&self) -> u64
    {
        let mut result: u64 = 0;
        let mut mask: u64 = 1;

        for y in 0..self.image.get_height()
        {
            for x in 0..self.image.get_width()
            {
                if '#' == self.image.get_char(&Point::new(x, y))
                {
                    result |= mask;
                }
                mask <<= 1;
            }
        }

        result
    }
}

fn part_1() -> u64
{
    let mut seen = HashSet::new();
    let mut state = Bugs::new();

    loop
    {
        if !seen.insert(state.clone())
        {
            return state.biodiversity();
        }
        
        state = state.next();
    }
}

fn part_2() -> String
{
    "not complete".to_owned()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(24)
        .part_1(|| Answer { calculated: part_1(), expected: 17863741, })
        .part_2(|| Answer { calculated: part_2(), expected: "not complete", })
}
