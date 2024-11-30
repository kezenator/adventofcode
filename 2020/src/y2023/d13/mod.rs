use crate::support::*;
use itertools::*;

const EXAMPLE: &str = include_str!("example.txt");

enum Reflection
{
    Horizontal { rows_above: i64 },
    Vertical { columns_to_left: i64 },
}

impl Reflection
{
    fn summary(&self) -> i64
    {
        match self
        {
            Self::Horizontal { rows_above } => 100 * rows_above,
            Self::Vertical { columns_to_left } => *columns_to_left,
        }
    }
}

fn find_reflection_in_dir<F>(num_differences: usize, a_size: i64, b_size: i64, get: &F) -> Option<i64>
    where F: Fn(i64, i64) -> char
{
    let mut result = None;

    for ai in 1..a_size
    {
        let mut ai_pairs = Vec::new();
        for c in 0..a_size
        {
            let ai_lower = ai - c - 1;
            let ai_upper = ai + c;
            if (ai_lower >= 0) && (ai_upper < a_size)
            {
                ai_pairs.push((ai_lower, ai_upper))
            }
        }

        let mut count_diff = 0;

        for bi in 0..b_size
        {
            for (ai_lower, ai_upper) in ai_pairs.iter()
            {
                if get(*ai_lower, bi) != get(*ai_upper, bi)
                {
                    count_diff += 1;
                }
            }
        }

        if count_diff == num_differences
        {
            if result.is_some()
            {
                unreachable!();
            }

            result = Some(ai);
        }
    }
    result
}

fn find_reflection(grid: &CharGrid, num_differences: usize) -> Reflection
{
    if let Some(i) = find_reflection_in_dir(num_differences, grid.get_width(), grid.get_height(), &|a, b| grid.get_char(&Point::new(a, b)))
    {
        return Reflection::Vertical { columns_to_left: i };
    }
    if let Some(i) = find_reflection_in_dir(num_differences, grid.get_height(), grid.get_width(), &|a, b| grid.get_char(&Point::new(b, a)))
    {
        return Reflection::Horizontal { rows_above: i };
    }
    panic!("No reflection found");
}

fn summarize(input: &str, num_differences: usize) -> i64
{
    let grids = input_to_groups(input).into_iter()
        .map(|lines| CharGrid::new_from_input(&lines.into_iter().join("\n"), '.'))
        .collect_vec();

    grids.iter()
        .map(|g| find_reflection(g, num_differences).summary())
        .sum()
}

fn part_1(input: &str) -> i64
{
    summarize(input, 0)
}

fn part_2(input: &str) -> i64
{
    summarize(input, 1)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(13)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 405,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 27502,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 400,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 31947,
        })
}
