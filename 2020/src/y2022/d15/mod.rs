use crate::support::*;
use itertools::*;

const EXAMPLE: &str = include_str!("example.txt");

struct Datum
{
    sensor: Point,
    nearest_beacon: Point,
}

impl Datum
{
    fn size(&self) -> i64
    {
        (self.sensor - self.nearest_beacon).manhatten_size()
    }
    

    fn x_extent_at_y(&self, y: i64) -> Option<RangeInc<i64>>
    {
        let size = self.size();
        let dist = (y - self.sensor.y).abs();
        if dist <= size
        {
            let offset = size - dist;
            Some(RangeInc::new_range(self.sensor.x - offset, self.sensor.x + offset))
        }
        else
        {
            None
        }
    }
}

fn parse_datum(line: &str) -> Datum
{
    let (sx, sy, bx, by) = scan(line)
        .skip_str("Sensor at x=")
        .until(", y=").parse()
        .until(": closest beacon is at x=").parse()
        .until(", y=").parse()
        .remaining().parse();

    Datum { sensor: Point::new(sx, sy), nearest_beacon: Point::new(bx, by) }
}

fn parse(input: &str) -> Vec<Datum>
{
    input_to_lines(input)
        .iter()
        .map(|l| parse_datum(l))
        .collect()
}

fn x_may_contain_beacon_at_y(data: &Vec<Datum>, y: i64, include_known_beacons: bool) -> RangeSet<i64>
{
    let mut result = RangeSet::all();

    for d in data
    {
        if let Some(r) = d.x_extent_at_y(y)
        {
            result.remove_range(r);
        }
    }

    if include_known_beacons
    {
        for d in data.iter().filter(|d| d.nearest_beacon.y == y)
        {
            result.insert_value(d.nearest_beacon.x);
        }
    }

    result
}

fn part_1(input: &str, y: i64) -> usize
{
    let data = parse(input);

    x_may_contain_beacon_at_y(&data, y, true)
        .inverse()
        .values()
        .count()
}

fn part_2(input: &str, size: i64) -> i64
{
    let data = parse(input);

    for y in 0..(size + 1)
    {
        if let Some(x) =
            x_may_contain_beacon_at_y(&data, y, false)
            .insersection_with_range(RangeInc::new_range(0, size+1))
            .values()
            .next()
        {
            return x * 4000000 + y;
        }
    }
    unreachable!();
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(15)
        .example(|| Answer {
            calculated: part_1(EXAMPLE, 10),
            expected: 26,
        })
        .part_1(|input| Answer {
            calculated: part_1(input, 2000000),
            expected: 5508234,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE, 20),
            expected: 56000011,
        })
        .part_2(|input| Answer {
            calculated: part_2(input, 4000000),
            expected: 10457634860779i64,
        })
}
