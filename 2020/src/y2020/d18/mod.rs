use crate::support::*;

const EXAMPLE_1: &str = "1 + 2 * 3 + 4 * 5 + 6";
const EXAMPLE_2: &str = "1 + (2 * 3) + (4 * (5 + 6))";
const EXAMPLE_3: &str = "2 * 3 + (4 * 5)";
const EXAMPLE_4: &str = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
const EXAMPLE_5: &str = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
const EXAMPLE_6: &str = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Token
{
    Num(u64),
    Add,
    Mult,
    Open,
    Close,
}

fn to_tokens(line: &str) -> Vec<Token>
{
    line
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c|
            match c
            {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => Token::Num(c.to_digit(10).unwrap() as u64),
                '+' => Token::Add,
                '*' => Token::Mult,
                '(' => Token::Open,
                ')' => Token::Close,
                _ => unreachable!(),
            })
        .collect()
}

fn to_postfix(line: &str, prec: &dyn Fn(Token) -> usize) -> Vec<Token>
{
    let mut result = Vec::new();
    let mut stack = Vec::new();

    for tok in to_tokens(line)
    {
        match tok
        {
            Token::Num(_) => result.push(tok),
            Token::Open => stack.push(tok),
            Token::Close =>
            {
                while stack.last() != None
                    && stack.last() != Some(&Token::Open)
                {
                    result.push(stack.pop().unwrap());
                }
                assert_eq!(stack.last(), Some(&Token::Open));
                stack.pop();
            },
            _ =>
            {
                while stack.last() != None
                    && stack.last() != Some(&Token::Open)
                    && (prec(tok) <= prec(*stack.last().unwrap()))
                {
                    result.push(stack.pop().unwrap());
                }

                stack.push(tok);
            },
        }
    }

    result.extend(stack.into_iter().rev());

    result
}

fn eval(exp: Vec<Token>) -> u64
{
    let mut stack = Vec::new();

    for tok in exp
    {
        match tok
        {
            Token::Num(num) => stack.push(num),
            Token::Add =>
            {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                stack.push(a + b);
            },
            Token::Mult =>
            {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                stack.push(a * b);
            },
            _ => unreachable!(),
        }
    }

    assert_eq!(stack.len(), 1);
    stack.pop().unwrap()
}

fn prec_1(tok: Token) -> usize
{
    match tok
    {
        Token::Add => 1,
        Token::Mult => 1,
        _ => unreachable!(),
    }
}

fn prec_2(tok: Token) -> usize
{
    match tok
    {
        Token::Add => 2,
        Token::Mult => 1,
        _ => unreachable!(),
    }
}

fn eval_1(line: &str) -> u64
{
    eval(to_postfix(line, &prec_1))
}

fn eval_2(line: &str) -> u64
{
    eval(to_postfix(line, &prec_2))
}

fn part_1(input: &str) -> u64
{
    input_to_lines(input).into_iter()
        .map(|line| eval_1(&line))
        .sum()
}

fn part_2(input: &str) -> u64
{
    input_to_lines(input).into_iter()
        .map(|line| eval_2(&line))
        .sum()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(18)
        .example(|| Answer { calculated: eval_1(EXAMPLE_1), expected: 71, })
        .example(|| Answer { calculated: eval_1(EXAMPLE_2), expected: 51, })
        .example(|| Answer { calculated: eval_1(EXAMPLE_3), expected: 26, })
        .example(|| Answer { calculated: eval_1(EXAMPLE_4), expected: 437, })
        .example(|| Answer { calculated: eval_1(EXAMPLE_5), expected: 12240, })
        .example(|| Answer { calculated: eval_1(EXAMPLE_6), expected: 13632, })
        .part_1(|input| Answer { calculated: part_1(input), expected: 12956356593940u64, })
        .example(|| Answer { calculated: eval_2(EXAMPLE_1), expected: 231, })
        .example(|| Answer { calculated: eval_2(EXAMPLE_2), expected: 51, })
        .example(|| Answer { calculated: eval_2(EXAMPLE_3), expected: 46, })
        .example(|| Answer { calculated: eval_2(EXAMPLE_4), expected: 1445, })
        .example(|| Answer { calculated: eval_2(EXAMPLE_5), expected: 669060, })
        .example(|| Answer { calculated: eval_2(EXAMPLE_6), expected: 23340, })
        .part_2(|input| Answer { calculated: part_2(input), expected: 94240043727614u64, })
}
