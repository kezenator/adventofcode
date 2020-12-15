use crate::support::*;
use crate::y2019::intcode::Intcode;

const INPUT: &str = include_str!("input.txt");

fn run(part_2: bool) -> i64
{
    let mut computers: Vec<Intcode> = Vec::new();

    for addr in 0..50
    {
        computers.push(Intcode::new_from_input(INPUT));
        computers.last_mut().unwrap().input(addr);
    }

    let mut nat: Option<(i64, i64)> = None;
    let mut nat_last_y: Option<i64> = None;

    loop
    {
        let mut idle_count = 0;
        let mut any_output = false;

        for addr in 0..50
        {
            if computers[addr].is_input_buffer_empty()
            {
                idle_count += 1;
                computers[addr].input(-1);
            }

            let _ = computers[addr].run_until_halt_or_input_required();

            while computers[addr].output_len() >= 3
            {
                any_output = true;

                let other_addr = computers[addr].pop_output() as usize;
                let x = computers[addr].pop_output();
                let y = computers[addr].pop_output();

                if other_addr == 255
                {
                    if !part_2
                    {
                        return y;
                    }
                    else
                    {
                        nat = Some((x, y));
                    }
                }
                else
                {
                    computers[other_addr].input(x);
                    computers[other_addr].input(y);
                }
            }
        }

        if (idle_count == 50) && !any_output
        {
            let nat_val = nat.unwrap();
            nat = None;

            if let Some(last_y) = nat_last_y
            {
                if last_y == nat_val.1
                {
                    return last_y;
                }
            }

            computers[0].input(nat_val.0);
            computers[0].input(nat_val.1);

            nat_last_y = Some(nat_val.1);
        }
    }
}

fn part_1() -> i64
{
    run(false)
}

fn part_2() -> i64
{
    run(true)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(23)
        .part_1(|| Answer { calculated: part_1(), expected: 24555, })
        .part_2(|| Answer { calculated: part_2(), expected: 19463, })
}
