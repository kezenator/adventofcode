use crate::support::*;
use std::str::FromStr;

const EXAMPLE: &str = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc\n";
const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Policy
{
    a: usize,
    b: usize,
    ch: char,
    password: String,
}

impl FromStr for Policy
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let (a, b, ch, password) = scan(s)
            .until("-").parse::<usize>()
            .until(" ").parse::<usize>()
            .until(": ").parse::<char>()
            .remaining().parse::<String>();

        Ok(Policy{ a, b, ch, password })
    }
}

impl Policy
{
    fn valid_1(&self) -> bool
    {
        let count = self.password.chars().filter(|c| *c == self.ch).count();

        (count >= self.a) && (count <= self.b)
    }

    fn valid_2(&self) -> bool
    {
        let a = self.password.chars().nth(self.a - 1).unwrap() == self.ch;
        let b = self.password.chars().nth(self.b - 1).unwrap() == self.ch;

        (a || b) && !(a && b)
    }
}

fn part_1(input: &str) -> usize
{
    input_to_lines_parsed::<Policy>(input)
        .iter()
        .filter(|p| p.valid_1())
        .count()
}
 
fn part_2(input: &str) -> usize
{
    input_to_lines_parsed::<Policy>(input)
        .iter()
        .filter(|p| p.valid_2())
        .count()
}
 
pub fn puzzles(puzzles: &mut PuzzleSet)
{
    puzzles.register("y2020.d02.e1", || Answer {
        calculated: part_1(EXAMPLE),
        expected: 2,
    });

    puzzles.register("y2020.d02.e2", || Answer {
        calculated: part_2(EXAMPLE),
        expected: 1,
    });

    puzzles.register("y2020.d02.p1", || Answer {
        calculated: part_1(INPUT),
        expected: 445,
    });

    puzzles.register("y2020.d02.p2", || Answer {
        calculated: part_2(INPUT),
        expected: 491,
    });
}
