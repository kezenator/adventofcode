use crate::support::*;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;

pub fn puzzles() -> PuzzleYear
{
    puzzle_year(2023)
        .with(d01::puzzles())
        .with(d02::puzzles())
        .with(d03::puzzles())
        .with(d04::puzzles())
        .with(d05::puzzles())
        .with(d06::puzzles())
        .with(d07::puzzles())
        .with(d08::puzzles())
        .with(d09::puzzles())
        .with(d10::puzzles())
        .with(d11::puzzles())
        .with(d12::puzzles())
        .with(d13::puzzles())
        .with(d14::puzzles())
        .with(d15::puzzles())
        .with(d16::puzzles())
        .with(d17::puzzles())
        .with(d18::puzzles())
        .with(d19::puzzles())
}
