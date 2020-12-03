use crate::support::*;
use super::intcode::Intcode;

const EXAMPLE: &str = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
const INPUT: &str = include_str!("input.txt");

fn run(prog: &str, input: i64) -> i64
{
    let mut comp = Intcode::new_from_input(prog);    comp.input(input);
    comp.run_until_halt();

    let output = comp.all_output();

    output[output.len() - 1]
}

pub fn puzzles(puzzles: &mut PuzzleSet)
{
    puzzles.register("y2019.d05.e1", || Answer {
        calculated: run(EXAMPLE, 7),
        expected: 999,
    });

    puzzles.register("y2019.d05.e2", || Answer {
        calculated: run(EXAMPLE, 8),
        expected: 1000,
    });

    puzzles.register("y2019.d05.e3", || Answer {
        calculated: run(EXAMPLE, 9),
        expected: 1001,
    });

    puzzles.register("y2019.d05.p1", || Answer {
        calculated: run(INPUT, 1),
        expected: 13787043,
    });

    puzzles.register("y2019.d05.p2", || Answer {
        calculated: run(INPUT, 5),
        expected: 3892695,
    });
}
