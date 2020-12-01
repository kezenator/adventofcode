use std::collections::BTreeMap;
use std::str::FromStr;
use std::fmt::Debug;
use std::time::Instant;

pub struct Answer<T, U>
{
    pub calculated: T,
    pub expected: U,
}

pub struct StrAnswer
{
    pub calculated: String,
    pub expected: String,
}

impl<T: ToString, U: ToString> Into<StrAnswer> for Answer<T, U>
{
    fn into(self) -> StrAnswer
    {
        StrAnswer
        {
            calculated: self.calculated.to_string(),
            expected: self.expected.to_string(),
        }
    }
}

pub type Puzzle = Box<dyn Fn() -> StrAnswer>;

pub struct PuzzleSet
{
    funcs: BTreeMap<String, Puzzle>,
}

impl PuzzleSet
{
    pub fn new() -> Self
    {
        PuzzleSet{ funcs: BTreeMap::new() }
    }

    pub fn register<T, U, F>(&mut self, name: &str, func: F)
        where T: 'static + ToString,
            U: 'static + ToString,
            F: 'static + Fn() -> Answer<T, U>
    {
        let existing = self.funcs.insert(name.to_owned(), Box::new(move || func().into()));
        assert!(existing.is_none());
    }

    pub fn run(&self)
    {
        for entry in self.funcs.iter()
        {
            let start = Instant::now();

            let answer = entry.1();

            let duration = Instant::now().duration_since(start);

            println!("[ {:10} ] [ {:3}.{:06} s] => [ {:10} ]",
                entry.0,
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
}

pub fn input_to_lines(input: &str) -> Vec<String>
{
    input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect()
}

pub fn input_to_lines_parsed<T>(input: &str) -> Vec<T>
    where T: FromStr,
        T::Err: Debug
{
    input_to_lines(input)
        .drain(..)
        .map(|s| s.parse::<T>().unwrap())
        .collect()
}
