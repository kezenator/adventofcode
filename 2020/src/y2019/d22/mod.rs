use crate::support::*;
use std::str::FromStr;
use std::collections::HashSet;
use itertools::Itertools;

const EXAMPLE_1: &str = "deal with increment 7\ndeal into new stack\ndeal into new stack";
const EXAMPLE_2: &str = "cut 6\ndeal with increment 7\ndeal into new stack";
const EXAMPLE_3: &str = "deal with increment 7\ndeal with increment 9\ncut -2";
const EXAMPLE_4: &str = "deal into new stack\ncut -2\ndeal with increment 7\ncut 8\ncut -4\ndeal with increment 7\ncut 3\ndeal with increment 9\ndeal with increment 3\ncut -1";

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Function
{
    pub m: i64,
    pub c: i64,
    pub modulus: i64,
}

impl Function
{
    pub fn new(m: i64, c: i64, modulus: i64) -> Self
    {
        Function
        {
            m: Function::mod_fix(m, modulus),
            c: Function::mod_fix(c, modulus),
            modulus,
        }
    }

    pub fn chain(&self, other: Function) -> Self
    {
        assert_eq!(self.modulus, other.modulus);

        Function::new(
            self.m * other.m,
            Function::mod_fix(self.m * other.c, self.modulus) + self.c,
            self.modulus)
    }

    pub fn apply(&self, val: i64) -> i64
    {
        Function::mod_fix(Function::mod_fix(val * self.m, self.modulus) + self.c, self.modulus)
    }

    fn mod_fix(val: i64, modulus: i64) -> i64
    {
        let mut result = val % modulus;
        if result < 0
        {
            result += modulus;
        }
        assert!(result >= 0 && result < modulus);
        result
    }
}

#[derive(Debug)]
enum Step
{
    Deal,
    Cut(i64),
    DealInc(i64),
}

impl Step
{
    pub fn get_function(&self, num_cards: i64) -> Function
    {
        match self
        {
            Step::Deal => Function::new(-1, -1, num_cards),
            Step::Cut(cut) => Function::new(1, *cut, num_cards),
            Step::DealInc(inc) => Function::new(modinverse::modinverse(*inc, num_cards).unwrap(), 0, num_cards),
        }
    }
}

impl FromStr for Step
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        if s == "deal into new stack"
        {
            Ok(Step::Deal)
        }
        else if s.starts_with("cut ")
        {
            let (num,) = scan(s)
                .skip_str("cut ")
                .remaining().parse::<i64>();

            Ok(Step::Cut(num))
        }
        else
        {
            let (num,) = scan(s)
                .skip_str("deal with increment ")
                .remaining().parse::<i64>();

            Ok(Step::DealInc(num))
        }
    }
}

fn func_pos_to_card_single(input: &str, len: i64) -> Function
{
    let mut func = Function::new(1, 0, len);

    for step in input_to_lines_parsed::<Step>(input).iter().rev()
    {
        func = step.get_function(len).chain(func);
    }

    func
}

fn func_pos_to_card_repeat(input: &str, len: i64, repeats: i64) -> Function
{
    let single_step = func_pos_to_card_single(input, len);

    let mut result = Function::new(1, 0, len);
    let mut times = 0;
    let mut seen = HashSet::new();

    loop
    {
        result = result.chain(single_step.clone());
        times += 1;

        if times == repeats
        {
            return result;
        }

        if !seen.insert(result.clone())
        {
            println!("Seen the same after {} times", times);
            assert!(false);
        }
    }
}

fn example(input: &str) -> String
{
    let len = 10;
    let pos_to_card = func_pos_to_card_repeat(input, len, 1);

    (0..len)
        .map(|i| pos_to_card.apply(i).to_string())
        .join(" ")
}

fn part_1(input: &str) -> i64
{
    let len = 10007;
    let pos_to_card = func_pos_to_card_repeat(input, len, 1);

    for i in 0..len
    {
        if pos_to_card.apply(i) == 2019
        {
            return i;
        }
    }
    unreachable!();
}

fn part_2(_input: &str) -> i64
{
    //func_pos_to_card_repeat(input, 119315717514047, 101741582076661).apply(2020)
    0
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(22)
        .example(|| Answer { calculated: example(""), expected: "0 1 2 3 4 5 6 7 8 9", })
        .example(|| Answer { calculated: example("deal into new stack"), expected: "9 8 7 6 5 4 3 2 1 0", })
        .example(|| Answer { calculated: example("deal with increment 7"), expected: "0 3 6 9 2 5 8 1 4 7", })
        .example(|| Answer { calculated: example("deal with increment 3"), expected: "0 7 4 1 8 5 2 9 6 3", })
        .example(|| Answer { calculated: example("deal with increment 9"), expected: "0 9 8 7 6 5 4 3 2 1", })
        .example(|| Answer { calculated: example("cut 3"), expected: "3 4 5 6 7 8 9 0 1 2", })
        .example(|| Answer { calculated: example("cut -4"), expected: "6 7 8 9 0 1 2 3 4 5", })
        .example(|| Answer { calculated: example(EXAMPLE_1), expected: "0 3 6 9 2 5 8 1 4 7", })
        .example(|| Answer { calculated: example(EXAMPLE_2), expected: "3 0 7 4 1 8 5 2 9 6", })
        .example(|| Answer { calculated: example(EXAMPLE_3), expected: "6 3 0 7 4 1 8 5 2 9", })
        .example(|| Answer { calculated: example(EXAMPLE_4), expected: "9 2 5 8 1 4 7 0 3 6", })
        .part_1(|input| Answer { calculated: part_1(input), expected: 1498, })
        .part_2(|input| Answer { calculated: part_2(input), expected: 0, })
}
