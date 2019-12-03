use std::cmp::{min, max};

const INPUT: &str = include_str!("input_3.txt");

#[derive(Debug, Copy, Clone)]
struct Line
{
    sx: i64,
    sy: i64,
    ex: i64,
    ey: i64,
    vert: bool,
}

impl Line
{
    fn new(sx: i64, sy: i64, ex: i64, ey: i64) -> Self
    {
        let vert = sx == ex;
        Line { sx, sy, ex, ey, vert}
    }

    fn points(&self) -> Vec<(i64, i64)>
    {
        let mut result = Vec::new();

        let mut x = self.sx;

        let dx = if self.sx <= self.ex { 1 } else { -1 };
        let dy = if self.sy <= self.ey { 1 } else { -1 };

        loop
        {
            let mut y = self.sy;
            
            loop
            {
                result.push((x, y));

                if y == self.ey
                {
                    break;
                }
                y += dy;
            }

            if x == self.ex
            {
                return result;
            }
            x += dx;
        }
    }

    fn intersects(&self, other: &Line) -> Option<(i64, i64, i64)>
    {
        if self.vert == other.vert
        {
            return None;
        }

        let sminx = min(self.sx, self.ex);
        let smaxx = max(self.sx, self.ex);
        let sminy = min(self.sy, self.ey);
        let smaxy = max(self.sy, self.ey);

        let ominx = min(other.sx, other.ex);
        let omaxx = max(other.sx, other.ex);
        let ominy = min(other.sy, other.ey);
        let omaxy = max(other.sy, other.ey);

        if (smaxx < ominx)
            || (smaxy < ominy)
            || (sminx > omaxx)
            || (sminy > omaxy)
        {
            return None;
        }

        let ps = self.points();
        let po = other.points();

        let mut dist_self = 0;

        for a in ps.iter()
        {
            let mut dist_other = 0;

            for b in po.iter()
            {
                if a == b
                {
                    let (x, y) = a;
                    let dist_origin = x.checked_abs().unwrap()
                        + y.checked_abs().unwrap();

                    return Some((dist_origin, dist_self, dist_other));
                }

                dist_other += 1;
            }

            dist_self += 1;
        }
        None
    }

    fn len(&self) -> i64
    {
        (self.sx - self.ex).checked_abs().unwrap()
            + (self.sy - self.ey).checked_abs().unwrap()
    }
}

fn load_line(input: &str) -> Vec<Line>
{
    let mut result = Vec::new();
    let mut x = 0;
    let mut y = 0;

    for desc in input.split(",")
    {
        let dir = desc.chars().nth(0).unwrap();
        let dist = desc.chars().skip(1).collect::<String>().parse::<i64>().unwrap();

        match dir
        {
            'R' =>
            {
                result.push(Line::new(x, y, x + dist, y));
                x += dist;
            },
            'L' =>
            {
                result.push(Line::new(x, y, x - dist, y));
                x -= dist;
            },
            'U' =>
            {
                result.push(Line::new(x, y, x, y + dist));
                y += dist;
            },
            'D' =>
            {
                result.push(Line::new(x, y, x, y - dist));
                y -= dist;
            },
            _ => {assert!(false); unreachable!();}
        }
    }

    result
}

fn load_lines(input: &str) -> (Vec<Line>, Vec<Line>)
{
    let vec = input
        .split("\n")
        .filter(|a| !a.is_empty())
        .map(|a| load_line(a))
        .collect::<Vec<_>>();

    let a = vec[0].clone();
    let b = vec[1].clone();

    (a, b)
}

fn part_1(input: &str) -> i64
{
    let (a, b) = load_lines(input);
    
    let mut intersections = Vec::new();

    for aa in a.iter()
    {
        for bb in b.iter()
        {
            if let Some((dist_origin, _dist_a, _dist_b)) = aa.intersects(bb)
            {
                if dist_origin != 0
                {
                    intersections.push(dist_origin);
                }
            }
        }
    }
    assert!(!intersections.is_empty());

    intersections.sort();
    intersections[0]
}

fn part_2(input: &str) -> i64
{
    let (a, b) = load_lines(input);
    
    let mut times = Vec::new();

    let mut total_a = 0;

    for aa in a.iter()
    {
        let mut total_b = 0;

        for bb in b.iter()
        {
            if let Some((dist_origin, dist_a, dist_b)) = aa.intersects(bb)
            {
                if dist_origin != 0
                {
                    times.push(total_a + dist_a + total_b + dist_b);
                }
            }

            total_b += bb.len();
        }

        total_a += aa.len();
    }
    assert!(!times.is_empty());

    times.sort();
    times[0]
}

fn main()
{
    const EXAMPLE_0: &str = "R8,U5,L5,D3\nU7,R6,D4,L4\n";
    const EXAMPLE_1: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83\n";
    const EXAMPLE_2: &str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7\n";

    assert_eq!(part_1(EXAMPLE_0), 6);
    assert_eq!(part_1(EXAMPLE_1), 159);
    assert_eq!(part_1(EXAMPLE_2), 135);

    let answer_1 = part_1(INPUT);
    println!("Answer #1={}", answer_1);
    assert_eq!(answer_1, 651);

    assert_eq!(part_2(EXAMPLE_1), 610);
    assert_eq!(part_2(EXAMPLE_2), 410);

    let answer_2 = part_2(INPUT);
    println!("Answer #2={}", answer_2);
    assert_eq!(answer_2, 7534);
}