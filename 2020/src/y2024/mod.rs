use crate::support::*;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;

pub fn puzzles() -> PuzzleYear
{
    puzzle_year(2024)
    .with(d01::puzzles())
    .with(d02::puzzles())
    .with(d03::puzzles())
    .with(d04::puzzles())
    .with(d05::puzzles())
    .with(d06::puzzles())
    .with(d07::puzzles())
    .with(d08::puzzles())
}
