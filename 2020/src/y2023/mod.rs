use crate::support::*;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;

pub fn puzzles() -> PuzzleYear
{
    puzzle_year(2023)
        .with(d01::puzzles())
        .with(d02::puzzles())
        .with(d03::puzzles())
        .with(d04::puzzles())
        .with(d05::puzzles())
}
