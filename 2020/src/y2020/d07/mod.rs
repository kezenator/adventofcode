use std::collections::HashMap;
use crate::support::*;

const EXAMPLE_1: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.";
const EXAMPLE_2: &str = "shiny gold bags contain 2 dark red bags.\ndark red bags contain 2 dark orange bags.\ndark orange bags contain 2 dark yellow bags.\ndark yellow bags contain 2 dark green bags.\ndark green bags contain 2 dark blue bags.\ndark blue bags contain 2 dark violet bags.\ndark violet bags contain no other bags.";
const INPUT: &str = include_str!("input.txt");

fn parse_rules(input: &str) -> HashMap<String, Vec<(usize, String)>>
{
    input_to_lines(input)
        .iter()
        .map(|line|
            {
                let (color, remain) = scan(line)
                    .until(" bags contain ").parse::<String>()
                    .remaining().parse::<String>();

                let mut contents = Vec::new();

                if remain != "no other bags."
                {
                    let remain = remain.trim_end_matches(".");

                    for part in remain.split(", ")
                    {
                        let (num, other_color) = scan(part)
                            .until(" ").parse::<usize>()
                            .remaining().parse::<String>();

                        let other_color = other_color.trim_end_matches(" bags");
                        let other_color = other_color.trim_end_matches(" bag").to_owned();

                        contents.push((num, other_color));
                    }
                }

                (color, contents)
            })
        .collect()
}

fn contains_target(color: &String, target: &str, rules: &HashMap<String, Vec<(usize, String)>>) -> bool
{
    for (_, sub_color) in rules.get(color).unwrap()
    {
        if sub_color == target
        {
            return true;
        }
        else if contains_target(sub_color, target, rules)
        {
            return true;
        }
    }
    return false;
}

fn contents_size(color: &str, rules: &HashMap<String, Vec<(usize, String)>>) -> usize
{
    rules.get(color).unwrap().iter()
        .map(|(num, sub_color)| num * (1 + contents_size(sub_color, rules)))
        .sum()
}

fn part_1(input: &str) -> usize
{
    let rules = parse_rules(input);
    
    rules.iter()
        .filter(|(color, _)| contains_target(color, "shiny gold", &rules))
        .count()
}

fn part_2(input: &str) -> usize
{
    let rules = parse_rules(input);

    contents_size("shiny gold", &rules)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(7)
        .example(|| Answer { calculated: part_1(EXAMPLE_1), expected: 4, })
        .part_1(|| Answer { calculated: part_1(INPUT), expected: 248, })
        .example(|| Answer { calculated: part_2(EXAMPLE_2), expected: 126, })
        .part_2(|| Answer { calculated: part_2(INPUT), expected: 57281, })
}
