use std::collections::BTreeMap;
use crate::support::*;
use itertools::*;

const EXAMPLE: &str = include_str!("example.txt");

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Rank
{
    High,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<Vec<usize>> for Rank
{
    fn from(value: Vec<usize>) -> Self
    {
        assert!(value.len() == 5);

        if value == vec![0; 5]
        {
            // Special case - 5 Jokers
            return Rank::FiveOfAKind;
        }

        // Sort into runs, then group by card: card => count

        let mut value = value;
        value.sort();

        let mut groups = value.into_iter()
            .chunk_by(|v| *v)
            .into_iter()
            .map(|(key, it)| (key, it.count()))
            .collect::<BTreeMap::<_, _>>();

        // Get the number of Jokers and remove
        // these cards from consideration

        let num_jokers = groups.get(&0).cloned().unwrap_or(0);
        groups.remove(&0);

        // Sort groups by count (highest to lowest)

        let mut groups = groups.into_iter()
            .sorted_by(|(card1, count1), (card2, count2)| (count2, card2).cmp(&(count1, card1)))
            .collect_vec();

        // Apply Jokers to the card group with the largest count

        assert!(!groups.is_empty());

        groups[0].1 += num_jokers;

        // Finally - now we can rank the hand

        let best_count = groups[0].1;

        if best_count == 5
        {
            Rank::FiveOfAKind
        }
        else if best_count == 4
        {
            Rank::FourOfAKind
        }
        else if best_count == 3
        {
            if groups[1].1 == 2
            {
                Rank::FullHouse
            }
            else
            {
                Rank::ThreeOfAKind                
            }
        }
        else if best_count == 2
        {
            if groups[1].1 == 2
            {
                Rank::TwoPair
            }
            else
            {
                Rank::Pair
            }
        }
        else
        {
            Rank::High
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand
{
    rank: Rank,
    cards: Vec<usize>,
}

#[derive(Debug)]
struct HandBid
{
    hand: Hand,
    bid: usize,
}

fn parse_line(line: &str, part_2: bool) -> HandBid
{
    let map = if part_2 { "J-23456789T-QKA" } else { "--23456789TJQKA" };
    let parts = line.split(" ").collect_vec();
    let cards = parts[0].chars().map(|c| map.find(c).unwrap()).collect_vec();
    let rank = cards.clone().into();
    let hand = Hand { rank, cards };
    let bid = parts[1].parse().unwrap();

    HandBid{hand, bid}
}

fn total_winnings(input: &str, part_2: bool) -> usize
{
    let mut hands_and_bids = input_to_lines(input).iter().map(|l| parse_line(l, part_2)).collect_vec();
    hands_and_bids.sort_by(|a, b| a.hand.cmp(&b.hand));

    hands_and_bids.iter().enumerate()
        .map(|(index, hand_bid)| (index + 1) * hand_bid.bid)
        .sum()
}

fn part_1(input: &str) -> usize
{
    total_winnings(input, false)
}

fn part_2(input: &str) -> usize
{
    total_winnings(input, true)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(7)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 6440,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 249726565,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 5905,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 251135960,
        })
}
