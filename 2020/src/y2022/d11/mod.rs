use crate::support::*;
use itertools::*;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

enum BinaryOp
{
    Mult,
    Add,
}

impl BinaryOp
{
    fn parse(s: &str) -> BinaryOp
    {
        match s
        {
            "+" => BinaryOp::Add,
            "*" => BinaryOp::Mult,
            _ => unreachable!(),
        }
    }
}

enum Var
{
    Old,
    Immediate(u64),
}

impl Var
{
    fn parse(s: &str) -> Var
    {
        if s == "old" { Var::Old }
        else { Var::Immediate(s.parse().unwrap())}
    }

    fn evaluate(&self, old: u64) -> u64
    {
        match self
        {
            Var::Old => old,
            Var::Immediate(imm) => *imm,
        }
    }
}

struct Op
{
    left: Var,
    op: BinaryOp,
    right: Var,
}

impl Op
{
    fn new(left: &str, op: &str, right: &str) -> Op
    {
        Op
        {
            left: Var::parse(left),
            op: BinaryOp::parse(op),
            right: Var::parse(right),
        }
    }

    fn evaulate(&self, old: u64) -> u64
    {
        let left = self.left.evaluate(old);
        let right = self.right.evaluate(old);

        match self.op
        {
            BinaryOp::Add => left + right,
            BinaryOp::Mult => left * right,
        }
    }
}

struct Monkey
{
    items: Vec<u64>,
    op: Op,
    test_divisible_by: u64,
    dest_true: usize,
    dest_false: usize,
    inspect_count: usize,
}

impl Monkey
{
    fn parse(lines: Vec<String>) -> Monkey
    {
        let items = lines[1].split_at(18).1.split(", ").map(|s| s.parse().unwrap()).collect_vec();
        let op_parts = lines[2].split_at(19).1.split(" ").collect_vec();
        let op = Op::new(op_parts[0], op_parts[1], op_parts[2]);
        let test_divisible_by = lines[3].split_at(21).1.parse().unwrap();
        let dest_true = lines[4].split_at(29).1.parse().unwrap();
        let dest_false = lines[5].split_at(30).1.parse().unwrap();

        Monkey { items, op, test_divisible_by, dest_true, dest_false, inspect_count: 0 }
    }
}

fn round(monkeys: &mut Vec<Monkey>, divide_worry: bool, test_divisible_lcm: u64)
{
    let len = monkeys.len();
    for i in 0..len
    {
        let items = monkeys[i].items.clone();
        monkeys[i].items.clear();

        for mut item in items
        {
            monkeys[i].inspect_count += 1;

            item = monkeys[i].op.evaulate(item);

            if divide_worry
            {
                item = item / 3;
            }
            else
            {
                item = item % test_divisible_lcm;
            }

            let dest = if (item % monkeys[i].test_divisible_by) == 0
            {
                monkeys[i].dest_true
            }
            else
            {
                monkeys[i].dest_false
            };

            assert!(dest != i);
            monkeys[dest].items.push(item);
        }
    }
}

fn monkey_business(input: &str, num_rounds: usize, divide_worry: bool) -> usize
{
    let mut monkeys = input_to_groups(input).drain(..)
        .map(|g| Monkey::parse(g))
        .collect_vec();

    let mut test_divisible_lcm = 1;
    for monkey in monkeys.iter()
    {
        test_divisible_lcm = num::lcm(test_divisible_lcm, monkey.test_divisible_by);
    }

    for _ in 0..num_rounds { round(&mut monkeys, divide_worry, test_divisible_lcm); }

    monkeys.iter()
        .map(|m| m.inspect_count )
        .sorted()
        .rev()
        .take(2)
        .product()
}

fn part_1(input: &str) -> usize
{
    monkey_business(input, 20, true)
}

fn part_2(input: &str) -> usize
{
    monkey_business(input, 10000, false)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(11)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 10605,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 99852,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 2713310158i64,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 25935263541i64,
        })
}
