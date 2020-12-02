mod support;
mod d01;
mod d02;

use support::*;

fn main()
{
    let mut puzzles = PuzzleSet::new();

    d01::puzzles(&mut puzzles);
    d02::puzzles(&mut puzzles);

    puzzles.run();
}
