use std::collections::{HashMap, HashSet};
use crate::support::*;

const EXAMPLE_1: &str = "class: 1-3 or 5-7\nrow: 6-11 or 33-44\nseat: 13-40 or 45-50\n\nyour ticket:\n7,1,14\n\nnearby tickets:\n7,3,47\n40,4,50\n55,2,20\n38,6,12";
const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Class
{
    name: String,
    min_a: i64,
    max_a: i64,
    min_b: i64,
    max_b: i64,
}

impl Class
{
    pub fn scan(input: &str) -> Self
    {
        let (name, min_a, max_a, min_b, max_b) = scan(input)
            .until(": ").parse::<String>()
            .until("-").parse::<i64>()
            .until(" or ").parse::<i64>()
            .until("-").parse::<i64>()
            .remaining().parse::<i64>();

        Class { name, min_a, max_a, min_b, max_b }
    }

    pub fn is_valid(&self, val: i64) -> bool
    {
        (self.min_a <= val && val <= self.max_a)
            || (self.min_b <= val && val <= self.max_b)
    }
}

struct Input
{
    classes: HashMap<String, Class>,
    your_ticket: Vec<i64>,
    invalid_tickets: Vec<Vec<i64>>,
    valid_tickets: Vec<Vec<i64>>,
}

impl Input
{
    pub fn scan(input: &str) -> Self
    {
        let groups = input_to_groups(input);

        let classes = groups[0].iter()
            .map(|s| Class::scan(s))
            .map(|c| (c.name.clone(), c))
            .collect::<HashMap<String, Class>>();

        let (your_ticket,) = scan(&groups[1][1]).remaining().parse_vec::<i64>(",");

        let all_tickets = groups[2][1..].iter()
            .map(|s| scan(s).remaining().parse_vec::<i64>(",").0)
            .collect::<Vec<Vec<i64>>>();

        let matches_any_class = |i: i64| -> bool
        {
            classes.values()
                .filter(|c| c.is_valid(i))
                .count() != 0
        };

        let invalid_tickets = all_tickets.iter()
            .filter(|t| t.iter().filter(|&i| !matches_any_class(*i)).count() != 0)
            .cloned()
            .collect::<Vec<_>>();

        let valid_tickets = all_tickets.iter()
            .filter(|t| t.iter().filter(|&i| !matches_any_class(*i)).count() == 0)
            .cloned()
            .collect::<Vec<_>>();

        Input{ classes, your_ticket, invalid_tickets, valid_tickets }
    }
}

pub fn part_1(input: &str) -> i64
{
    let input = Input::scan(input);

    let matches_any_class = |i: i64| -> bool
    {
        input.classes.values()
            .filter(|c| c.is_valid(i))
            .count() != 0
    };

    input.invalid_tickets.iter().flatten()
        .filter(|&i| !matches_any_class(*i))
        .sum()
}

fn part_2(input: &str) -> i64
{
    let input = Input::scan(input);

    // Keep some maps about which classes we've mapped
    // to which value index

    let mut class_to_vindex = HashMap::<String, usize>::new();
    let mut vindex_found = HashSet::<usize>::new();

    // Loop until all classes have been assigned a value index

    while class_to_vindex.len() < input.classes.len()
    {
        // Search through the remaining classes to be mapped

        let remaining_classes = input.classes.keys()
            .filter(|&n| !class_to_vindex.contains_key(n))
            .cloned()
            .collect::<Vec<String>>();

        for name in remaining_classes
        {
            let class = input.classes.get(&name).unwrap();

            // Count how many remaining value indexes
            // are valid for all valid tickets for this class

            let mut possible_vindex = Vec::new();

            for vindex in 0..input.your_ticket.len()
            {
                if !vindex_found.contains(&vindex)
                {
                    if input.valid_tickets.iter()
                        .map(|t| t[vindex])
                        .filter(|&i| !class.is_valid(i))
                        .count() == 0
                    {
                        // This vindex is valid across all valid tickets
                        // for this class

                        possible_vindex.push(vindex);
                    }
                }
            }

            // If this remaining class has no indexes where all valid
            // tickets are valid - then we can't make any progress

            if possible_vindex.is_empty()
            {
                println!("** class {} has no possible remaining indexes it will support", name);
                unreachable!();
            }

            // If there's only *one* value index that's valid for this class,
            // then we can assign it to this class.

            if possible_vindex.len() == 1
            {
                //println!("class {} is assigned to index {}", name, possible_vindex[0]);

                class_to_vindex.insert(name, possible_vindex[0]);
                vindex_found.insert(possible_vindex[0]);
            }
        }
    }

    // Finally, return the product of
    // all "departure" filds for your ticket.

    input.classes.values()
        .filter(|&c| c.name.starts_with("departure"))
        .map(|c| input.your_ticket[*class_to_vindex.get(&c.name).unwrap()])
        .product()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(16)
        .example(|| Answer { calculated: part_1(EXAMPLE_1), expected: 71, })
        .part_1(|| Answer { calculated: part_1(INPUT), expected: 27911, })
        .part_2(|| Answer { calculated: part_2(INPUT), expected: 737176602479i64, })
}
