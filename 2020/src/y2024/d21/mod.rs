
use itertools::Itertools;
use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");

fn button_to_location(button: char, level: usize) -> Point
{
    if level == 0
    {
        match button
        {
            '0' => Point::new(1, 3),
            '1' => Point::new(0, 2),
            '2' => Point::new(1, 2),
            '3' => Point::new(2, 2),
            '4' => Point::new(0, 1),
            '5' => Point::new(1, 1),
            '6' => Point::new(2, 1),
            '7' => Point::new(0, 0),
            '8' => Point::new(1, 0),
            '9' => Point::new(2, 0),
            'A' => Point::new(2, 3),
            _ => unreachable!(),
        }
    }
    else
    {
        match button
        {
            '^' => Point::new(1, 0),
            '<' => Point::new(0, 1),
            'v' => Point::new(1, 1),
            '>' => Point::new(2, 1),
            'A' => Point::new(2, 0),
            _ => unreachable!(),
        }
    }
}

fn location_to_neighbours(location: &Point, level: usize) -> impl Iterator<Item = Point>
{
    let (max_y, invalid_p) = if level == 0
    {
        (3, Point::new(0, 3))
    }
    else
    {
        (1, Point::new(0, 0))
    };

    location.neighbours_4()
            .filter(move |n| n.x <= 2 && n.y <= max_y && (*n != invalid_p))
}

fn dir_to_button(dir: Point) -> char
{
    match dir
    {
        Point{x: -1, y: 0} => '<',
        Point{x: 1, y: 0} => '>',
        Point{x: 0, y: -1} => '^',
        Point{x: 0, y: 1} => 'v',
        _ => unreachable!(),
    }
}

fn astar_dir_buttons(start: &Point, end: &Point, level: usize) -> Vec<String>
{
    let astar_result = pathfinding::directed::astar::astar_bag_collect(
        start,
        |cur| location_to_neighbours(cur, level).map(|n| (n, 1)),
        |_| 0,
        |cur| *cur == *end);

    astar_result.unwrap().0.into_iter()
        .map(|path|
            {
                path.into_iter().tuple_windows()
                    .map(|(prev, next)| dir_to_button(next - prev))
                    .collect()
            })
        .collect_vec()
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct CodeLevel
{
    code: String,
    cur_level: usize,
    max_levels: usize,
}

fn len_of_min_presses(code: &str, max_levels: usize) -> usize
{
    let memorized = Memorized::new(
        & move |input: &CodeLevel, memorized| -> usize
        {
            if input.cur_level == input.max_levels
            {
                return input.code.len();
            }

            let mut location = 'A';
            let mut shortest_len = 0;

            for button in input.code.chars()
            {
                let possible_sub_sequences = astar_dir_buttons(
                    &button_to_location(location, input.cur_level),
                    &button_to_location(button, input.cur_level),
                    input.cur_level);

                let shortest_sub_len = possible_sub_sequences.into_iter()
                    .map(|sub_code|
                    {
                        memorized.get(&CodeLevel
                        {
                            code: format!("{}A", sub_code),
                            cur_level: input.cur_level + 1,
                            max_levels: input.max_levels,
                        })
                    })
                    .min().unwrap();

                location = button;
                shortest_len += shortest_sub_len;
            }

            shortest_len
        });
    
    memorized.get(&CodeLevel{ code: code.to_string(), cur_level: 0, max_levels })
}

fn complexity_code(code: &str, robot_chain_size: usize) -> usize
{
    let min_presses = len_of_min_presses(code, robot_chain_size + 1);
    let numeric_part = scan(code).take_digits().parse::<usize>().remaining().ignore().0;

    min_presses * numeric_part
}

fn complexity_code_sum(input: &str, robot_chain_size: usize) -> usize
{
    input_to_lines(input).into_iter()
        .map(|code| complexity_code(&code, robot_chain_size))
        .sum()
}

fn part_1(input: &str) -> usize
{
    complexity_code_sum(input, 2)
}

fn part_2(input: &str) -> usize
{
    complexity_code_sum(input, 25)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(21)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 126384,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 154208,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 188000493837892usize,
        })
}
