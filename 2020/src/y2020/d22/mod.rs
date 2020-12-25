use std::collections::{HashSet, VecDeque};
use crate::support::*;

const EXAMPLE: &str = "Player 1:\n9\n2\n6\n3\n1\n\nPlayer 2:\n5\n8\n4\n7\n10";
const INPUT: &str = include_str!("input.txt");


pub fn part_1(input: &str) -> u64
{
    let groups = input_to_groups(input);

    let mut p1 = groups[0][1..].iter().map(|s| s.parse::<u64>().unwrap()).collect::<VecDeque<u64>>();
    let mut p2 = groups[1][1..].iter().map(|s| s.parse::<u64>().unwrap()).collect::<VecDeque<u64>>();

    while !p1.is_empty() && !p2.is_empty()
    {
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        if c1 > c2
        {
            p1.push_back(std::cmp::max(c1, c2));
            p1.push_back(std::cmp::min(c1, c2));
        }
        else
        {
            p2.push_back(std::cmp::max(c1, c2));
            p2.push_back(std::cmp::min(c1, c2));
        }

    }

    let mut winning_hand = p1;
    winning_hand.extend(p2);

    winning_hand.into_iter()
        .rev()
        .enumerate()
        .map(|(i, c)| ((i + 1) as u64) * c)
        .sum()
}

fn recursive_cards(input: &(VecDeque<usize>, VecDeque<usize>), mem: &Memorized<(VecDeque<usize>, VecDeque<usize>), (bool, Vec<usize>)>) -> (bool, Vec<usize>)
{
    /*println!();
    println!("INPUT: {:?}", input);
    println!("STARTED");*/

    let mut p1 = input.0.clone();
    let mut p2 = input.1.clone();

    let mut prev_states = HashSet::new();

    while !p1.is_empty() && !p2.is_empty()
    {
        if prev_states.contains(&(p1.clone(), p2.clone()))
        {
            break;
        }

        prev_states.insert((p1.clone(), p2.clone()));

        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        if c1 <= p1.len() && c2 <= p2.len()
        {
            let p1_sub = p1.iter().take(c1).cloned().collect();
            let p2_sub = p2.iter().take(c2).cloned().collect();
            
            /*println!();
            println!("INPUT: {:?}", input);
            println!("CALLING SUB");
            println!("{} {:?} {} {:?}", c1, p1_sub, c2, p2_sub);*/

            let (p1_sub_winner, _) = mem.get(&(p1_sub, p2_sub));

            if p1_sub_winner
            {
                p1.push_back(c1);
                p1.push_back(c2);
            }
            else
            {
                p2.push_back(c2);
                p2.push_back(c1);
            }
        }
        else if c1 > c2
        {
            p1.push_back(std::cmp::max(c1, c2));
            p1.push_back(std::cmp::min(c1, c2));
        }
        else
        {
            p2.push_back(std::cmp::max(c1, c2));
            p2.push_back(std::cmp::min(c1, c2));
        }

        /*println!();
        println!("INPUT: {:?}", input);
        println!("COMPLETED TURN");
        println!("{:?}", p1);
        println!("{:?}", p2);*/
    }

    let p1_winner = !p1.is_empty();

    let mut winning_hand = p1.into_iter().collect::<Vec<usize>>();
    winning_hand.extend(p2);

    /*println!();
    println!("INPUT: {:?}", input);
    println!("RESULT");
    println!("{:?} {:?}", p1_winner, winning_hand);*/

    (p1_winner, winning_hand)
}

fn part_2(input: &str) -> usize
{
    let groups = input_to_groups(input);

    let p1 = groups[0][1..].iter().map(|s| s.parse::<usize>().unwrap()).collect::<VecDeque<usize>>();
    let p2 = groups[1][1..].iter().map(|s| s.parse::<usize>().unwrap()).collect::<VecDeque<usize>>();

    let mem = Memorized::new(&recursive_cards).debug(false);

    let (_p1_winner, final_deck) = mem.get(&(p1, p2));

    final_deck.into_iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (i + 1) * c)
        .sum()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(22)
        .example(|| Answer { calculated: part_1(EXAMPLE), expected: 306, })
        .part_1(|| Answer { calculated: part_1(INPUT), expected: 34127, })
        .example(|| Answer { calculated: part_2(EXAMPLE), expected: 291, })
        .part_2(|| Answer { calculated: part_2(INPUT), expected: 32054, })
}
