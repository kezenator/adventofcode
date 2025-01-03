use crate::support::*;

const EXAMPLE1: &str = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab\n";
const EXAMPLE2: &str = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz\n";

fn count(line: &str, ch: char) -> usize
{
    line.chars()
        .filter(|c| *c == ch)
        .count()
}

fn has_exactly(line: &str, target: usize) -> bool
{
    for ch in line.chars()
    {
        if count(line, ch) == target
        {
            return true;
        }
    }
    return false;
}

fn count_with_exactly(inp: &str, target: usize) -> usize
{
    input_to_lines(inp).iter()
        .filter(|a| has_exactly(a, target))
        .count()
}

fn part_1(inp: &str) -> usize
{
    return count_with_exactly(inp, 2) * count_with_exactly(inp, 3);
}

fn difference(a: &str, b: &str) -> usize
{
    return a.chars().zip(b.chars())
        .filter(|(ca, cb)| *ca != *cb)
        .count();
}

fn part_2(inp: &str) -> String
{
    let lines = input_to_lines(inp);

    for la in lines.iter()
    {
        for lb in lines.iter()
        {
            if difference(la, lb) == 1
            {
                return la.chars().zip(lb.chars())
                    .filter(|(ca, cb)| *ca == *cb)
                    .map(|(ca, _cb)| ca)
                    .collect();
            }
        }
    }

    assert!(false);
    unreachable!();
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(2)
        .example(|| Answer {
            calculated: part_1(EXAMPLE1),
            expected: 12, })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 5928, })
        .example(|| Answer {
            calculated: part_2(EXAMPLE2),
            expected: "fgij", })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: "bqlporuexkwzyabnmgjqctvfs", })
}