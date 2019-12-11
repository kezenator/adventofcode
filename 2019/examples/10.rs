use std::collections::HashSet;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
struct Point
{
    x: i64,
    y: i64,
}

impl Point
{
    pub fn new(x: i64, y: i64) -> Self
    {
        Point { x, y }
    }

    pub fn points_between(&self, other: &Point) -> HashSet<Point>
    {
        let sign_x = (other.x - self.x).signum();
        let sign_y = (other.y - self.y).signum();

        let dist_x = (other.x - self.x).abs();
        let dist_y = (other.y - self.y).abs();

        let mut result = HashSet::new();

        // Go through the mid points
        // dx/dist_x = dy/dist_y
        // so e.g. for x
        //
        // dy = (dx * dist_y) / dist_x
        // and only if this is an integer

        for dx in 1..dist_x
        {
            let num = dx * dist_y;
            if num % dist_x == 0
            {
                let dy = num / dist_x;

                result.insert(Point::new(
                    self.x + dx * sign_x,
                    self.y + dy * sign_y));
            }
        }

        for dy in 1..dist_y
        {
            let num = dy * dist_x;
            if num % dist_y == 0
            {
                let dx = num / dist_y;

                result.insert(Point::new(
                    self.x + dx * sign_x,
                    self.y + dy * sign_y));
            }
        }

        // Ensure we don't get either in - due to rouding errors

        result.remove(self);
        result.remove(other);

        //println!("{:?} .between {:?} => {:?}", self, other, result);

        result
    }

    pub fn dist_to(&self, other: &Point) -> i64
    {
        (self.x - other.x).abs()
        + (self.y - other.y).abs()
    }

    pub fn angle_to(&self, other: &Point) -> i64
    {
        // Up is zero, then clockwise

        let dx = (other.x - self.x) as f64;
        let dy = (other.y - self.y) as f64;

        let ang = (-dy).atan2(dx);

        let ang = if ang < 0.0
        {
            std::f64::consts::FRAC_PI_2 + -ang
        }
        else if ang <= std::f64::consts::FRAC_PI_2
        {
            std::f64::consts::FRAC_PI_2 - ang
        }
        else
        {
            std::f64::consts::PI
                + std::f64::consts::PI
                + std::f64::consts::FRAC_PI_2
                - ang
        };

        //println!("{:?} ang {:?} => {}", self, other, ang);
        assert!(ang >= 0.0 && ang < (2.0 * std::f64::consts::PI));

        (ang * 1000000.0) as i64
    }
}

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

    let mut num_seen = Vec::new();

    for loc in points.iter()
    {
        let mut seen_from_here: i64 = 0;

        for other in points.iter()
        {
            if loc != other
            {
                let mut found_mid = false;

                for mid in loc.points_between(other)
                {
                    if points.contains(&mid)
                    {
                        //println!("{:?} => {:?} blocked by {:?}", loc, other, mid);
                        found_mid = true;
                    }
                }

                if !found_mid
                {
                    seen_from_here += 1;
                }
            }
        }

        //println!("{:?} sees {}", loc, seen_from_here);

        num_seen.push((seen_from_here, *loc));
    }

    num_seen.sort();
    *num_seen.last().unwrap()
}

fn part_1(input: &str) -> i64
{
    best_monitoring(input).0
}

fn next_destroyed(loc: &Point, points: &HashSet<Point>, angle: i64) -> Option<Point>
{
    let mut found = points.iter()
        .map(|p| (loc.angle_to(p), loc.dist_to(p), p.clone()))
        .filter(|(a, _d, _p)| *a > angle)
        .collect::<Vec<_>>();

    found.sort();

    if found.is_empty()
    {
        return None;
    }
    //println!("{:?}", found.first().unwrap());
    Some(found.first().unwrap().2)
}

fn part_2(input: &str, nth: i64) -> i64
{
    let mut points = read_points(input);
    let monitoring = best_monitoring(input).1;
    //let monitoring = Point::new(8, 3);

    //println!("Mon: {:?}", monitoring);

    points.remove(&monitoring);

    let mut angle = -1;
    let mut count = 0;

    loop
    {
        match next_destroyed(&monitoring, &points, angle)
        {
            Some(next) =>
            {
                count += 1;
                points.remove(&next);
                angle = monitoring.angle_to(&next);
                //println!("{} => {:?}", count, next);
                if count == nth
                {
                    return next.x * 100 + next.y;
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
    let points = vec![
        Point::new(0, -1),
        Point::new(1, -2),
        Point::new(1, -1),
        Point::new(2, -1),
        Point::new(1, 0),
        Point::new(2, 1),
        Point::new(1, 1),
        Point::new(1, 2),
        Point::new(0, 1),
        Point::new(-1, 2),
        Point::new(-1, 1),
        Point::new(-2, 1),
        Point::new(-1, 0),
        Point::new(-2, -1),
        Point::new(-1, -1),
        Point::new(-1, -2),
    ];
    assert_eq!(Point::new(0, 0).angle_to(&points[0]), 0);
    for i in 1..16
    {
        let a = Point::new(0, 0).angle_to(&points[i - 1]);
        let b = Point::new(0, 0).angle_to(&points[i]);
        assert!(b > a);
    }

    const INPUT: &str = include_str!("input_10.txt");

    const EXAMPLE_1_1: &str = ".#..#\n.....\n#####\n....#\n...##\n";
    const EXAMPLE_1_2: &str = "......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####\n";
    const EXAMPLE_1_3: &str = ".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##\n";

    //const EXAMPLE_2_1: &str = ".#....#####...#..\n##...##.#####..##\n##...#...#.#####.\n..#.....X...###..\n..#.#.....#....##\n";

    assert_eq!(part_1(EXAMPLE_1_1), 8);
    assert_eq!(part_1(EXAMPLE_1_2), 33);
    assert_eq!(part_1(EXAMPLE_1_3), 210);

    let answer_1 = part_1(INPUT);
    println!("Answer #1={}", answer_1);
    assert_eq!(answer_1, 214);

    //assert_eq!(part_2(EXAMPLE_2_1, 36), 1303);
    assert_eq!(part_2(EXAMPLE_1_3, 200), 802);

    let answer_2 = part_2(INPUT, 200);
    println!("Answer #2={}", answer_2);
    assert_eq!(answer_2, 502);
}