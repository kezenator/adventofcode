use crate::support::*;

mod d01;

pub fn puzzles() -> PuzzleYear
{
    puzzle_year(2024)
        .with(d01::puzzles())
}
