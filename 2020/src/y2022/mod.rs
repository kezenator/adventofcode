use crate::support::*;

mod d01;
mod d02;
mod d03;

pub fn puzzles() -> PuzzleYear
{
    puzzle_year(2022)
        .with(d01::puzzles())
        .with(d02::puzzles())
        .with(d03::puzzles())
}
