use std::collections::BTreeMap;
use std::time::Instant;

mod answer;
mod day;
mod input;
mod year;

pub use answer::*;
pub use day::*;
pub use year::*;

pub type PuzzleExample = Box<dyn Fn() -> StrAnswer>;
pub type PuzzleWithInput = Box<dyn Fn(&str) -> StrAnswer>;

use input::InputCache;

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

    pub fn run(&self, year_filter: Option<usize>, day_filter: Option<usize>, part2only: bool)
    {
        let mut input_cache = InputCache::new();

        for (&year, puzzle_year) in self.years.iter()
        {
            if year_filter.is_none()
                || year_filter == Some(year)
            {
                puzzle_year.run(PuzzleYearRunner::new(year, day_filter, part2only, &mut input_cache));
            }
        }
    }
}

pub struct PuzzleYearRunner<'a>
{
    year: usize,
    day_filter: Option<usize>,
    part2only: bool,
    input_cache: &'a mut InputCache,
}

impl<'a> PuzzleYearRunner<'a>
{
    fn new(year: usize, day_filter: Option<usize>, part2only: bool, input_cache: &'a mut InputCache) -> Self
    {
        PuzzleYearRunner
        {
            year,
            day_filter,
            part2only,
            input_cache,
        }
    }

    pub fn include_day(&self, day: usize) -> bool
    {
        self.day_filter.is_none() || self.day_filter == Some(day)
    }

    pub fn for_day(&mut self, day: usize) -> PuzzleDayRunner
    {
        println!("---- {:04} Day {:02} -----------------------------------------", self.year, day);

        PuzzleDayRunner
        {
            _year: self.year,
            _day: day,
            _part2only: self.part2only,
            _input: self.input_cache.get(self.year, day),
        }
    }
}

pub struct PuzzleDayRunner
{
    _year: usize,
    _day: usize,
    _part2only: bool,
    _input: String,
}

impl PuzzleDayRunner
{
    pub fn part2only(&self) -> bool
    {
        self._part2only
    }

    pub fn run_with_input(&self, name: &str, puzzle: &PuzzleWithInput)
    {
        self.run_internal(name, || puzzle(&self._input));
    }

    pub fn run_example(&self, name: &str, puzzle: &PuzzleExample)
    {
        self.run_internal(name, puzzle);
    }

    fn run_internal<F>(&self, name: &str, puzzle: F)
        where F: Fn() -> StrAnswer
    {
        let start = Instant::now();

        let answer = puzzle();

        let duration = Instant::now().duration_since(start);

        print!("[ {:11} ] [ {:3}.{:06} s] => [",
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
