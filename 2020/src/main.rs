mod support;

mod y2019;
mod y2020;

use support::*;

fn main()
{
    let mut puzzles = PuzzleSet::new();

    y2019::puzzles(&mut puzzles);
    y2020::puzzles(&mut puzzles);

    puzzles.run();
}
