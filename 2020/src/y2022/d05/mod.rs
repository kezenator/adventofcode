use crate::support::*;
use itertools::*;

const EXAMPLE: &str = include_str!("example.txt");

type Stack = Vec<char>;
type State = Vec<Stack>;

struct Move
{
    count: usize,
    from: usize,
    to: usize,
}

struct Initial
{
    initial_state: State,
    moves: Vec<Move>,
}

fn parse(input: &str) -> Initial
{
    let parts = input_to_groups(input);

    let num_stacks = (parts[0].last().unwrap().len() + 1) / 4;
    let mut state: State = Vec::new();
    state.resize(num_stacks, Vec::new());

    for line in parts[0].iter().rev().skip(1)
    {
        let chars = line.chars().collect_vec();
        for i in 0..num_stacks
        {
            let pos = (i * 4) + 1;

            if chars[pos] != ' ' { state[i].push(chars[pos]); }
        }
    }

    let moves = parts[1]
        .iter()
        .map(|l|
        {
            scan(l)
                .skip_str("move ")
                .until(" from ").parse::<usize>()
                .until(" to ").parse::<usize>()
                .remaining().parse::<usize>()
        })
        .map(|t| Move { count: t.0, from: t.1 - 1, to: t.2 - 1})
        .collect_vec();

    Initial{ initial_state: state, moves: moves }
}

fn cratemover_9000(input: Initial) -> State
{
    let mut state = input.initial_state;

    for mv in input.moves
    {
        for _ in 0..mv.count
        {
            let ch = state[mv.from].pop().unwrap();
            state[mv.to].push(ch);
        }
    }
    state
}

fn cratemover_9001(input: Initial) -> State
{
    let mut state = input.initial_state;

    for mv in input.moves
    {
        let from_len = state[mv.from].len();
        let from_grab_range = (from_len - mv.count)..from_len;
        let mut from_grabbed = state[mv.from][from_grab_range].iter().copied().collect_vec();

        state[mv.from].resize(from_len - mv.count, ' ');
        state[mv.to].append(&mut from_grabbed);
    }
    state
}

fn tops(state: State) -> String
{
    state.iter()
        .map(|stack| stack.last().unwrap())
        .collect()
}

fn part_1(input: &str) -> String
{
    tops(cratemover_9000(parse(input)))
}

fn part_2(input: &str) -> String
{
    tops(cratemover_9001(parse(input)))
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(5)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: "CMZ",
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: "MQSHJMWNH",
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: "MCD",
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: "LLWJRBHVZ",
        })
}
