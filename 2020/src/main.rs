mod support;
mod d01;

use support::*;

fn main()
{
    let mut puzzles = PuzzleSet::new();

    d01::register(&mut puzzles);

    puzzles.run();
}
