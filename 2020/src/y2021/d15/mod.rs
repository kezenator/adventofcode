use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

fn find_shortest_cost(input: &str, num_repeats: i64) -> i64
{
    let grid = CharGrid::new_from_input(input, ' ');
    let width = grid.get_width();
    let height = grid.get_height();

    let start = Point::new(0, 0);
    let end = Point::new((num_repeats * width) - 1, (num_repeats * height) - 1);

    let (_, path_cost) = pathfinding::directed::astar::astar(
        &start,
        |node: &Point|
        {
            node.neighbours_4()
                .filter_map(|neighbour|
                    {
                        if (neighbour.x < 0) || (neighbour.y < 0)
                        {
                            // Off the start of the grid - don't consider
                            return None;
                        }

                        let rep_x = neighbour.x / width;
                        let rep_y = neighbour.y / height;

                        if (rep_x >= num_repeats) || (rep_y >= num_repeats)
                        {
                            // Off the end of the grid - don't consider
                            return None;
                        }

                        let image_x = neighbour.x % width;
                        let image_y = neighbour.y % height;

                        let offset = rep_x + rep_y;
                        let orig = ((grid.get_char(&Point::new(image_x, image_y)) as u8) - ('0' as u8)) as i64;

                        // Sum mod 1..9 (i.e. mod 9 with an offset of 1)
                        let modified_cost = ((offset + orig - 1) % 9) + 1;

                        Some((neighbour, modified_cost))
                    })
                .collect::<Vec<_>>()
        },
        |node: &Point|
        {
            (*node - end).manhatten_size()
        },
        |node: &Point|
        {
            *node == end
        }).unwrap();

    path_cost
}

fn part_1(input: &str) -> i64
{
    find_shortest_cost(input, 1)
}

fn part_2(input: &str) -> i64
{
    find_shortest_cost(input, 5)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(15)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 40,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 462,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 315,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 2846,
        })
}
