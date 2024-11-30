use crate::support::*;
use itertools::*;
use pathfinding::prelude::directions::E;
use std::collections::HashMap;

const EXAMPLE: &str = include_str!("example.txt");

#[derive(Debug, Clone)]
enum Op
{
    Add,
    Sub,
    Mul,
    Div,
}

impl Op
{
    fn from_str(input: &str) -> Op
    {
        match input
        {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            "/" => Op::Div,
            _ => unreachable!(),
        }
    }

    fn execute(&self, v1: i64, v2: i64) -> i64
    {
        match self
        {
            Op::Add => v1 + v2,
            Op::Sub => v1 - v2,
            Op::Mul => v1 * v2,
            Op::Div => v1 / v2,
        }
    }
}

#[derive(Clone)]
enum Job
{
    Num(i64),
    Result(String, Op, String),
}

impl Job
{
    fn from_str(input: &str) -> (String, Job)
    {
        let (name, rest) = scan(input)
            .until(": ").parse::<String>()
            .remaining().parse::<String>();

        match rest.parse::<i64>()
        {
            Ok(num) => (name, Job::Num(num)),
            Err(_) =>
            {
                let (left, op_str, right) = scan(&rest)
                    .until(" ").parse::<String>()
                    .until(" ").parse::<String>()
                    .remaining().parse::<String>();

                (name, Job::Result(left, Op::from_str(&op_str), right))
            },
        }
    }
}

fn calc(lines: &HashMap<String, Job>, name: &str) -> i64
{
    match lines.get(name).unwrap()
    {
        Job::Num(num) =>
        {
            *num
        },
        Job::Result(left, op, right) =>
        {
            op.execute(calc(lines, left), calc(lines, right))
        },
    }
}

fn contains_var(lines: &HashMap<String, Job>, var_to_solve: &String, var_to_calc: &String) -> bool
{
    if *var_to_solve == *var_to_calc
    {
        return true;
    }

    match lines.get(var_to_calc).unwrap()
    {
        Job::Num(_) => false,
        Job::Result(left, _, right) =>
            contains_var(lines, var_to_solve, left) || contains_var(lines, var_to_solve, right),
    }
}

fn solve(lines: &HashMap<String, Job>, var_to_solve: &String, var_to_calc: &String, desired_val: i64) -> i64
{
    match lines.get(var_to_calc).unwrap()
    {
        Job::Num(_) =>
        {
            assert!(*var_to_solve == *var_to_calc);
            desired_val
        },
        Job::Result(left, op, right) =>
        {
            assert!(*var_to_solve != *var_to_calc);

            let in_left = contains_var(lines, var_to_solve, left);
            let in_right = contains_var(lines, var_to_solve, right);

            assert!((in_left || in_right) && !(in_left && in_right));

            if in_left
            {
                let right_val = calc(lines, right);
                match op
                {
                    Op::Add => solve(lines, var_to_solve, left, desired_val - right_val),
                    Op::Sub => solve(lines, var_to_solve, left, desired_val + right_val),
                    Op::Mul => solve(lines, var_to_solve, left, desired_val / right_val),
                    Op::Div => solve(lines, var_to_solve, left, desired_val * right_val),
                }
            }
            else
            {
                let left_val = calc(lines, left);
                match op
                {
                    Op::Add => solve(lines, var_to_solve, right, desired_val - left_val),
                    Op::Sub => solve(lines, var_to_solve, right, left_val - desired_val),
                    Op::Mul => solve(lines, var_to_solve, right, desired_val / left_val),
                    Op::Div => solve(lines, var_to_solve, right, left_val / desired_val),
                }
            }
        }
    }
}

fn part_1(input: &str) -> i64
{
    let lines = input_to_lines(input)
        .iter()
        .map(|l| Job::from_str(l))
        .collect::<HashMap<_,_>>();

    calc(&lines, "root")
}

fn part_2(input: &str) -> i64
{
    let mut lines = input_to_lines(input)
        .iter()
        .map(|l| Job::from_str(l))
        .collect::<HashMap<_,_>>();

    let humn_str = "humn".to_string();
    let root = lines.get("root").cloned().unwrap();
    if let Job::Result(left, _, right) = &root
    {
        let in_left = contains_var(&lines, &humn_str, left);
        let in_right = contains_var(&lines, &humn_str, right);
        assert!((in_left || in_right) && !(in_left && in_right));

        if in_left
        {
            let right_val = calc(&lines, right);
            let result = solve(&lines, &humn_str, left, right_val);
            lines.insert(humn_str.clone(), Job::Num(result));
            assert!(right_val == calc(&lines, left));
            return result;
        }
        else
        {
            let left_val = calc(&lines, left);
            let result = solve(&lines, &humn_str, right, left_val);
            lines.insert(humn_str.clone(), Job::Num(result));
            assert!(left_val == calc(&lines, right));
            return result;
        }
    }
    unreachable!();
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(21)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 152,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 157714751182692i64,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 301,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 3373767893067i64,
        })
}
