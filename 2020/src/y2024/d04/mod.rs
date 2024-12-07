
use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");

fn starts_word(grid: &CharGrid, word: &str, start: &Point, dir: &Point) -> bool
{
    word.chars().enumerate()
        .all(|(i, ch)|
        {
            let p = *start + (i as i64) * *dir;
            grid.is_point_in_bounds(&p) && (grid.get_char(&p) == ch)
        })
}

fn count_words(grid: &CharGrid, word: &str, start: &Point) -> usize
{
    Point::directions_8().into_iter()
        .filter(|dir| starts_word(grid, word, start, dir))
        .count()
}

fn num_words(input: &str, word: &str) -> usize
{
    let grid = CharGrid::new_from_input(input, '.');

    grid.all_points().into_iter()
        .map(|p| count_words(&grid, word, &p))
        .sum()
}

fn part_1(input: &str) -> usize
{
    num_words(input, "XMAS")
}

fn is_xmas_ends(grid: &CharGrid, start: &Point, dir: &Point) -> bool
{
    let a = *start + *dir;
    let b = *start - *dir;
    if grid.is_point_in_bounds(&a) && grid.is_point_in_bounds(&b)
    {
        let cha = grid.get_char(&a);
        let chb = grid.get_char(&b);

        if (cha == 'M' && chb == 'S') || (cha == 'S' && chb == 'M')
        {
            return true;
        }
    }
    return false;
}

fn part_2(input: &str) -> usize
{
    let grid = CharGrid::new_from_input(input, '.');

    grid.all_points().into_iter()
        .filter(|p|
            (grid.get_char(p) == 'A')
            && is_xmas_ends(&grid, p, &Point::new(1, 1))
            && is_xmas_ends(&grid, p, &Point::new(-1, 1)))
        .count()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(4)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 18,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 2378,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 9,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 1796,
        })
}
