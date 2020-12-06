use crate::support::*;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;

pub fn puzzles<'a>(puzzles: &mut PuzzleSet)
{
    d01::puzzles(puzzles);
    d02::puzzles(puzzles);
    d03::puzzles(puzzles);
    d04::puzzles(puzzles);
    d05::puzzles(puzzles);
    d06::puzzles(puzzles);
}
