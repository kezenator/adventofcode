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

    pub fn run(&self, year_filter: Option<usize>, day_filter: Option<usize>)
    {
        for (&year, puzzle_year) in self.years.iter()
        {
            if year_filter.is_none()
                || year_filter == Some(year)
            {
                puzzle_year.run(PuzzleYearRunner::new(year, day_filter));
            }
        }
    }
}

pub struct PuzzleYearRunner
{
    year: usize,
    day_filter: Option<usize>,
}

impl PuzzleYearRunner
{
    fn new(year: usize, day_filter: Option<usize>) -> Self
    {
        PuzzleYearRunner
        {
            year,
            day_filter,
        }
    }

    pub fn include_day(&self, day: usize) -> bool
    {
        self.day_filter.is_none() || self.day_filter == Some(day)
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

        print!("[ {:10} ] [ {:3}.{:06} s] => [",
            name,
            duration.as_secs(),
            duration.subsec_micros());

        if (answer.calculated.len() <= 20)
            && answer.calculated.chars().position(|c| c == '\n').is_none()
        {
            println!(" {:20} ]", answer.calculated);
        }
        else
        {
            println!("");
            for l in crate::support::input_to_lines(&answer.calculated)
            {
                println!("{:35}{}", "", l);
            }
            println!("{:34}]", "");
        }

        if answer.calculated != answer.expected
        {
            println!("   *** Expected {}", answer.expected);
        }

        assert!(answer.calculated == answer.expected);
    }
}
