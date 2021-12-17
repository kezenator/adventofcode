use crate::support::*;

const EXAMPLE: &str = "target area: x=20..30, y=-10..-5";
const INPUT: &str = "target area: x=269..292, y=-68..-44";

fn parse_target_area(input: &str) -> Rect
{
    let (x1, x2, y1, y2) = scan(input)
        .skip_str("target area: x=")
        .until("..").parse::<i64>()
        .until(", y=").parse::<i64>()
        .until("..").parse::<i64>()
        .remaining().parse::<i64>();

    Rect::new_from_coords(x1, y1, x2, y2)
}

fn possible_initial_velocities(target_area: Rect) -> impl Iterator<Item = Point>
{
    let x_range = 1..(target_area.get_max_x() + 1);
    let y_range = target_area.get_min_y()..(-target_area.get_min_y() + 1);

    y_range
        .map(move |y| x_range.clone().map(move |x| Point::new(x, y)))
        .flatten()
}

fn points_in_tragectory(target_area: Rect, initial_velocity: Point) -> impl Iterator<Item = Point>
{
    std::iter::successors(
        Some((Point::new(0, 0), initial_velocity)),
        |(cur_pos, cur_vel)|
        {
            let next_pos = *cur_pos + *cur_vel;
            let next_vel = Point::new(i64::max(cur_vel.x - 1, 0), cur_vel.y - 1);

            Some((next_pos, next_vel))
        })
    .map(|(pos, _)| pos)
    .take_while(move |pos| pos.x <= target_area.get_max_x() && pos.y >= target_area.get_min_y())
}

fn max_y_in_tragectory(target_area: Rect, initial_velocity: Point) -> i64
{
    points_in_tragectory(target_area, initial_velocity)
        .map(|p| p.y)
        .max().unwrap()
}

fn does_tragectory_hit_target_area(target_area: Rect, initial_velocity: Point) -> bool
{
    points_in_tragectory(target_area, initial_velocity)
        .any(|p| target_area.does_point_intersect(p))
}

fn initial_velocities_that_hit_target_area(target_area: Rect) -> impl Iterator<Item = Point>
{
    possible_initial_velocities(target_area)
        .filter(move |initial_velocity| does_tragectory_hit_target_area(target_area, *initial_velocity))
}

fn part_1(input: &str) -> i64
{
    let target_area = parse_target_area(input);

    initial_velocities_that_hit_target_area(target_area)
        .map(move |initial_velocity| max_y_in_tragectory(target_area, initial_velocity))
        .max().unwrap()
}

fn part_2(input: &str) -> usize
{
    let target_area = parse_target_area(input);

    initial_velocities_that_hit_target_area(target_area)
        .count()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(17)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 45,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 2278,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 112,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 996,
        })
}
