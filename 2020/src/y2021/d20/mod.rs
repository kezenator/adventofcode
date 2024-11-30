use crate::support::*;
use std::str::FromStr;
use itertools::Itertools;

const EXAMPLE: &str = include_str!("example.txt");

struct Image
{
    index: Vec<char>,
    grid: CharGrid
}

impl FromStr for Image
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let lines = input_to_lines(s);

        let index = lines[0].chars().collect::<Vec<char>>();

        if index.len() != (1 << 9)
        {
            return Err("Unexpected map length".to_owned());
        }

        let grid = CharGrid::new_from_input(&lines.iter().cloned().skip(2).join("\n"), '.');

        Ok(Image { index, grid })
    }
}

impl Image
{
    pub fn num_lit_pixels(&self) -> usize
    {
        // Can't return count of image with infinite lit pixels
        assert!(self.grid.get_default() == '.');

        self.grid.all_chars().iter()
            .filter(|&c| *c != '.')
            .count()
    }

    pub fn enhance(&self) -> Image
    {
        let new_default = match self.grid.get_default()
        {
            '.' =>
            {
                // Cur grid is filled with dark (0)
                // All bits will be off when finding index
                // for inifinite "edge" pixesls
                self.index[0]
            },
            '#' =>
            {
                // Cur infinite grid is filled with light (1)
                // All bits will be set when finding index
                // for infinite "edge" pixels
                self.index[511]
            },
            _ => unreachable!(),
        };

        let index = self.index.clone();
        let mut grid = CharGrid::new_from_fill(
            (self.grid.get_width() as usize) + 2,
            (self.grid.get_height() as usize) + 2,
            new_default);

        // NOTE:
        // 1) X offsets go down as bit weights are reversed to example
        // 2) Y offsets reversed due to different co-ordinate systems

        const OFFSETS: [(i64, i64); 9] = [
            (1, 1), (0, 1), (-1, 1),
            (1, 0), (0, 0), (-1, 0),
            (1, -1), (0, -1), (-1, -1),
        ];

        for y in -1..(self.grid.get_height() + 1)
        {
            for x in -1..(self.grid.get_width() + 1)
            {
                let dest = Point::new(x + 1, y + 1);

                let mut index = 0;

                for i in 0..9
                {
                    let ch = self.grid.get_char(&Point::new(x + OFFSETS[i].0, y + OFFSETS[i].1));

                    if ch == '#'
                    {
                        index |= 1 << i;
                    }
                }

                grid.put_char(&dest, self.index[index]);
            }
        }

        Image { index, grid }
    }
}

fn solve(input: &str, times: usize) -> usize
{
    let mut i = input.parse::<Image>().unwrap();

    for _ in 0..times
    {
        i = i.enhance();
    }

    i.num_lit_pixels()
}

fn part_1(input: &str) -> usize
{
    solve(input, 2)
}

fn part_2(input: &str) -> usize
{
    solve(input, 50)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(20)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 35,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 5647,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 3351,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 15653,
        })
}
