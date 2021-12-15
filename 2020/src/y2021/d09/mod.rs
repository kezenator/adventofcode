use crate::support::*;
use pathfinding::prelude::*;

const EXAMPLE: &str = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";
const INPUT: &str = include_str!("input.txt");

const OUTSIDE: char = '9';

fn load_image(input: &str) -> CharGrid
{
    CharGrid::new_from_input(input, OUTSIDE)
}

fn find_basins(image: &CharGrid) -> Vec<Point>
{
    // Find an collect all points where...

    image.all_points().drain(..)
        .filter(|&p|
            {
                let p_height = image.get_char(&p);

                // There is no neighbour n (from this point p and a direction d)
                // who is equal or lower to us (i.e. count is 0)

                p.neighbours_4()
                    .filter(|n| image.get_char(n) <= p_height)
                    .count() == 0
            })
        .collect::<Vec<_>>()
}

fn risk_level(c: char) -> usize
{
    (c as usize) - ('0' as usize) + 1
}

fn basin_size(image: &CharGrid, basin: Point) -> usize
{
    // The basin size is the number size of the reach
    // of the directed graph where we
    // step up/level, but not OUTSIDE

    bfs_reach(
        basin,
        |&p|
        {
            // Where can we step up from p?
            // For all directions, find the neighbour,
            // and filter for only ones that are a step up/level but not OUTSIDE

            let p_height = image.get_char(&p);

            p.neighbours_4()
                .filter(move |&n|
                    {
                        let n_height = image.get_char(&n);

                        (n_height != OUTSIDE) && (n_height >= p_height)
                    })
        }
    ).count()
}

fn part_1(input: &str) -> usize
{
    let image = load_image(input);

    find_basins(&image).iter()
        .map(|p| risk_level(image.get_char(p)))
        .sum()
}

fn part_2(input: &str) -> usize
{
    let image = load_image(input);

    // Find the size of all basins

    let mut sizes = find_basins(&image).iter()
        .map(|&p| basin_size(&image, p))
        .collect::<Vec<usize>>();

    // Return the product of the three largest

    sizes.sort();
    
    sizes.iter()
        .rev()
        .take(3)
        .product()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(9)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 15,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 526,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 1134,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 1123524,
        })
}
