use itertools::*;
use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");

struct Step
{
    dir: Point,
    count: usize,
}

#[derive(Clone, Copy)]
enum ParseFormat
{
    Part1,
    Part2,
}

fn parse_plan_step(step_str: &str, format: ParseFormat) -> Step
{
    match format
    {
        ParseFormat::Part1 =>
        {
            let (dir_ch, count) = scan(step_str)
                .take(1).parse()
                .skip_ws()
                .take_digits().parse()
                .remaining().ignore();

            let dir = match dir_ch
            {
                'L' => Point::new(-1, 0),
                'R' => Point::new(1, 0),
                'U' => Point::new(0, -1),
                'D' => Point::new(0, 1),
                _ => unreachable!(),
            };

            Step{ dir, count, }
        },
        ParseFormat::Part2 =>
        {
            let (count_str, dir_ch) = scan(step_str)
                .skip(1)
                .skip_ws()
                .until_whitespace().ignore()
                .skip_ws()
                .skip_str("(#")
                .take(5).parse::<String>()
                .take(1).parse::<char>()
                .remaining().ignore();

            let count = usize::from_str_radix(&count_str, 16).unwrap();
            let dir = match dir_ch
            {
                '0' => Point::new(1, 0),
                '1' => Point::new(0, 1),
                '2' => Point::new(-1, 0),
                '3' => Point::new(0, -1),
                _ => unreachable!(),
            };

            Step{ dir, count }
        },
    }
}

fn steps_to_lines(steps: Vec<Step>) -> Vec<Line>
{
    let mut result = Vec::new();
    let mut cur_pos = Point::new(0, 0);

    for s in steps
    {
        let end_pos = cur_pos + (s.count as i64) * s.dir;
        result.push(Line::new(cur_pos, end_pos));
        cur_pos = end_pos;
    }

    result
}

fn calc_lagoon_volume(input: &str, format: ParseFormat) -> i64
{
    // Read in all the steps

    let steps = input_to_lines(input).into_iter()
        .map(|l| parse_plan_step(&l, format))
        .collect_vec();

    // Create a set of lines

    let lines = steps_to_lines(steps);

    let h_lines = lines.iter()
        .filter(|l| l.start.y == l.end.y)
        .cloned()
        .collect_vec();

    let v_lines = lines.iter()
        .filter(|l| l.start.x == l.end.x)
        .cloned()
        .collect_vec();

    // Find the y points of all horizontal lines

    let h_ys = h_lines.iter()
        .map(|l| l.start.y)
        .unique()
        .sorted()
        .collect_vec();

    // Now - step down each section between the
    // horizontal lines, and add in the included rectangular area

    let mut result = 0;

    for (y1, y2) in h_ys.into_iter().tuple_windows()
    {
        assert!(y1 < y2);

        // Find the vertical lines that are present at this y offset

        let active_v_lines = v_lines.iter()
            .filter(|&l|
                {
                    let ly_min = l.start.y.min(l.end.y);
                    let ly_max = l.start.y.max(l.end.y);
                    ly_min < y2 && ly_max > y1
                })
            .cloned()
            .collect_vec();

        // Find the sorted, unique x offsets of these lines.
        // Vertical lines shouldn't connect - so unique x offsets
        // should have the same size as the active V lines.
        // It should also be even

        let sorted_v_line_xs = active_v_lines.iter()
            .map(|l| l.start.x)
            .unique()
            .sorted()
            .collect_vec();

        assert!(sorted_v_line_xs.len() == active_v_lines.len());
        assert!(sorted_v_line_xs.len() % 2 == 0);

        // Break these into pairs - and then range between each pair (inclusive)
        // is definately included for this area from y1..y2

        let mut xs_between_verticals = RangeSet::new();

        for pair in sorted_v_line_xs.iter().chunks(2).into_iter().map(|c| c.cloned().collect_vec())
        {
            xs_between_verticals.insert_range(RangeInc::new_range(pair[0], pair[1]));
        }

        let mut xs_between_verticals_count = xs_between_verticals.count();

        // If it's the first row, we need to include an extra
        // count for the first horizontal row

        if result == 0
        {
            result += xs_between_verticals_count;
        }
        
        // Then add all the rows y1+1 .. y2-1

        result += xs_between_verticals_count * (y2 - y1 - 1);

        // Finally - we need to add on all of these points - PLUS
        // any extra points on horizontal lines at y2

        for hl_at_y2 in h_lines.iter().filter(|l| l.start.y == y2)
        {
            xs_between_verticals.insert_range(RangeInc::new_range(
                hl_at_y2.start.x.min(hl_at_y2.end.x),
                hl_at_y2.start.x.max(hl_at_y2.end.x)));
        }

        xs_between_verticals_count = xs_between_verticals.count();

        result += xs_between_verticals_count;
    }

    result
}

fn part_1(input: &str) -> i64
{
    calc_lagoon_volume(input, ParseFormat::Part1)
}

fn part_2(input: &str) -> i64
{
    calc_lagoon_volume(input, ParseFormat::Part2)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(18)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 62,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 34329,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 952408144115i64,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 42617947302920i64,
        })
}
