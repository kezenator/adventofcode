use itertools::Itertools;
use crate::support::*;
use crate::y2019::intcode::Intcode;

const INPUT: &str = include_str!("input.txt");

fn example(input: &str) -> String
{
    let mut comp = Intcode::new_from_input(input);
    comp.run_until_halt();

    comp.all_output().iter()
        .map(|i| i.to_string())
        .join(",")
}

fn run(input: i64) -> i64
{
    let mut comp = Intcode::new_from_input(INPUT);
    comp.input(input);
    comp.run_until_halt();
    comp.all_output()[0]
}

fn part_1() -> i64
{
    run(1)
}

fn part_2() -> i64
{
    run(2)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(9)
        .example(|| Answer { calculated: example("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"), expected: "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99", })
        .example(|| Answer { calculated: example("1102,34915192,34915192,7,4,7,99,0"), expected: "1219070632396864", })
        .example(|| Answer { calculated: example("104,1125899906842624,99"), expected: "1125899906842624", })
        .part_1(|| Answer { calculated: part_1(), expected: 3518157894i64, })
        .part_2(|| Answer { calculated: part_2(), expected: 80379, })
}
