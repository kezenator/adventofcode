use crate::support::*;
use std::collections::{ VecDeque, HashMap };

const EXAMPLE: &str = "Player 1 starting position: 4\nPlayer 2 starting position: 8";
const INPUT: &str = "Player 1 starting position: 7\nPlayer 2 starting position: 5";

struct DeterministicDice
{
    num_rolls: usize,
}

impl DeterministicDice
{
    fn new() -> Self
    {
        DeterministicDice { num_rolls: 0 }
    }

    fn roll(&mut self) -> usize
    {
        self.num_rolls += 1;
        self.num_rolls
    }

    fn get_num_rols(&self) -> usize
    {
        self.num_rolls
    }
}

struct State
{
    pos: usize,
    score: usize,
    turns: usize,
}

impl State
{
    fn new(pos: usize, turns: usize) -> Self
    {
        let score = 0;

        State { pos, score, turns }
    }

    fn advance(&self, roll: usize) -> Self
    {
        let pos = 1 + ((self.pos - 1 + roll) % 10);
        let score = self.score + pos;
        let turns = self.turns + 2;

        State { pos, score, turns }
    }

    fn get_turns(&self) -> usize
    {
        self.turns
    }

    fn get_score(&self) -> usize
    {
        self.score
    }
}

fn parse_input(input: &str) -> (usize, usize)
{
    let lines = input_to_lines(input);
    let (s1,) = scan(&lines[0]).skip_str("Player 1 starting position: ").remaining().parse::<usize>();
    let (s2,) = scan(&lines[1]).skip_str("Player 2 starting position: ").remaining().parse::<usize>();

    (s1, s2)
}

fn part_1(input: &str) -> usize
{
    let input = parse_input(input);

    let mut p1 = State::new(input.0, 0);
    let mut p2 = State::new(input.1, 1);
    let mut dice = DeterministicDice::new();

    loop
    {
        p1 = p1.advance(dice.roll() + dice.roll() + dice.roll());

        if p1.get_score() >= 1000
        {
            return p2.get_score() * dice.get_num_rols();
        }

        p2 = p2.advance(dice.roll() + dice.roll() + dice.roll());

        if p2.get_score() >= 1000
        {
            return p1.get_score() * dice.get_num_rols();
        }
    }
}

fn calc_dirac_turns_score_to_num_ways(start: State) -> HashMap<(usize, usize), usize>
{
    // When rolling a dirac dice three times, there are seven outecomes
    // with a total of 27 ways to get there

    const ROLLS: [(usize, usize); 7] = [
        (3, 1),
        (4, 3),
        (5, 6),
        (6, 7),
        (7, 6),
        (8, 3),
        (9, 1),
    ];

    // Find the map from (num turns, score) (i.e. unique states)
    // to the number of ways to get to that state.

    let mut result = HashMap::new();
    let mut to_advance = VecDeque::new();

    to_advance.push_back((start, 1));

    while !to_advance.is_empty()
    {
        let (cur_state, cur_num_ways) = to_advance.pop_front().unwrap();

        for (roll, roll_num_ways) in ROLLS.iter()
        {
            let next_state = cur_state.advance(*roll);
            let next_num_ways = cur_num_ways * *roll_num_ways;

            *result.entry((next_state.get_turns(), next_state.get_score())).or_insert(0) += next_num_ways;

            if next_state.get_score() < 21
            {
                to_advance.push_back((next_state, next_num_ways));
            }
        }
    }

    result
}

fn calc_dirac_num_universes_wins_in(pa: &HashMap<(usize, usize), usize>, pb: &HashMap<(usize, usize), usize>) -> usize
{
    // a) Find all pa ways to win - i.e. score is 21 or more
    //    a.a)For each, find all outcomes for pb with 1 turn less and a non-winning score
    //    a.b) sum a) ways multipled by a.a) ways
    // b) Sum all
    pa.iter()
        .filter(|((_, pa_score), _)| *pa_score >= 21)
        .map(move |((pa_turns, _), pa_num_ways)|
        {
            *pa_num_ways *
            pb.iter()
                .filter(|((pb_turns, pb_score), _)| *pb_turns == (*pa_turns - 1) && *pb_score < 21)
                .map(|((_, _), pb_num_ways)| *pb_num_ways)
                .sum::<usize>()
        })
        .sum()
}

fn part_2(input: &str) -> usize
{
    let input = parse_input(input);
    
    let p1_details = calc_dirac_turns_score_to_num_ways(State::new(input.0, 0));
    let p2_details = calc_dirac_turns_score_to_num_ways(State::new(input.1, 1));

    usize::max(
        calc_dirac_num_universes_wins_in(&p1_details, &p2_details),
        calc_dirac_num_universes_wins_in(&p2_details, &p1_details))
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(21)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 739785,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 798147,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 444356092776315usize,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 809953813657517usize,
        })
}
