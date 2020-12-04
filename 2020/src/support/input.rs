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

pub fn input_to_lines_parsed<T>(input: &str) -> Vec<T>
    where T: FromStr,
        T::Err: Debug
{
    input_to_lines(input)
        .drain(..)
        .map(|s| s.parse::<T>().unwrap())
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
    fn test_input_to_lines_parsed()
    {
        assert_eq!(input_to_lines_parsed::<String>("a\nb"), vec!["a".to_owned(), "b".to_owned()]);
        assert_eq!(input_to_lines_parsed::<char>("a\nb"), vec!['a', 'b']);
        assert_eq!(input_to_lines_parsed::<u64>("123\n456"), vec![123, 456]);
    }
}