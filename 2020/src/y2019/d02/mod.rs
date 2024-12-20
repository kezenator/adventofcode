use crate::support::*;
use super::intcode::*;

fn mem_after_run(input: &str) -> String
{
    let mut comp = Intcode::new_from_input(input);
    comp.run_until_halt();
    comp.get_mem().iter().map(|i| i.to_string()).collect::<Vec<String>>().join(",")
}

fn part_1(input: &str) -> i64
{
    let mut comp = Intcode::new_from_input(input);
    comp.write_mem(1, 12);
    comp.write_mem(2, 2);
    comp.run_until_halt();

    comp.read_mem(0)
}

fn part_2(input: &str) -> i64
{
    for noun in 0..100
    {
        for verb in 0..100
        {
            let mut comp = Intcode::new_from_input(input);
            comp.write_mem(1, noun);
            comp.write_mem(2, verb);
            comp.run_until_halt();

            let output = comp.read_mem(0);

            if output == 19690720
            {
                return 100 * noun + verb;
            }
        }
    }
    unreachable!();
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(2)
        .example(|| Answer { calculated: mem_after_run("1,0,0,0,99"), expected: "2,0,0,0,99", })
        .example(|| Answer { calculated: mem_after_run("2,3,0,3,99"), expected: "2,3,0,6,99", })
        .example(|| Answer { calculated: mem_after_run("2,4,4,5,99,0"), expected: "2,4,4,5,99,9801", })
        .example(|| Answer { calculated: mem_after_run("1,1,1,4,99,5,6,0,99"), expected: "30,1,1,4,2,5,6,0,99", })
        .part_1(|input| Answer { calculated: part_1(input), expected: 4930687, })
        .part_2(|input| Answer { calculated: part_2(input), expected: 5335, })
}
