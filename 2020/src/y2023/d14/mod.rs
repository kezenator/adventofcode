use crate::support::*;
use std::collections::HashMap;

const EXAMPLE: &str = include_str!("example.txt");

fn roll(grid: &mut CharGrid, dir: Point)
{
    loop
    {
        let mut changed = false;

        for p in grid.all_points()
        {
            if grid.get_char(&p) == 'O'
            {
                let mut p = p;
                loop
                {
                    let np = p + dir;
                    if !grid.is_point_in_bounds(&np)
                        || (grid.get_char(&np) != '.')
                    {
                        break;
                    }
                    grid.put_char(&p, '.');
                    grid.put_char(&np, 'O');
                    p = np;
                    changed = true;
                }
            }
        }

        if !changed
        {
            return;
        }
    }
}

fn spin_cycle(grid: &mut CharGrid)
{
    roll(grid, Point::new(0, -1));
    roll(grid, Point::new(-1, 0));
    roll(grid, Point::new(0, 1));
    roll(grid, Point::new(1, 0));
}

fn calc_load_on_north_beam(grid: &CharGrid) -> i64
{
    let mut result = 0;
    for p in grid.all_points()
    {
        if grid.get_char(&p) == 'O'
        {
            result += grid.get_height() - p.y;
        }
    }
    result
}

fn part_1(input: &str) -> i64
{
    let mut grid = CharGrid::new_from_input(input, '.');
    roll(&mut grid, Point::new(0, -1));
    calc_load_on_north_beam(&grid)
}

fn part_2(input: &str) -> i64
{
    const CYCLES: i64 = 1000000000;

    let mut grid = CharGrid::new_from_input(input, '.');

    // Keep performing spin cycles until we get back to the same point
    let mut seen_states = HashMap::new();
    let mut states_in_order = Vec::new();

    seen_states.insert(grid.to_string(), 0);
    states_in_order.push(grid.clone());

    let mut num_spin_cycles = 0;
    let cycle_start;
    loop
    {
        spin_cycle(&mut grid);
        num_spin_cycles += 1;

        match seen_states.insert(grid.to_string(), num_spin_cycles)
        {
            None =>
            {
                // OK - another new state
            },
            Some(prev_val) =>
            {
                cycle_start = prev_val;
                break;
            }
        }
        states_in_order.push(grid.clone());
    }

    // The state after 1,000,000,000 cycles will be
    // the same after being reduced mod the cycle length -
    // with an offset for the cycle start...

    let modulus = cycle_start + ((CYCLES - cycle_start) % (num_spin_cycles - cycle_start));

    calc_load_on_north_beam(&states_in_order[modulus as usize])
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(14)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 136,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 109385,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 64,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 93102,
        })
}
