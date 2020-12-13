use crate::support::*;

const EXAMPLE: &str = "939\n7,13,x,x,59,x,31,19";
const INPUT: &str = include_str!("input.txt");

fn part_1(input: &str) -> u64
{
    let lines = input_to_lines(input);
    let start_ts = lines[0].parse::<u64>().unwrap();
    let ids = lines[1]
        .split(",")
        .filter(|i| *i != "x")
        .map(|i| i.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut cur_ts = start_ts;
    loop
    {
        for id in ids.iter()
        {
            if (cur_ts % id) == 0
            {
                return id * (cur_ts - start_ts);
            }
        }

        cur_ts += 1;
    }
}

fn combine_new_bus(first_ts: u64, period: u64, bus_id: u64, index: u64) -> (u64, u64)
{
    let mut cur_ts = first_ts;
    loop
    {
        if ((cur_ts + index) % bus_id) == 0
        {
            return (cur_ts, lcm(period, bus_id));
        }
        cur_ts += period;
    }
}

fn part_2_ans(line: &str) -> u64
{
    let ids = line.split(",").map(|i| if i == "x" { None } else { Some(i.parse::<u64>().unwrap()) }).collect::<Vec<Option<u64>>>();

    let mut first_ts = 0;
    let mut period = 1;

    for i in 0..ids.len()
    {
        if ids[i].is_some()
        {
            let combined = combine_new_bus(first_ts, period, ids[i].unwrap(), i as u64);
            first_ts = combined.0;
            period = combined.1;
        }
    }

    first_ts
}

fn part_2(input: &str) -> u64
{
    part_2_ans(&input_to_lines(input)[1])
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(13)
        .example(|| Answer { calculated: part_1(EXAMPLE), expected: 295, })
        .part_1(|| Answer { calculated: part_1(INPUT), expected: 3789, })
        .example(|| Answer { calculated: part_2(EXAMPLE), expected: 1068781, })
        .example(|| Answer { calculated: part_2_ans("17,x,13,19"), expected: 3417, })
        .example(|| Answer { calculated: part_2_ans("67,7,59,61"), expected: 754018, })
        .example(|| Answer { calculated: part_2_ans("67,x,7,59,61"), expected: 779210, })
        .example(|| Answer { calculated: part_2_ans("67,7,x,59,61"), expected: 1261476, })
        .example(|| Answer { calculated: part_2_ans("1789,37,47,1889"), expected: 1202161486, })
        .part_2(|| Answer { calculated: part_2(INPUT), expected: 667437230788118u64, })
}
