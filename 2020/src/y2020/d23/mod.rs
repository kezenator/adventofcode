use itertools::Itertools;
use crate::support::*;

const EXAMPLE: &str = "389125467";
const INPUT: &str = "974618352";

fn simulate(input: &str, num_cups: usize, num_moves: usize) -> Vec<usize>
{
    let mut cups = input.chars().map(|c| (c as usize) - ('0' as usize)).collect::<Vec<usize>>();
    let mut new_cups = Vec::<usize>::new();

    cups.reserve(num_cups);
    new_cups.reserve(num_cups);

    for i in 10..(num_cups + 1)
    {
        cups.push(i);
    }

    for mv in 0..num_moves
    {
        if mv % 10000 == 0
        {
            //println!();
            println!("{}", mv);
            //println!("{:?}", cups);
        }

        let cur = cups[0];
        let a = cups[1];
        let b = cups[2];
        let c = cups[3];

        let mut dest = ((cur + num_cups - 2) % num_cups) + 1;
        while dest == a || dest == b || dest == c
        {
            dest = ((dest + num_cups - 2) % num_cups) + 1
        }
        
        let dest_index = cups.iter().position(|i| *i == dest).unwrap();

        new_cups.resize(0, 0);
        new_cups.reserve(num_cups);

        new_cups.extend(&cups[4..(dest_index + 1)]);
        new_cups.push(a);
        new_cups.push(b);
        new_cups.push(c);
        new_cups.extend(&cups[(dest_index + 1)..]);
        new_cups.push(cur);

        std::mem::swap(&mut cups, &mut new_cups);
    }

    // Finally - end up with cup 1 at the start

    let one_index = cups.iter().position(|i| *i == 1).unwrap();

    let mut result = Vec::<usize>::new();
    result.reserve(num_cups);

    result.extend(&cups[one_index..]);
    result.extend(&cups[..one_index]);

    result
}

fn part_1(input: &str) -> String
{
    simulate(input, 9, 100)
        .iter()
        .skip(1)
        .map(|c| c.to_string())
        .join("")
}

fn part_2(input: &str) -> usize
{
    simulate(input, 1000000, 10000000)
        .iter()
        .skip(1)
        .take(2)
        .product()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(23)
        .example(|| Answer { calculated: simulate(EXAMPLE, 9, 10).iter().join(""), expected: "192658374", })
        .example(|| Answer { calculated: part_1(EXAMPLE), expected: "67384529", })
        .part_1(|| Answer { calculated: part_1(INPUT), expected: "75893264", })
        .example(|| Answer { calculated: part_2(EXAMPLE), expected: 0, })
        .part_2(|| Answer { calculated: part_2(INPUT), expected: 38162588308, })
}
