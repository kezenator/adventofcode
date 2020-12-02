use crate::support::*;

pub mod intcode;

mod d01;
mod d02;

pub fn puzzles<'a>(puzzles: &mut PuzzleSet)
{
    d01::puzzles(puzzles);
    d02::puzzles(puzzles);
}
