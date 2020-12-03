use itertools::Itertools;
use crate::support::*;
use super::intcode::Intcode;

const INPUT: &str = include_str!("input.txt");

pub fn run(input: &str, phases: Vec<i64>) -> i64
{
    let mut comps = Vec::new();
    for ph in phases.iter()
    {
        comps.push(Intcode::new_from_input(input));
        comps.last_mut().unwrap().input(*ph);
    }

    let mut output = 0;

    while !comps[0].is_halted()
    {
        comps[0].input(output);
        comps[0].run_until_halt_or_input_required();

        for i in 1..phases.len()
        {
            let partial = comps[i - 1].pop_output();
            comps[i].input(partial);
            comps[i].run_until_halt_or_input_required();
        }

        output = comps.last_mut().unwrap().pop_output();
    }

    output
}

pub fn part_1(input: &str) -> i64
{
    (0..5).permutations(5)
        .map(|phases| run(input, phases))
        .max().unwrap()
}

pub fn part_2(input: &str) -> i64
{
    (5..10).permutations(5)
        .map(|phases| run(input, phases))
        .max().unwrap()
}

pub fn puzzles(puzzles: &mut PuzzleSet)
{
    puzzles.register("y2019.d07.e1", || Answer {
        calculated: part_1("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
        expected: 43210,
    });

    puzzles.register("y2019.d07.e2", || Answer {
        calculated: part_1("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"),
        expected: 54321,
    });

    puzzles.register("y2019.d07.e3", || Answer {
        calculated: part_1("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"),
        expected: 65210,
    });

    puzzles.register("y2019.d07.e4", || Answer {
        calculated: part_2("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"),
        expected: 139629729,
    });

    puzzles.register("y2019.d07.e5", || Answer {
        calculated: part_2("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"),
        expected: 18216,
    });

    puzzles.register("y2019.d07.p1", || Answer {
        calculated: part_1(INPUT),
        expected: 21760,
    });

    puzzles.register("y2019.d07.p2", || Answer {
        calculated: part_2(INPUT),
        expected: 69816958,
    });
}
