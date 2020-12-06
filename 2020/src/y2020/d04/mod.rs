use std::collections::HashMap;
use crate::support::*;

const EXAMPLE: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n\nhcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm\n\nhcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in\n";
const INPUT: &str = include_str!("input.txt");

fn parse_passports(input: &str) -> Vec<HashMap<String, String>>
{
    let mut result = Vec::new();

    for group in input_to_groups(input)
    {
        let mut pport = HashMap::new();

        for line in group
        {
            let (fields,) = scan(&line).remaining().parse_vec::<String>(" ");

            for field in fields
            {
                let (name, val) = scan(&field)
                    .until(":").parse::<String>()
                    .remaining().parse::<String>();

                    pport.insert(name, val);
            }
        }

        result.push(pport);
    }

    result
}

fn is_length(s: &str, len: usize) -> bool
{
    s.len() == len
}

fn is_int(s: &str, min: u64, max: u64) -> bool
{
    match s.parse::<u64>()
    {
        Ok(x) =>
        {
            if min <= x && x <= max
            {
                true
            }
            else
            {
                false
            }
        },
        Err(_) =>
        {
            false
        }
    }
}

fn valid_height(s: &str) -> bool
{
    let (val, units) = scan(s)
        .take_digits().parse::<u64>()
        .remaining().parse::<String>();

    match units.as_str()
    {
        "cm" => 150 <= val && val <= 193,
        "in" => 59 <= val && val <= 76,
        _ => false,
    }
}

fn valid_hair(s: &str) -> bool
{
    let (ch, rest) = scan(s)
        .take(1).parse::<char>()
        .remaining().parse::<String>();

    (ch == '#')
        && (rest.len() == 6)
        && rest.chars().filter(|&c| ((c >= '0' && c <= '9') || (c >= 'a' && c <= 'f'))).count() == 6
}

fn valid_eye(s: &str) -> bool
{
    match s
    {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false,
    }
}

fn valid_pid(s: &str) -> bool
{
    let (digits, rest) = scan(s)
        .take_digits().parse::<String>()
        .remaining().parse::<String>();

    digits.len() == 9 && rest.len() == 0
}

fn valid_1(pport: &HashMap<String, String>) -> bool
{
    pport.contains_key("byr")
        && pport.contains_key("iyr")
        && pport.contains_key("eyr")
        && pport.contains_key("hgt")
        && pport.contains_key("hcl")
        && pport.contains_key("ecl")
        && pport.contains_key("pid")
}

fn valid_2(pport: &HashMap<String, String>) -> bool
{
    pport.iter()
        .filter(|(name, value)| !match name.as_str()
            {
                "byr" =>
                {
                    is_length(value, 4) && is_int(value, 1920, 2002)
                },
                "iyr" =>
                {
                    is_length(value, 4) && is_int(value, 2010, 2020)
                },
                "eyr" =>
                {
                    is_length(value, 4) && is_int(value, 2020, 2030)
                },
                "hgt" =>
                {
                    valid_height(value)
                },
                "hcl" =>
                {
                    valid_hair(value)
                },
                "ecl" =>
                {
                    valid_eye(value)
                },
                "pid" =>
                {
                    valid_pid(value)
                },
                "cid" =>
                {
                    // Ignore
                    true
                },
                _ =>
                {
                    // Invalid field
                    false
                },
            })
        .count() == 0
}

pub fn part_1(input: &str) -> usize
{
    parse_passports(input)
        .iter()
        .filter(|p| valid_1(p))
        .count()
}

pub fn part_2(input: &str) -> usize
{
    parse_passports(input)
        .iter()
        .filter(|p| valid_1(p) && valid_2(p))
        .count()
}

pub fn puzzles(puzzles: &mut PuzzleSet)
{
    puzzles.register("y2020.d04.e1", || Answer {
        calculated: part_1(EXAMPLE),
        expected: 2,
    });

    puzzles.register("y2020.d04.p1", || Answer {
        calculated: part_1(INPUT),
        expected: 206,
    });

    puzzles.register("y2020.d04.p2", || Answer {
        calculated: part_2(INPUT),
        expected: 123,
    });
}
