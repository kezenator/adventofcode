use std::collections::BTreeMap;
use std::time::Instant;

mod answer;
mod day;
mod year;

pub use answer::*;
pub use day::*;
pub use year::*;

pub type Puzzle = Box<dyn Fn() -> StrAnswer>;

pub struct PuzzleSet
{
    years: BTreeMap<usize, PuzzleYear>,
}

impl PuzzleSet
{
    pub fn new() -> Self
    {
        PuzzleSet
        {
            years: BTreeMap::new()
        }
    }

    pub fn with(mut self, year: PuzzleYear) -> Self
    {
        let year_num = year.get_year();
        let previous = self.years.insert(year_num, year);

        assert!(previous.is_none());

        self
    }

    pub fn run(&self)
    {
        for (&year, puzzle_year) in self.years.iter()
        {
            puzzle_year.run(PuzzleYearRunner::new(year));
        }
    }
}

pub struct PuzzleYearRunner
{
    year: usize,
}

impl PuzzleYearRunner
{
    fn new(year: usize) -> Self
    {
        PuzzleYearRunner
        {
            year,
        }
    }

    pub fn for_day(&self, day: usize) -> PuzzleDayRunner
    {
        println!("---- {:04} Day {:02} -----------------------------------------", self.year, day);

        PuzzleDayRunner
        {
            _year: self.year,
            _day: day,
        }
    }
}

pub struct PuzzleDayRunner
{
    _year: usize,
    _day: usize,
}

impl PuzzleDayRunner
{
    pub fn run(&self, name: &str, puzzle: &Puzzle)
    {
        let start = Instant::now();

        let answer = puzzle();

        let duration = Instant::now().duration_since(start);

        println!("[ {:10} ] [ {:3}.{:06} s] => [ {:20} ]",
            name,
            duration.as_secs(),
            duration.subsec_micros(),
            answer.calculated);

        if answer.calculated != answer.expected
        {
            println!("   *** Expected {}", answer.expected);
        }

        assert!(answer.calculated == answer.expected);
    }
}
