use crate::support::*;
use itertools::*;
use std::collections::HashMap;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(Copy, Clone)]
enum Instruction
{
    AddX(i64),
    Noop,
}

impl Instruction
{
    fn parse(input: &str) -> Vec<Instruction>
    {
        input_to_lines(input)
            .iter()
            .map(|l|
            {
                if l == "noop"
                {
                    Instruction::Noop
                }
                else
                {
                    let num = l.split(' ').skip(1).next().unwrap().parse().unwrap();
                    Instruction::AddX(num)
                }
            })
            .collect_vec()
    }
}

struct CommsDevice
{
    instructions: Vec<Instruction>,
    sprite_positions: HashMap<i64, i64>,
    image: CharGrid,
}

impl CommsDevice
{
    fn new(input: &str) -> CommsDevice
    {
        let instructions = Instruction::parse(input);
        CommsDevice
        {
            instructions,
            sprite_positions: HashMap::new(),
            image: CharGrid::new_from_fill(40, 6, '.'),
        }
    }

    fn run(&mut self)
    {
        let mut x = 1;
        let mut cycle = 1;

        for inst in self.instructions.clone()
        {
            match inst
            {
                Instruction::Noop =>
                {    
                    self.cycle(&mut cycle, x);
                }
                Instruction::AddX(num) =>
                {
                    self.cycle(&mut cycle, x);
                    self.cycle(&mut cycle, x);
                    x += num;
                },
            }
        }
    }

    fn cycle(&mut self, cycle: &mut i64, x: i64)
    {
        self.sprite_positions.insert(*cycle, x);

        let crtx = (*cycle - 1) % 40;
        let crty = (*cycle - 1) / 40;
        assert!(crty <= 5);

        if (x == (crtx - 1))
            || (x == crtx)
            || (x == (crtx + 1))
        {
            self.image.put_char(&Point::new(crtx, crty), '#');
        }

        *cycle += 1;
    }

    fn signal_strength(&mut self, cycle: i64) -> i64
    {
        *self.sprite_positions.get(&cycle).unwrap() * cycle
    }
}

fn part_1(input: &str) -> i64
{
    let mut device = CommsDevice::new(input);
    device.run();

    (0..6)
        .map(|i| 20 + 40 * i)
        .map(|cycle| device.signal_strength(cycle))
        .sum()
}

fn part_2(input: &str) -> String
{
    let mut device = CommsDevice::new(input);
    device.run();

    device.image.to_string()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(10)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 13140,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 14320,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....",
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: "###...##..###..###..#..#..##..###....##.\n#..#.#..#.#..#.#..#.#.#..#..#.#..#....#.\n#..#.#....#..#.###..##...#..#.#..#....#.\n###..#....###..#..#.#.#..####.###.....#.\n#....#..#.#....#..#.#.#..#..#.#....#..#.\n#.....##..#....###..#..#.#..#.#.....##..",
        })
}
