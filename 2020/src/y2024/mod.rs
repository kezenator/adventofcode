use crate::support::*;

mod d01;
mod d02;

pub fn puzzles() -> PuzzleYear
{
    puzzle_year(2024)
    .with(d01::puzzles())
    .with(d02::puzzles())
}
