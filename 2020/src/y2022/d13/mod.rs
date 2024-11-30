use crate::support::*;
use itertools::*;
use std::cmp::Ordering;

const EXAMPLE: &str = include_str!("example.txt");

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet
{
    Int(usize),
    List(Vec<Packet>),
}

impl Packet
{
    fn parse(line: &str) -> Self
    {
        let chars = line.chars().collect_vec();
        let mut pos = 0;
        let list = Packet::parse_list(&chars, &mut pos);
        assert!(pos == chars.len());
        return Packet::List(list);
    }

    fn parse_list(chars: &Vec<char>, pos: &mut usize) -> Vec<Packet>
    {
        let mut result = Vec::new();
        assert!(chars[*pos] == '[');
        *pos += 1;
        while chars[*pos] != ']'
        {
            if chars[*pos] == '['
            {
                result.push(Packet::List(Packet::parse_list(chars, pos)));
            }
            else
            {
                assert!(chars[*pos].is_digit(10));
                let start = *pos;
                *pos += 1;
                while chars[*pos].is_digit(10) { *pos += 1; }
                result.push(Packet::Int(chars[start..*pos].iter().collect::<String>().parse().unwrap()));
            }
            assert!(chars[*pos] == ',' || chars[*pos] == ']');
            if chars[*pos] == ',' { *pos += 1; }
        }
        *pos += 1;
        result
    }

    fn custom_compare(&self, other: &Self) -> Ordering
    {
        match (self, other)
        {
            (Packet::Int(a), Packet::Int(b)) => a.cmp(b),
            (Packet::List(a), Packet::List(b)) =>
            {
                for i in 0..a.len().min(b.len())
                {
                    match a[i].custom_compare(&b[i])
                    {
                        Ordering::Equal => continue,
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                    }
                }
                a.len().cmp(&b.len())
            },
            (Packet::Int(a), Packet::List(_)) =>
            {
                Packet::List(vec![Packet::Int(*a)]).custom_compare(other)
            },
            (Packet::List(_), Packet::Int(b)) =>
            {
                self.custom_compare(&Packet::List(vec![Packet::Int(*b)]))
            },
        }
    }
}

fn part_1(input: &str) -> usize
{
    input_to_groups(input)
        .iter()
        .enumerate()
        .map(|(i, g)| (i, (Packet::parse(&g[0]), Packet::parse(&g[1]))))
        .filter(|(_, (a, b))| a.custom_compare(b) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum()
}

fn part_2(input: &str) -> usize
{
    let mut packets = input_to_lines(input)
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| Packet::parse(l))
        .collect_vec();

    let divider_2 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let divider_6 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);

    packets.push(divider_2.clone());
    packets.push(divider_6.clone());

    packets.sort_by(|a, b| a.custom_compare(b));

    packets.iter()
        .enumerate()
        .filter(|(_, p)| **p == divider_2 || **p == divider_6)
        .map(|(i, _)| i + 1)
        .product()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(13)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 13,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 5717,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 140,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 25935,
        })
}
