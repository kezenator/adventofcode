use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

fn single_move(grid: &CharGrid, ch: char, dir: Point) -> CharGrid
{
    let mut result = CharGrid::new_from_fill(grid.get_width() as usize, grid.get_height() as usize, '.');

    for p in result.all_points()
    {
        let cur = grid.get_char(&p);

        if grid.get_char(&p) == ch
        {
            let new_pos = Point::new(
                (p.x + dir.x) % grid.get_width(),
                (p.y + dir.y) % grid.get_height());

            if grid.get_char(&new_pos) == '.'
            {
                result.put_char(&new_pos, ch);
            }
            else
            {
                result.put_char(&p, ch);
            }
        }
        else if (cur == '>') || (cur == 'v')
        {
            result.put_char(&p, cur);
        }
    }

    result
}

fn move_cucumbers(grid: &CharGrid) -> CharGrid
{
    let grid = single_move(&grid, '>', Point::new(1, 0));
    single_move(&grid, 'v', Point::new(0, 1))
}

fn part_1(input: &str) -> usize
{
    let mut cur = CharGrid::new_from_input(input, '.');
    let mut moves = 0;

    loop
    {
        let next = move_cucumbers(&cur);
        moves += 1;

        if next == cur
        {
            return moves;
        }
        cur = next;
    }
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(25)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 58,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 353,
        })
        .final_gift()
}
