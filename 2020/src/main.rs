use structopt::StructOpt;

mod support;

mod y2018;
mod y2019;
mod y2020;
mod y2021;
mod y2022;
mod y2023;
mod y2024;

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
    /// Run part 2 only
    #[structopt(short, long)]
    part2only: bool,
}

fn main()
{
    let args = CmdArgs::from_args();

    let puzzles = PuzzleSet::new()
        .with(y2018::puzzles())
        .with(y2019::puzzles())
        .with(y2020::puzzles())
        .with(y2021::puzzles())
        .with(y2022::puzzles())
        .with(y2023::puzzles())
        .with(y2024::puzzles())
    ;

    puzzles.run(args.year, args.day, args.part2only);
}
