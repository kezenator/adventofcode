use crate::support::*;

const EXAMPLE: &str = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#\n";
const INPUT: &str = include_str!("input.txt");

pub fn count_trees_down_slope(lines: &Vec<String>, right: usize, down: usize) -> u64
{
    let mut result = 0;
    let mut line = 0;
    let mut pos = 0;

    while line < lines.len()
    {
        let width = lines[line].len();
        pos = pos % width;

        if lines[line].chars().nth(pos).unwrap() == '#'
        {
            result += 1;
        }

        line += down;
        pos += right;
    }

    result
}

pub fn part_1(input: &str) -> u64
{
    count_trees_down_slope(&input_to_lines(input), 3, 1)
}

pub fn part_2(input: &str) -> u64
{
    count_trees_down_slope(&input_to_lines(input), 1, 1)
        * count_trees_down_slope(&input_to_lines(input), 3, 1)
        * count_trees_down_slope(&input_to_lines(input), 5, 1)
        * count_trees_down_slope(&input_to_lines(input), 7, 1)
        * count_trees_down_slope(&input_to_lines(input), 1, 2)
}

pub fn puzzles(puzzles: &mut PuzzleSet)
{
    puzzles.register("y2020.d03.e1", || Answer {
        calculated: part_1(EXAMPLE),
        expected: 7,
    });

    puzzles.register("y2020.d03.e2", || Answer {
        calculated: part_2(EXAMPLE),
        expected: 336,
    });

    puzzles.register("y2020.d03.p1", || Answer {
        calculated: part_1(INPUT),
        expected: 265,
    });

    puzzles.register("y2020.d03.p2", || Answer {
        calculated: part_2(INPUT),
        expected: 3154761400u64,
    });
}
