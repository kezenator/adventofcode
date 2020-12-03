pub fn string_split_into_runs(s: &str) -> Vec<String>
{
    let mut result = Vec::new();
    let mut last = None;
    let mut cur = String::new();

    for ch in s.chars()
    {
        match last
        {
            None =>
            {
                cur.push(ch);
                last = Some(ch);
            },
            Some(last_ch) =>
            {
                if ch == last_ch
                {
                    cur.push(ch);
                }
                else
                {
                    result.push(cur);
                    cur = String::new();
                    cur.push(ch);
                    last = Some(ch);
                }
            },
        }
    }

    if !cur.is_empty()
    {
        result.push(cur)
    }

    result
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_scan()
    {
        assert_eq!(string_split_into_runs(""), Vec::<String>::new());
        assert_eq!(string_split_into_runs("a"), vec!["a".to_owned()]);
        assert_eq!(string_split_into_runs("aa"), vec!["aa".to_owned()]);
        assert_eq!(string_split_into_runs("ab"), vec!["a".to_owned(), "b".to_owned()]);
        assert_eq!(string_split_into_runs("aab"), vec!["aa".to_owned(), "b".to_owned()]);
        assert_eq!(string_split_into_runs("abb"), vec!["a".to_owned(), "bb".to_owned()]);
        assert_eq!(string_split_into_runs("aabb"), vec!["aa".to_owned(), "bb".to_owned()]);
        assert_eq!(string_split_into_runs("aabbcaaddd"), vec!["aa".to_owned(), "bb".to_owned(), "c".to_owned(), "aa".to_owned(), "ddd".to_owned()]);
    }
}
