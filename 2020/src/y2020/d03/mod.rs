use crate::support::*;

const EXAMPLE: &str = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#\n";

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

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(3)
        .example(|| Answer { calculated: part_1(EXAMPLE), expected: 7, })
        .part_1(|input| Answer { calculated: part_1(input), expected: 265, })
        .example(|| Answer { calculated: part_2(EXAMPLE), expected: 336, })
        .part_2(|input| Answer { calculated: part_2(input), expected: 3154761400u64, })
}
