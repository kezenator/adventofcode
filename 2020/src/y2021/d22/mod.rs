use crate::support::*;

const EXAMPLE_1: &str = include_str!("example_1.txt");
const EXAMPLE_2: &str = include_str!("example_2.txt");
const EXAMPLE_3: &str = include_str!("example_3.txt");
const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> Vec<(bool, Cuboid)>
{
    let mut result = Vec::new();

    for line in input_to_lines(input)
    {
        let (on_off, x1, x2, y1, y2, z1, z2) = scan(&line)
            .until(" x=").parse::<String>()
            .until("..").parse::<i64>()
            .until(",y=").parse::<i64>()
            .until("..").parse::<i64>()
            .until(",z=").parse::<i64>()
            .until("..").parse::<i64>()
            .remaining().parse::<i64>();

        result.push((
            on_off == "on",
            Cuboid::new(Point3::new(x1, y1, z1), Point3::new(x2, y2, z2))));
    }

    result
}

fn points_after_operations(ops: Vec<(bool, Cuboid)>) -> i64
{
    let mut on_regions: Vec<Cuboid> = Vec::new();
    let mut num_remaining = ops.len();

    for (on, region) in ops
    {
        let mut new_regions = Vec::new();

        // Always subtract this new region from
        // any existing regions
        // a) When turning on lights - so we don't count
        //    a single location more than once.
        // b) When turning off lights - so they are
        //    not counted as on any more.

        for existing in on_regions
        {
            if let Some(diff) = existing.difference(region)
            {
                for d in diff
                {
                    new_regions.push(d);
                }
            }
        }

        // If turning on - add this region

        if on
        {
            new_regions.push(region);
        }

        on_regions = new_regions;
    }

    on_regions.iter()
        .map(|c| c.volume())
        .sum()
}

fn part_1(input: &str) -> i64
{
    let ops = parse(input);

    // Filter our just the initialization area

    let init_volume = Cuboid::new(Point3::new(-50, -50, -50), Point3::new(50, 50, 50));

    let ops = ops.iter().copied()
        .filter(|(_, cuboid)| { init_volume.completely_contains(*cuboid) })
        .collect::<Vec<_>>();

    points_after_operations(ops)
}

fn part_2(input: &str) -> i64
{
    // Use all operations without filtering
    points_after_operations(parse(input))
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(22)
        .example(|| Answer {
            calculated: part_1(EXAMPLE_1),
            expected: 39,
        })
        .example(|| Answer {
            calculated: part_1(EXAMPLE_2),
            expected: 590784,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 524792,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE_3),
            expected: 2758514936282235i64,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 1213461324555691i64,
        })
}
