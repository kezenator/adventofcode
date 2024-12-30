
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use rayon::prelude::*;
use crate::support::*;

const EXAMPLE1: &str = include_str!("example_1.txt");
const EXAMPLE2: &str = include_str!("example_2.txt");

fn mix(result: i64, secret: i64) -> i64
{
    result ^ secret
}

fn prune(secret: i64) -> i64
{
    secret % 16777216
}

fn evolve(secret: i64) -> i64
{
    let step1 = prune(mix(secret * 64, secret));
    let step2 = prune(mix(step1 / 32, step1));
    prune(mix(step2 * 2048, step2))
}

fn buyers_numbers(initial_secret: i64) -> impl Iterator<Item = i64>
{
    let mut secret = initial_secret;
    let mut result = Vec::new();
    result.push(initial_secret);
    for _ in 0..2000
    {
        secret = evolve(secret);
        result.push(secret);
    }
    result.into_iter()
}

fn buyer_seq_to_price(initial_secret: i64) -> HashMap<(i64, i64, i64, i64), i64>
{
    let mut result = HashMap::new();

    for (seq, price) in buyers_numbers(initial_secret)
        .map(|sec| sec % 10)
        .tuple_windows()
        .map(|(a, b, c, d, e)|
            {
                let seq = ((b - a), (c - b), (d - c), (e - d));
                let price = e;
                (seq, price)
            })
    {
        result.entry(seq).or_insert(price);
    }
    result
}

fn part_1(input: &str) -> i64
{
    input_to_lines_parsed(input).into_iter()
        .map(|s| buyers_numbers(s).last().unwrap())
        .sum()
}

fn part_2(input: &str) -> i64
{
    let initial_secrets = input_to_lines_parsed::<i64>(input);
    
    let seq_to_prices = initial_secrets.iter()
        .map(|is| buyer_seq_to_price(*is))
        .collect_vec();
    
    let all_seqs = seq_to_prices.iter()
        .map(|s2p| s2p.keys())
        .flatten()
        .cloned()
        .collect::<HashSet<_>>();

    all_seqs.into_par_iter()
        .map(|seq|
        {
            seq_to_prices.iter()
                .map(|s2p| s2p.get(&seq).map(|p| p.clone()))
                .map(|opt_p| opt_p.unwrap_or(0))
                .sum()
        })
        .max().unwrap()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(22)
        .example(|| Answer {
            calculated: part_1(EXAMPLE1),
            expected: 37327623,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 13461553007i64,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE2),
            expected: 23,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 1499,
        })
}
