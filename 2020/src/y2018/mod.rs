use crate::support::*;

mod d01;
mod d02;
mod d03;
mod d17;
mod d20;

pub fn puzzles() -> PuzzleYear
{
    puzzle_year(2018)
        .with(d01::puzzles())
        .with(d02::puzzles())
        .with(d03::puzzles())
        .with(d17::puzzles())
        .with(d20::puzzles())
}
