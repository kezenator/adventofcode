
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");

struct LanParty
{
    computers: HashSet<String>,
    links: HashMap<String, HashSet<String>>,
}

impl LanParty
{
    fn new(input: &str) -> Self
    {
        let pairs = input_to_lines(input).iter()
            .map(|l| scan(l).until("-").parse().remaining().parse())
            .collect::<Vec<(String, String)>>();

        let mut computers = HashSet::new();
        let mut links = HashMap::<String, HashSet<String>>::new();

        for p in pairs
        {
            computers.insert(p.0.clone());
            computers.insert(p.1.clone());
            links.entry(p.0.clone()).or_default().insert(p.1.clone());
            links.entry(p.1).or_default().insert(p.0);
        }

        LanParty { computers, links }
    }

    fn cliques(&self) -> Vec<HashSet<String>>
    {
        let result = bron_kerbosch_maximal_cliques(
            self.computers.iter(),
            |v| self.links.get(v).unwrap().iter().cloned());

        result
    }

    fn triples(&self) -> HashSet<Vec<String>>
    {
        let mut result = HashSet::new();
        for c0 in self.computers.iter()
        {
            for pairs in self.links.get(c0).unwrap().iter()
                .combinations(2)
            {
                if self.links.get(pairs[0]).unwrap().contains(pairs[1])
                {
                    let mut triple = vec![
                        c0.clone(),
                        pairs[0].clone(),
                        pairs[1].clone()
                    ];
                    triple.sort();
                    result.insert(triple);
                }
            }
        }
        result
    }

    fn largest_set(&self) -> HashSet<String>
    {
        let sets = self.cliques();
        let largest_size = sets.iter()
            .map(|s| s.len())
            .max().unwrap();
        sets.into_iter()
            .filter(|s| s.len() == largest_size)
            .next().unwrap()
    }
}

fn part_1(input: &str) -> usize
{
    let party = LanParty::new(input);

    party.triples().iter()
        .filter(|trip|
        {
            trip.iter().any(|c| c.starts_with("t"))
        })
        .count()
}

fn part_2(input: &str) -> String
{
    let party = LanParty::new(input);
    let mut largest_set = party.largest_set().into_iter().collect_vec();
    largest_set.sort();
    largest_set.join(",")
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(23)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 7,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 1269,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: "co,de,ka,ta",
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: "ad,jw,kt,kz,mt,nc,nr,sb,so,tg,vs,wh,yh",
        })
}
