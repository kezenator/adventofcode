use std::collections::BTreeMap;

use super::day::PuzzleDay;
use super::PuzzleYearRunner;

pub fn puzzle_year(year: usize) -> PuzzleYear
{
    PuzzleYear
    {
        year: year,
        days: BTreeMap::new(),
    }
}

pub struct PuzzleYear
{
    year: usize,
    days: BTreeMap<usize, PuzzleDay>,
}

impl PuzzleYear
{
    pub fn get_year(&self) -> usize
    {
        self.year
    }

    pub fn with(mut self, day: PuzzleDay) -> Self
    {
        let day_num = day.get_day();
        let previous = self.days.insert(day_num, day);

        assert!(previous.is_none());

        self
    }

    pub fn run(&self, runner: PuzzleYearRunner)
    {
        for (&day, day_puzzles) in self.days.iter()
        {
            day_puzzles.run(runner.for_day(day));
        }
    }
}