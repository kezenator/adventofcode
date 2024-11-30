use crate::support::*;
use itertools::*;

const EXAMPLE: &str = include_str!("example.txt");

fn parse_point(input: &str) -> Point
{
    let parts = input.split(",").collect_vec();
    Point::new(parts[0].parse().unwrap(), parts[1].parse().unwrap())
}

fn parse_path(input: &str) -> Vec<Line>
{
    input.split(" -> ")
        .collect_vec()
        .windows(2)
        .map(|w| Line::new(parse_point(&w[0]), parse_point(&w[1])))
        .collect_vec()
}

struct SandPile
{
    chars: CharGrid,
    start: Point,
}

impl SandPile
{
    fn parse(input: &str, part2: bool) -> Self
    {
        let lines = input_to_lines(input)
            .iter()
            .map(|l| parse_path(l))
            .flatten()
            .collect_vec();

        let points = lines.iter()
            .map(|l| vec![l.start, l.end])
            .flatten()
            .collect_vec();

        let top = points.iter().map(|p| p.y).min().unwrap().min(0);
        let bottom = points.iter().map(|p| p.y).max().unwrap().max(0) + 2;
        let height = bottom - top + 1;

        let left = points.iter().map(|p| p.x).min().unwrap().min(499 - height);
        let right = points.iter().map(|p| p.x).max().unwrap().max(501 + height);
        let width = right - left + 1;

        let offset = Point::new(-left, -top);

        let start = Point::new(500, 0) + offset;
        
        let mut chars = CharGrid::new_from_fill(width as usize, height as usize, '.');

        for l in lines
        {
            for p in l.points_exactly_on_line_inclusive()
            {
                chars.put_char(&(p + offset), '#');
            }
        }

        if part2
        {
            for x in 0..width
            {
                chars.put_char(&Point::new(x, height - 1), '#');
            }
        }

        SandPile { chars, start }
    }

    fn place_next_grain(&mut self) -> bool
    {
        let max_y = self.chars.get_height();
        let mut pos = self.start;

        if self.chars.get_char(&pos) != '.'
        {
            // Already full
            return false;
        }

        while pos.y <= max_y
        {
            if self.chars.get_char(&Point::new(pos.x, pos.y + 1)) == '.'
            {
                // Down
                pos += Point::new(0, 1);
            }
            else if self.chars.get_char(&Point::new(pos.x - 1, pos.y + 1)) == '.'
            {
                // Down-Left
                pos += Point::new(-1, 1);
            }
            else if self.chars.get_char(&Point::new(pos.x + 1, pos.y + 1)) == '.'
            {
                // Down-Right
                pos += Point::new(1, 1);
            }
            else
            {
                // At rest
                self.chars.put_char(&pos, 'o');
                return true;
            }
        }
        // Fallen into the abyss
        false
    }
}

fn num_grains_placed(input: &str, part2: bool) -> usize
{
    let mut pile = SandPile::parse(input, part2);
    //println!("{}", pile.chars.to_string());

    let mut count = 0;
    while pile.place_next_grain()
    {
        count += 1;
    }

    //println!("{}", pile.chars.to_string());
    count
}

fn part_1(input: &str) -> usize
{
    num_grains_placed(input, false)
}

fn part_2(input: &str) -> usize
{
    num_grains_placed(input, true)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(14)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 24,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 1406,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 93,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 20870,
        })
}
