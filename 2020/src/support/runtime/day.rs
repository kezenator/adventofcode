use super::Puzzle;
use super::Answer;
use super::PuzzleDayRunner;

pub fn puzzle_day(day: usize) -> PuzzleDayBuilder
{
    PuzzleDayBuilder
    {
        day: day,
        part1_examples: Vec::new(),
    }
}

pub struct PuzzleDay
{
    day: usize,
    part1_examples: Vec<Puzzle>,
    part1: Puzzle,
    part2_examples: Vec<Puzzle>,
    part2: Puzzle,
}

impl PuzzleDay
{
    pub fn get_day(&self) -> usize
    {
        self.day
    }

    pub fn run(&self, runner: PuzzleDayRunner)
    {
        for example in self.part1_examples.iter()
        {
            runner.run("Example", example);
        }

        runner.run("Part 1", &self.part1);

        for example in self.part2_examples.iter()
        {
            runner.run("Example", example);
        }

        runner.run("Part 2", &self.part2);
    }
}

pub struct PuzzleDayBuilder
{
    day: usize,
    part1_examples: Vec<Puzzle>,
}

impl PuzzleDayBuilder
{
    pub fn example<T, U, F>(mut self, puzzle: F) -> Self
        where T: 'static + ToString,
            U: 'static + ToString,
            F: 'static + Fn() -> Answer<T, U>
    {
        self.part1_examples.push(Box::new(move || puzzle().into()));
        self
    }

    pub fn part_1<T, U, F>(self, puzzle: F) -> PuzzleDayBuilderPart1Done
        where T: 'static + ToString,
            U: 'static + ToString,
            F: 'static + Fn() -> Answer<T, U>
    {
        PuzzleDayBuilderPart1Done
        {
            day: self.day,
            part1_examples: self.part1_examples,
            part1: Box::new(move || puzzle().into()),
            part2_examples: Vec::new(),
        }
    }
}

pub struct PuzzleDayBuilderPart1Done
{
    day: usize,
    part1_examples: Vec<Puzzle>,
    part1: Puzzle,
    part2_examples: Vec<Puzzle>,
}

impl PuzzleDayBuilderPart1Done
{
    pub fn example<T, U, F>(mut self, puzzle: F) -> Self
        where T: 'static + ToString,
            U: 'static + ToString,
            F: 'static + Fn() -> Answer<T, U>
    {
        self.part2_examples.push(Box::new(move || puzzle().into()));
        self
    }

    pub fn part_2<T, U, F>(self, puzzle: F) -> PuzzleDay
        where T: 'static + ToString,
            U: 'static + ToString,
            F: 'static + Fn() -> Answer<T, U>
    {
        PuzzleDay
        {
            day: self.day,
            part1_examples: self.part1_examples,
            part1: self.part1,
            part2_examples: self.part2_examples,
            part2: Box::new(move || puzzle().into())
        }
    }
}
