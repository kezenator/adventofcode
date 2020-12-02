use std::str::FromStr;
use std::fmt::Debug;
use crate::support::tuple_append::TupleAppend;

pub fn scan<'a>(s: &'a str) -> ScanTokenize<'a, ()>
{
    ScanTokenize::<'a, ()>
    {
        tuple: (),
        remaining: s,
    }
}

#[must_use]
pub struct ScanTokenize<'a, T>
{
    tuple: T,
    remaining: &'a str,
}

impl<'a, T> ScanTokenize<'a, T>
{
    pub fn skip(self, num: usize) -> ScanTokenize<'a, T>
    {
        ScanTokenize::<'a, T>
        {
            tuple: self.tuple,
            remaining: self.remaining.split_at(num).1
        }
    }

    #[allow(dead_code)]
    pub fn skip_ws(self) -> ScanTokenize<'a, T>
    {
        let pos = self.remaining.find(|c: char| !c.is_whitespace()).expect("Error parsing input: no whitespace found");

        self.skip(pos)
    }

    pub fn take_skip(self, take: usize, skip: usize) -> ScanParse<'a, T>
    {
        let to_parse = self.remaining.split_at(take).0;
        let new_remaining = self.remaining.split_at(take + skip).1;

        ScanParse
        {
            tuple: self.tuple,
            to_parse: to_parse,
            remaining: new_remaining,
        }
    }

    #[allow(dead_code)]
    pub fn take(self, num: usize) -> ScanParse<'a, T>
    {
        self.take_skip(num, 0)
    }

    #[allow(dead_code)]
    pub fn take_digits(self) -> ScanParse<'a, T>
    {
        let pos = self.remaining.find(|c: char| !c.is_ascii_digit()).expect("Error parsing input: no non-digits found");

        self.take_skip(pos, 0)
    }

    pub fn until(self, s: &str) -> ScanParse<'a, T>
    {
        let pos = self.remaining.find(s).expect("Error parsing input: Until str not found");

        self.take_skip(pos, s.len())
    }

    #[allow(dead_code)]
    pub fn until_whitespace(self) -> ScanParse<'a, T>
    {
        let first_ws = self.remaining.find(|c: char| c.is_ascii_whitespace()).expect("Error parsing input: no whitespace found");
        let num_ws = self.remaining.split_at(first_ws).1.find(|c: char| !c.is_ascii_whitespace()).expect("Error parsing input: no non-whitespace found after whitespace");

        self.take_skip(first_ws, num_ws)
    }

    pub fn remaining(self) -> ScanParseFinal<'a, T>
    {
        ScanParseFinal
        {
            tuple: self.tuple,
            to_parse: self.remaining,
        }
    }
}

pub struct ScanParse<'a, T>
{
    tuple: T,
    to_parse: &'a str,
    remaining: &'a str,
}

impl<'a, T> ScanParse<'a, T>
{
    pub fn parse<V>(self) -> ScanTokenize<'a, T::ResultType>
        where T: TupleAppend<V>,
            V: 'static + Debug + FromStr,
            V::Err: Debug
    {
        let parsed_val = self.to_parse.parse::<V>().expect("Error parsing input: Could not parse value");
        let new_tuple = self.tuple.append(parsed_val);

        ScanTokenize::<T::ResultType>
        {
            tuple: new_tuple,
            remaining: self.remaining,
        }
    }
}

pub struct ScanParseFinal<'a, T>
{
    tuple: T,
    to_parse: &'a str,
}

impl<'a, T> ScanParseFinal<'a, T>
{
    pub fn parse<V>(self) -> T::ResultType
        where T: TupleAppend<V>,
            V: 'static + Debug + FromStr,
            V::Err: Debug
    {
        let parsed_val = self.to_parse.parse::<V>().expect("Error parsing input: Could not parse value");
        let new_tuple = self.tuple.append(parsed_val);

        new_tuple
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_scan()
    {
        let expected: (String, char, char, String, i64, String, i64) = ("asdf".to_owned(), 'x', '*', "abc".to_owned(), 123, "fgh".to_owned(), 456);

        let scanned = scan("asdf x:---*  \n abcde123fgh 456")
            .until(" ").parse::<String>()
            .until(":").parse::<char>()
            .skip(3)
            .take(1).parse::<char>()
            .skip_ws()
            .take_skip(3, 2).parse::<String>()
            .take_digits().parse::<i64>()
            .until_whitespace().parse::<String>()
            .remaining().parse::<i64>();
            
        assert_eq!(expected, scanned);
    }
}