use crate::support::*;
use std::collections::HashSet;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

struct Square
{
    pub x: usize,
    pub y: usize,
    pub number: usize,
    pub marked: bool
}

impl Square
{
    pub fn new(x: usize, y: usize, number: usize) -> Self
    {
        Square { x, y, number, marked: false }
    }
}

struct Board
{
    squares: Vec<Square>
}

impl Board
{
    pub fn new(lines: &Vec<String>, index: usize) -> Self
    {
        let mut squares = Vec::new();

        for y in 0..5
        {
            let row = lines[index + y]
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            assert!(row.len() == 5);

            for x in 0..5
            {
                squares.push(Square::new(x, y, row[x]));
            }
        }

        Board { squares }
    }

    pub fn mark(&mut self, num: usize)
    {
        for sq in self.squares.iter_mut()
        {
            if sq.number == num
            {
                sq.marked = true;
            }
        }
    }

    pub fn has_won(&self) -> bool
    {
        for i in 0..5
        {
            let in_row = self.squares.iter()
                .filter(|&sq| (sq.y == i) && sq.marked)
                .count();
            let in_column = self.squares.iter()
                .filter(|&sq| (sq.x == i) && sq.marked)
                .count();

            if (in_row == 5) || (in_column == 5)
            {
                return true;
            }
        }
        return false;
    }

    pub fn sum_of_unmarked_squares(&self) -> usize
    {
        self.squares.iter()
            .filter(|s| s.marked == false)
            .map(|s| s.number)
            .sum()
    }
}

fn parse(input: &str) -> (Vec<usize>, Vec<Board>)
{
    let lines = input_to_lines(input);

    let numbers = lines[0]
        .split_terminator(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut boards = Vec::new();

    let mut index = 2;
    while index < lines.len()
    {
        boards.push(Board::new(&lines, index));
        index += 6;
    }

    (numbers, boards)
}

fn scores_of_winning_boards_by_order(input: &str) -> Vec<usize>
{
    let (numbers, boards) = parse(input);
    let mut boards = boards;

    let mut boards_in_play = (0..boards.len()).collect::<HashSet<usize>>();
    let mut scores = Vec::new();

    for num in numbers
    {
        for (index, board) in boards.iter_mut().enumerate()
        {
            if boards_in_play.contains(&index)
            {
                board.mark(num);

                if board.has_won()
                {
                    let score = num * board.sum_of_unmarked_squares();
                    scores.push(score);
                    boards_in_play.remove(&index);

                    if boards_in_play.is_empty()
                    {
                        return scores;
                    }
                }
            }
        }
    }

    unreachable!();
}

fn part_1(input: &str) -> usize
{
    let scores = scores_of_winning_boards_by_order(input);
    scores[0]
}

fn part_2(input: &str) -> usize
{
    let scores = scores_of_winning_boards_by_order(input);
    scores[scores.len() - 1]
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(4)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 4512,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 89001,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 1924,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 7296,
        })
}
