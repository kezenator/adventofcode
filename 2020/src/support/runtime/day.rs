use super::PuzzleExample;
use super::PuzzleWithInput;
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
    part1_examples: Vec<PuzzleExample>,
    part1: PuzzleWithInput,
    part2_examples: Vec<PuzzleExample>,
    part2: Option<PuzzleWithInput>,
}

impl PuzzleDay
{
    pub fn get_day(&self) -> usize
    {
        self.day
    }

    pub fn run(&self, runner: PuzzleDayRunner)
    {
        if !runner.part2only()
        {
            for example in self.part1_examples.iter()
            {
                runner.run_example("Example", example);
            }

            runner.run_with_input("Part 1", &self.part1);
        }

        for example in self.part2_examples.iter()
        {
            runner.run_example("Example", example);
        }
        
        if let Some(part2) = &self.part2
        {
            runner.run_with_input("Part 2", &part2);
        }
    }
}

pub struct PuzzleDayBuilder
{
    day: usize,
    part1_examples: Vec<PuzzleExample>,
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
            F: 'static + Fn(&str) -> Answer<T, U>
    {
        PuzzleDayBuilderPart1Done
        {
            day: self.day,
            part1_examples: self.part1_examples,
            part1: Box::new(move |input| puzzle(input).into()),
            part2_examples: Vec::new(),
        }
    }
}

pub struct PuzzleDayBuilderPart1Done
{
    day: usize,
    part1_examples: Vec<PuzzleExample>,
    part1: PuzzleWithInput,
    part2_examples: Vec<PuzzleExample>,
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
            F: 'static + Fn(&str) -> Answer<T, U>
    {
        PuzzleDay
        {
            day: self.day,
            part1_examples: self.part1_examples,
            part1: self.part1,
            part2_examples: self.part2_examples,
            part2: Some(Box::new(move |input| puzzle(input).into())),
        }
    }

    pub fn final_gift(self) -> PuzzleDay
    {
        assert_eq!(self.day, 25);
        assert!(self.part2_examples.is_empty());

        PuzzleDay
        {
            day: self.day,
            part1_examples: self.part1_examples,
            part1: self.part1,
            part2_examples: self.part2_examples,
            part2: None,
        }
    }
}
