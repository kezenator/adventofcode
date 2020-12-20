use crate::support::*;

mod d17;
mod d20;

pub fn puzzles() -> PuzzleYear
{
    puzzle_year(2018)
        .with(d17::puzzles())
        .with(d20::puzzles())
}
