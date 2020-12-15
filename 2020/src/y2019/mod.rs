use crate::support::*;

pub mod intcode;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d09;
mod d11;
mod d13;
mod d22;
mod d23;
mod d24;
mod d25;

pub fn puzzles() -> PuzzleYear
{
    puzzle_year(2019)
        .with(d01::puzzles())
        .with(d02::puzzles())
        .with(d03::puzzles())
        .with(d04::puzzles())
        .with(d05::puzzles())
        .with(d06::puzzles())
        .with(d07::puzzles())
        .with(d09::puzzles())
        .with(d11::puzzles())
        .with(d13::puzzles())
        .with(d22::puzzles())
        .with(d23::puzzles())
        .with(d24::puzzles())
        .with(d25::puzzles())
}
