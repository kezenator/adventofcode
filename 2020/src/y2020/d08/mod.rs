use std::collections::HashSet;
use crate::support::*;

const EXAMPLE: &str = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6\n";
const INPUT: &str = include_str!("input.txt");

pub enum ConsoleResult
{
    InfiniteLoop(i64),
    Terminated(i64),
}

pub struct Console
{
    pc: usize,
    acc: i64,
    inst: Vec<(String, i64)>,
}

impl Console
{
    pub fn new(input: &str) -> Self
    {
        Console
        {
            pc: 0,
            acc: 0,
            inst: input_to_lines(input)
                .iter()
                .map(|line| 
                    scan(&line)
                        .until(" ").parse::<String>()
                        .remaining().parse::<i64>())
                .collect(),
        }
    }

    pub fn uncorrupt_instruction(&mut self, index: usize)
    {
        let (inst, arg) = self.inst[index].clone();

        let fixed = match inst.as_str()
        {
            "nop" => "jmp",
            "acc" => "acc",
            "jmp" => "nop",
            _ => unreachable!(),
        };

        self.inst[index] = (fixed.to_owned(), arg);        
    }

    pub fn run(&mut self) -> ConsoleResult
    {
        let mut seen_pc = HashSet::new();

        loop
        {
            if !seen_pc.insert(self.pc)
            {
                return ConsoleResult::InfiniteLoop(self.acc);
            }

            if self.pc >= self.inst.len()
            {
                return ConsoleResult::Terminated(self.acc);
            }

            let (inst, arg) = &self.inst[self.pc];

            match inst.as_str()
            {
                "nop" =>
                {
                    self.pc += 1;
                },
                "acc" =>
                {
                    self.acc += arg;
                    self.pc += 1;
                },
                "jmp" =>
                {
                    self.pc = ((self.pc as i64) + arg) as usize;
                },
                _ => unreachable!(),
            }
        }
    }
}

fn part_1(input: &str) -> i64
{
    match Console::new(input).run()
    {
        ConsoleResult::InfiniteLoop(result) => result,
        ConsoleResult::Terminated(_) => unreachable!(),
    }
}

fn part_2(input: &str) -> i64
{
    for i in 0..input_to_lines(input).len()
    {
        let mut console = Console::new(input);
        console.uncorrupt_instruction(i);

        match console.run()
        {
            ConsoleResult::InfiniteLoop(_) => (),
            ConsoleResult::Terminated(result) => return result,
        }
    }
    unreachable!();
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(8)
        .example(|| Answer { calculated: part_1(EXAMPLE), expected: 5, })
        .part_1(|| Answer { calculated: part_1(INPUT), expected: 2003, })
        .example(|| Answer { calculated: part_2(EXAMPLE), expected: 8, })
        .part_2(|| Answer { calculated: part_2(INPUT), expected: 1984, })
}
