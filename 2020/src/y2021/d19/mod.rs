use crate::support::*;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

const EXAMPLE: &str = include_str!("example.txt");

struct Scanner
{
    beacons: Vec<Point3>,
    origin: Option<Point3>,
}

fn parse_scanners(input: &str) -> Vec<Scanner>
{
    let groups = input_to_groups(input);
    let mut scanners = Vec::new();

    for g in groups
    {
        let mut beacons = Vec::with_capacity(g.len() - 1);
        let origin = None;

        for i in 1..g.len()
        {
            let (x, y, z) = scan(&g[i])
                .until(",").parse::<i64>()
                .until(",").parse::<i64>()
                .remaining().parse::<i64>();

            beacons.push(Point3::new(x, y, z));
        }

        scanners.push(Scanner{ beacons, origin });
    }

    scanners
}

fn solve_scanners(input: &str) -> Vec<Scanner>
{
    let mut scanners = parse_scanners(input);
    let mut solved_beacons = HashSet::new();

    // Mark the first scanner as solved in
    // its current position, and add all of it's
    // beacons into the solved set

    scanners[0].origin = Some(Point3::new(0, 0, 0));

    for b in scanners[0].beacons.iter()
    {
        solved_beacons.insert(*b);
    }

    // Loop until we have solved all scanners

    'main_loop:
    while scanners.iter().filter(|s| s.origin.is_none()).next().is_some()
    {
        for i in 0..scanners.len()
        {
            if scanners[i].origin.is_none()
            {
                for rot in Transform3::rotations_and_reflections_24()
                {
                    // Rotate this scanner's beacons

                    let test_beacons = scanners[i].beacons.iter()
                        .map(|b| rot.transform_point(*b))
                        .collect::<Vec<Point3>>();

                    // Test each beacon against the known
                    // beacons and get a list of offsets

                    let mut x_offset_counts = HashMap::<i64, usize>::new();
                    let mut y_offset_counts = HashMap::<i64, usize>::new();
                    let mut z_offset_counts = HashMap::<i64, usize>::new();

                    for b in test_beacons.iter()
                    {
                        for s in solved_beacons.iter()
                        {
                            let diff = *s - *b;

                            *x_offset_counts.entry(diff.x).or_insert(0) += 1;
                            *y_offset_counts.entry(diff.y).or_insert(0) += 1;
                            *z_offset_counts.entry(diff.z).or_insert(0) += 1;
                        }
                    }

                    // Collect offsets and sort by max count

                    let mut x_offset_counts = x_offset_counts.drain().collect::<Vec<_>>();
                    let mut y_offset_counts = y_offset_counts.drain().collect::<Vec<_>>();
                    let mut z_offset_counts = z_offset_counts.drain().collect::<Vec<_>>();

                    x_offset_counts.sort_by(|a, b| b.1.cmp(&a.1));
                    y_offset_counts.sort_by(|a, b| b.1.cmp(&a.1));
                    z_offset_counts.sort_by(|a, b| b.1.cmp(&a.1));

                    if (x_offset_counts[0].1 >= 12) && (y_offset_counts[0].1 >= 12) && (z_offset_counts[0].1 >= 12)
                    {
                        // We've found an offset with this rotation that
                        // lines up at least 12 points

                        let offset = Point3::new(x_offset_counts[0].0, y_offset_counts[0].0, z_offset_counts[0].0);

                        // Transform this beacon's point to their
                        // final "real" location

                        let transform = rot.append(&Transform3::translation(offset));

                        let new_beacons = scanners[i].beacons.iter()
                            .map(|b| transform.transform_point(*b))
                            .collect::<Vec<_>>();

                        // Check that at least 12 of these overlap with
                        // existing beacons

                        let num_overlap_existing = new_beacons.iter()
                            .filter(|b| solved_beacons.contains(b))
                            .count();

                        if num_overlap_existing >= 12
                        {
                            // OK - this one is solved.
                            // Add in the total set of new solved beacons.

                            for b in new_beacons.iter()
                            {
                                solved_beacons.insert(*b);
                            }

                            // Finally, mark this beacon as solved

                            scanners[i].beacons = new_beacons;
                            scanners[i].origin = Some(offset);

                            // Don't try new rotations for this beacon -
                            // try another beacon

                            continue 'main_loop;
                        }
                    }
                }
            }
        }

        // We couldn't line up any unsolved scanner with
        // the existing data - this is not good!
        unreachable!();
    }

    scanners
}

fn part_1(input: &str) -> usize
{
    let scanners = solve_scanners(input);

    scanners.iter()
        .map(|s| s.beacons.iter())
        .flatten()
        .copied()
        .unique()
        .count()
}

fn part_2(input: &str) -> i64
{
    let scanners = solve_scanners(input);

    scanners.iter()
        .combinations(2)
        .map(|pair| (pair[0].origin.unwrap() - pair[1].origin.unwrap()).manhatten_size())
        .max()
        .unwrap()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(19)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 79,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 454,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 3621,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 10813,
        })
}
