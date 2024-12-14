
use std::str::FromStr;
use itertools::*;
use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");

const EXAMPLE_SIZE: Point = Point { x: 11, y: 7};
const INPUT_SIZE: Point = Point { x: 101, y: 103 };

#[derive(Debug)]
struct Robot
{
    pos: Point,
    dir: Point,
}

impl FromStr for Robot
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let (px, py, dx, dy) = scan(s)
            .skip_str("p=")
            .until(",").parse()
            .until(" v=").parse()
            .until(",").parse()
            .remaining().parse();
        
        Ok(Robot{ pos: Point::new(px, py), dir: Point::new(dx, dy) })
    }
}

struct Room
{
    robots: Vec<Robot>,
    size: Point,
}

impl Room
{
    fn new(input: &str, size: Point) -> Self
    {
        Room
        {
            robots: input_to_lines_parsed(input),
            size,
        }
    }

    fn move_robots(&mut self, seconds: i64)
    {
        for r in self.robots.iter_mut()
        {
            r.pos.x = (r.pos.x + seconds * r.dir.x) % self.size.x;
            r.pos.y = (r.pos.y + seconds * r.dir.y) % self.size.y;

            if r.pos.x < 0 { r.pos.x += self.size.x; }
            if r.pos.y < 0 { r.pos.y += self.size.y; }
        }
    }

    fn safety_factor(&self) -> usize
    {
        let mid_x = self.size.x / 2;
        let mid_y = self.size.y / 2;

        self.count_robots_in(Point::new(0, 0), Point::new(mid_x - 1, mid_y - 1))
            * self.count_robots_in(Point::new(0, mid_y + 1), Point::new(mid_x - 1, self.size.y))
            * self.count_robots_in(Point::new(mid_x + 1, 0), Point::new(self.size.x, mid_y - 1))
            * self.count_robots_in(Point::new(mid_x + 1, mid_y + 1), Point::new(self.size.x, self.size.y))
    }

    fn count_robots_in(&self, min: Point, max: Point) -> usize
    {
        let count = self.robots.iter()
            .filter(|&r|
            {
                r.pos.x >= min.x && r.pos.x <= max.x
                && r.pos.y >= min.y && r.pos.y <= max.y
            })
            .count();
        count
    }

    fn to_char_grid(&self) -> CharGrid
    {
        CharGrid::new_from_points(self.robots.iter()
            .map(|r| r.pos.clone())
            .collect_vec())
    }
}

fn part_1(input: &str, size: Point) -> usize
{
    let mut room = Room::new(input, size);
    room.move_robots(100);
    room.safety_factor()
}

fn part_2(input: &str, size: Point) -> usize
{
    let mut room = Room::new(input, size);

    for count in 0..10000
    {
        room.move_robots(1);

        // Keep going until all robots
        // are in a unique location

        if room.robots.len() != room.robots.iter().map(|r| r.pos.clone()).unique().count()
        {
            continue;
        }

        let grid = room.to_char_grid();
        println!("{}", grid.to_string());

        return count + 1;
    }
    unreachable!();
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(14)
        .example(|| Answer {
            calculated: part_1(EXAMPLE, EXAMPLE_SIZE.clone()),
            expected: 12,
        })
        .part_1(|input| Answer {
            calculated: part_1(input, INPUT_SIZE.clone()),
            expected: 232253028,
        })
        .part_2(|input| Answer {
            calculated: part_2(input, INPUT_SIZE.clone()),
            expected: 8179,
        })
}
