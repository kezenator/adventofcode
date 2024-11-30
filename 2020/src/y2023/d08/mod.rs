use std::collections::HashMap;
use crate::support::*;
use itertools::*;

const EXAMPLE1: &str = include_str!("example1.txt");
const EXAMPLE2: &str = include_str!("example2.txt");
const EXAMPLE3: &str = include_str!("example3.txt");

struct Map
{
    instructions: String,
    network: HashMap<String, (String, String)>,
}

fn parse(input: &str) -> Map
{
    let groups = input_to_groups(input);
    assert!(groups.len() == 2);
    assert!(groups[0].len() == 1);

    let line_to_network_entry = |l: &String|
    {
        let (from, left, right) = scan(l)
            .until(" = (").parse::<String>()
            .until(", ").parse::<String>()
            .until(")").parse::<String>()
            .remaining().ignore();

        (from, (left, right))
    };

    let instructions = groups[0][0].clone();
    let network = groups[1].iter()
        .map(line_to_network_entry)
        .collect();

    Map { instructions, network }
}

fn steps_from_until<F: Fn(&str) -> bool>(map: &Map, from: &str, until: F) -> u64
{
    let num_instructions = map.instructions.len();
    let mut steps = 0;
    let mut location = from.to_string();

    while !until(&location)
    {
        let instruction = map.instructions.chars().nth(steps % num_instructions).unwrap();
        steps += 1;

        let network_steps = map.network.get(&location).unwrap();
        match instruction
        {
            'L' => location = network_steps.0.clone(),
            'R' => location = network_steps.1.clone(),
            _ => unreachable!(),
        }
    }

    steps as u64
}

fn part_1(input: &str) -> u64
{
    steps_from_until(&parse(input), "AAA", |l| l == "ZZZ")
}

fn part_2(input: &str) -> u64
{
    let map = parse(input);

    // Find all starting nodes

    let starting_nodes = map.network.iter()
        .map(|(from, _)| from.clone())
        .filter(|from| from.ends_with('A'))
        .collect_vec();
    
    //println!("{:?}", starting_nodes);

    // Find the length of each loop - for each starting
    // node to reach an ending node

    let loop_lengths = starting_nodes.iter()
        .map(|sn| steps_from_until(&map, sn, |l| l.ends_with('Z')))
        .collect_vec();
    
    //println!("{:?}", loop_lengths);

    // Find the least common multiple of all path length

    loop_lengths.iter()
        .fold(1, |a, b| num::lcm(a, *b))
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(8)
        .example(|| Answer {
            calculated: part_1(EXAMPLE1),
            expected: 2,
        })
        .example(|| Answer {
            calculated: part_1(EXAMPLE2),
            expected: 6,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 12643,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE3),
            expected: 6,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 13133452426987u64,
        })
}
