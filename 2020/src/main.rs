mod support;

mod y2019;
mod y2020;

use support::*;

fn main()
{
    let puzzles = PuzzleSet::new()
        .with(y2019::puzzles())
        .with(y2020::puzzles())
    ;

    puzzles.run();
}
