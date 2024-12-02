use std::str::FromStr;
use std::fmt::Debug;

pub fn input_to_lines(input: &str) -> Vec<String>
{
    let mut result = input
        .replace("\r\n", "\n")
        .split("\n")
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();

    if let Some(last) = result.last()
    {
        if last.is_empty()
        {
            result.pop();
        }
    }

    result
}

pub fn input_to_groups(input: &str) -> Vec<Vec<String>>
{
    let mut result = Vec::new();
    let mut cur = Vec::new();

    for line in input_to_lines(input)
    {
        if line.is_empty()
        {
            result.push(cur);
            cur = Vec::new();
        }
        else
        {
            cur.push(line);
        }
    }

    if !cur.is_empty()
    {
        result.push(cur);
    }

    result
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

pub fn input_to_lines_mapped<F, T>(input: &str, mapper: F) -> Vec<T>
    where F: Fn(&str) -> T
{
    input_to_lines(input)
        .drain(..)
        .map(|s| mapper(&s))
        .collect()
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_input_to_lines()
    {
        assert_eq!(input_to_lines("a\nb"), vec!["a".to_owned(), "b".to_owned()]);
        assert_eq!(input_to_lines("a\nb\n"), vec!["a".to_owned(), "b".to_owned()]);
        assert_eq!(input_to_lines("a\r\nb"), vec!["a".to_owned(), "b".to_owned()]);
        assert_eq!(input_to_lines("a\r\nb\r\n"), vec!["a".to_owned(), "b".to_owned()]);
    }

    #[test]
    fn test_input_to_groups()
    {
        assert_eq!(input_to_groups("a\nb"), vec![vec!["a".to_owned(), "b".to_owned()]]);
        assert_eq!(input_to_groups("a\nb\n\nc\nd\n\n"), vec![vec!["a".to_owned(), "b".to_owned()], vec!["c".to_owned(), "d".to_owned()]]);
    }

    #[test]
    fn test_input_to_lines_parsed()
    {
        assert_eq!(input_to_lines_parsed::<String>("a\nb"), vec!["a".to_owned(), "b".to_owned()]);
        assert_eq!(input_to_lines_parsed::<char>("a\nb"), vec!['a', 'b']);
        assert_eq!(input_to_lines_parsed::<u64>("123\n456"), vec![123, 456]);
    }

    #[test]
    fn test_input_to_lines_mapped()
    {
        assert_eq!(input_to_lines_mapped("a\nb", |l| l.to_uppercase()), vec!["A".to_owned(), "B".to_owned()]);
    }
}