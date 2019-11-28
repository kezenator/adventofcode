use std::collections::HashSet;

fn sum(input: &str) -> i32
{
    input.split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .fold(0, |a, b| a + b)
}

fn first_twice(input: &str) -> i32
{
    let changes = input.split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    
    let mut sum = 0;
    let mut found = HashSet::<i32>::new();
    found.insert(sum);

    loop
    {
        for change in changes.iter()
        {
            sum += change;
            if found.contains(&sum)
            {
                return sum;
            }
            found.insert(sum);
        }
    }
}

fn main()
{
    assert_eq!(sum("+1\n+1\n+1\n"), 3);
    assert_eq!(sum("+1\n+1\n-2\n"), 0);
    assert_eq!(sum("-1\n-2\n-3\n"), -6);

    println!("Answer #1={}", sum(include_str!("input_1.txt")));

    assert_eq!(first_twice("+1\n-1\n"), 0);
    assert_eq!(first_twice("+3\n+3\n+4\n-2\n-4\n"), 10);
    assert_eq!(first_twice("-6\n+3\n+8\n+5\n-6\n"), 5);
    assert_eq!(first_twice("+7\n+7\n-2\n-7\n-4\n"), 14);

    println!("Answer #2={}", first_twice(include_str!("input_1.txt")));
}