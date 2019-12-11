use aoc2019::*;
use std::collections::HashSet;

fn read_points(input: &str) -> HashSet<Point>
{
    let mut x = 0;
    let mut y = 0;
    let mut result = HashSet::new();

    for ch in input.chars()
    {
        match ch
        {
            '#' =>
            {
                result.insert(Point::new(x, y));
                x += 1;
            },
            '.' =>
            {
                x += 1;
            },
            '\n' =>
            {
                y += 1;
                x = 0;
            },
            _ => {},
        }
    }

    result
}

fn best_monitoring(input: &str) -> (i64, Point)
{
    let points = read_points(input);

    let mut num_seen_at = points.iter()
        .map(|&location|
            {
                let mut seen_from_here = 0;

                for other in points.iter().filter(|&other| *other != location)
                {
                    let num_stations_between = Line::new(location, *other).points_along().iter()
                        .filter(|mid| points.contains(mid))
                        .count();

                    if num_stations_between == 0
                    {
                        seen_from_here += 1;
                    }
                }

                (seen_from_here, location)
            })
        .collect::<Vec<_>>();

    num_seen_at.sort();
    *num_seen_at.last().unwrap()
}

fn part_1(input: &str) -> i64
{
    best_monitoring(input).0
}

fn next_destroyed(loc: &Point, points: &HashSet<Point>, angle: i64) -> Option<(i64, i64, Point)>
{
    let mut found = points.iter()
        .map(|p| ((Line::new(*loc, *p).degrees_clockwise_from_up() * 1000.0) as i64,
                  loc.manhatten_dist_to(p),
                  p.clone()))
        .filter(|(a, _d, _p)| *a > angle)
        .collect::<Vec<_>>();

    found.sort();

    if found.is_empty()
    {
        return None;
    }
    //println!("{:?}", found.first().unwrap());
    Some(*found.first().unwrap())
}

fn part_2(input: &str) -> i64
{
    let mut points = read_points(input);
    let monitoring = best_monitoring(input).1;

    points.remove(&monitoring);

    let mut angle = -1;
    let mut count = 0;

    loop
    {
        match next_destroyed(&monitoring, &points, angle)
        {
            Some((next_angle, _dist, next_point)) =>
            {
                count += 1;
                points.remove(&next_point);
                angle = next_angle;
                //println!("{} => {:?}", count, next_point);
                if count == 200
                {
                    return next_point.x * 100 + next_point.y;
                }
            },
            None =>
            {
                // None left on this rotation - try again
                // on the next rotation
                //println!("Resetting angle");
                angle = -1;
            }
        }
    }
}

fn main()
{
    const INPUT: &str = include_str!("input_10.txt");

    const EXAMPLE_1_1: &str = ".#..#\n.....\n#####\n....#\n...##\n";
    const EXAMPLE_1_2: &str = "......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####\n";
    const EXAMPLE_1_3: &str = ".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##\n";

    assert_eq!(part_1(EXAMPLE_1_1), 8);
    assert_eq!(part_1(EXAMPLE_1_2), 33);
    assert_eq!(part_1(EXAMPLE_1_3), 210);

    let answer_1 = part_1(INPUT);
    println!("Answer #1={}", answer_1);
    assert_eq!(answer_1, 214);

    assert_eq!(part_2(EXAMPLE_1_3), 802);

    let answer_2 = part_2(INPUT);
    println!("Answer #2={}", answer_2);
    assert_eq!(answer_2, 502);
}