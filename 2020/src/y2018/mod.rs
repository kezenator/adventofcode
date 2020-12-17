use crate::support::*;

mod d20;

pub fn puzzles() -> PuzzleYear
{
    puzzle_year(2018)
        .with(d20::puzzles())
}
