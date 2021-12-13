use crate::support::*;
use std::collections::{ HashSet, VecDeque };

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

struct OctoGrid
{
    cells: CharGrid,
    num_flashes: usize,
}

impl OctoGrid
{
    fn new(input: &str) -> Self
    {
        OctoGrid
        {
            cells: CharGrid::new_from_input(input, '0'),
            num_flashes: 0,
        }
    }

    fn get_num_octos(&self) -> usize
    {
        self.cells.all_points().len()
    }

    fn step(&mut self) -> usize
    {
        let mut to_inc = VecDeque::with_capacity(self.cells.get_width() as usize * self.cells.get_height() as usize);

        for p in self.cells.all_points()
        {
            to_inc.push_back(p);
        }

        let mut flashed = HashSet::new();

        while !to_inc.is_empty()
        {
            let p = to_inc.pop_front().unwrap();

            let mut c = self.cells.get_char(&p);
            c = ((c as u8) + 1) as char;
            if c > '9'
            {
                if flashed.insert(p.clone())
                {
                    for d in Point::directions_8()
                    {
                        to_inc.push_back(p + d);
                    }
                }
            }

            self.cells.put_char(&p, c);
        }

        let result = flashed.len();
        self.num_flashes += flashed.len();

        for p in flashed
        {
            self.cells.put_char(&p, '0');
        }

        result
    }

    fn get_num_flashes(&self) -> usize
    {
        self.num_flashes
    }
}


fn part_1(input: &str) -> usize
{
    let mut grid = OctoGrid::new(input);

    for _ in 0..100
    {
        grid.step();
    }

    grid.get_num_flashes()
}

fn part_2(input: &str) -> usize
{
    let mut grid = OctoGrid::new(input);
    let size = grid.get_num_octos();

    let mut steps = 0;
    
    loop
    {
        steps += 1;
        if grid.step() == size
        {
            return steps;
        }
    }
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(11)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 1656,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 1652,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 195,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 220,
        })
}
