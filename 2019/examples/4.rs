const INPUT: &str = "236491-713787";

fn meets_rules_1(input: i64) -> bool
{
    let val = input.to_string();
    let mut found_dup = false;

    for i in 1..6
    {
        if val.chars().nth(i-1).unwrap() > val.chars().nth(i).unwrap()
        {
            return false;
        }

        if val.chars().nth(i-1).unwrap() == val.chars().nth(i).unwrap()
        {
            found_dup = true;
        }
    }

    if !found_dup
    {
        return false;
    }

    true
}

fn part_1(input: &str) -> i64
{
    let parts = input
        .split("-")
        .map(|a| a.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut count = 0;

    for i in parts[0] .. (parts[1] + 1)
    {
        if meets_rules_1(i)
        {
            count += 1;
        }
    }
    count
}

fn meets_rules_2(input: i64) -> bool
{
    let val = input.to_string();
    let mut found_dup = false;
    let mut cur_run = 1;

    for i in 1..6
    {
        if val.chars().nth(i-1).unwrap() > val.chars().nth(i).unwrap()
        {
            return false;
        }

        if val.chars().nth(i-1).unwrap() == val.chars().nth(i).unwrap()
        {
            cur_run += 1;
        }
        else
        {
            if cur_run == 2
            {
                found_dup = true;
            }
            cur_run = 1;
        }
    }

    if cur_run == 2
    {
        found_dup = true;
    }

    if !found_dup
    {
        return false;
    }

    true
}

fn part_2(input: &str) -> i64
{
    let parts = input
        .split("-")
        .map(|a| a.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut count = 0;

    for i in parts[0] .. (parts[1] + 1)
    {
        if meets_rules_2(i)
        {
            count += 1;
        }
    }
    count
}

fn main()
{
    assert_eq!(meets_rules_1(111111), true);
    assert_eq!(meets_rules_1(122345), true);
    assert_eq!(meets_rules_1(111123), true);

    assert_eq!(meets_rules_1(135679), false);
    assert_eq!(meets_rules_1(223450), false);
    assert_eq!(meets_rules_1(123789), false);

    let answer_1 = part_1(INPUT);
    println!("Answer #1={}", answer_1);
    assert_eq!(answer_1, 1169);

    assert_eq!(meets_rules_2(112233), true);
    assert_eq!(meets_rules_2(123444), false);
    assert_eq!(meets_rules_2(111122), true);

    let answer_2 = part_2(INPUT);
    println!("Answer #2={}", answer_2);
    assert_eq!(answer_2, 757);
}
