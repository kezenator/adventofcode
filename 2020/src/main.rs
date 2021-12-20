use structopt::StructOpt;

mod support;

//mod y2018;
//mod y2019;
//mod y2020;
mod y2021;

use support::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "aocrust", about = "A collection of Advent of Code (https://adventofcode.com/) puzzles solved in rust")]
struct CmdArgs
{
    /// The year to run. All years if not specified.
    #[structopt(short, long)]
    year: Option<usize>,
    /// The day to run. All days if not specified.
    #[structopt(short, long)]
    day: Option<usize>,
}

fn main()
{
    let args = CmdArgs::from_args();

    let puzzles = PuzzleSet::new()
        //.with(y2018::puzzles())
        //.with(y2019::puzzles())
        //.with(y2020::puzzles())
        .with(y2021::puzzles())
    ;

    puzzles.run(args.year, args.day);
}
