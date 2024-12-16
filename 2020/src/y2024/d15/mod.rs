use crate::support::*;
use itertools::*;
use std::collections::HashSet;

const EXAMPLE_SMALL: &str = include_str!("example_small.txt");
const EXAMPLE_LARGE: &str = include_str!("example_large.txt");

struct Warehouse
{
    box_width: i64,
    boxes: HashSet<Point>,
    walls: HashSet<Point>,
    robot: Point,
    moves: Vec<Point>,
}

impl Warehouse
{
    fn new(input: &str, box_width: i64) -> Self
    {
        let groups = input_to_groups(input);
        let orig_grid = CharGrid::new_from_input(&groups[0].join("\n"), '#');

        let robot = orig_grid.all_points().into_iter()
            .filter(|p| orig_grid.get_char(p) == '@')
            .map(|p| Point::new(p.x * box_width, p.y))
            .next().unwrap();

        let moves = groups[1].join("").chars()
            .map(|ch|
            {
                match ch
                {
                    '^' => Point::new(0, -1),
                    'v' => Point::new(0, 1),
                    '<' => Point::new(-1, 0),
                    '>' => Point::new(1, 0),
                    _ => unreachable!(),
                }
            })
            .collect_vec();

        let boxes = orig_grid.all_points().into_iter()
            .filter(|p| orig_grid.get_char(p) == 'O')
            .map(|p| Point::new(p.x * box_width, p.y))
            .collect();

        let walls = orig_grid.all_points().into_iter()
            .filter(|p| orig_grid.get_char(p) == '#')
            .map(|p|
                {
                    (0..box_width).into_iter()
                        .map(|i| Point::new(p.x * box_width + i, p.y))
                        .collect_vec()
                })
            .flatten()
            .collect();

        Warehouse { box_width, boxes, walls, robot, moves }
    }

    #[allow(unused)]
    fn to_string(&self) -> String
    {
        let width = self.walls.iter().map(|p| p.x).max().unwrap() + 1;
        let height = self.walls.iter().map(|p| p.y).max().unwrap() + 1;

        let mut grid = CharGrid::new_from_fill(width as usize, height as usize, '.');

        for w in self.walls.iter()
        {
            grid.put_char(w, '#');
        }

        for b in self.boxes.iter()
        {
            if self.box_width == 1
            {
                grid.put_char(b, 'O');
            }
            else
            {
                grid.put_char(b, '[');
                grid.put_char(&Point::new(b.x + 1, b.y), ']');
            }
        }

        grid.put_char(&self.robot, '@');

        return grid.to_string();
    }

    fn do_moves(&mut self)
    {
        let moves = self.moves.clone();
        for mv in moves
        {
            let cur_robot = self.robot.clone();
            if self.can_push(&cur_robot, &mv)
            {
                self.push(&cur_robot, &mv);
                self.robot = cur_robot + mv;
            }
        }
    }

    fn can_push(&self, src: &Point, dir: &Point) -> bool
    {
        let dest = *src + *dir;

        if self.walls.contains(&dest)
        {
            return false;
        }
        else if let Some(box_point) = self.find_box(dest)
        {
            match *dir
            {
                Point{x: -1, y: 0} =>
                {
                    return self.can_push(&Point::new(box_point.x, box_point.y), dir);
                },
                Point{x: 1, y: 0} =>
                {
                    return self.can_push(&Point::new(box_point.x + self.box_width - 1, box_point.y), dir);
                },
                Point{x: 0, y: _ } =>
                {
                    for i in 0..self.box_width
                    {
                        if !self.can_push(&Point::new(box_point.x + i, box_point.y), dir)
                        {
                            return false;
                        }
                    }
                    return true;
                },
                _ => unreachable!(),
            }
        }
        // Must be empty!
        return true;
    }

    fn find_box(&self, point: Point) -> Option<Point>
    {
        for i in 0..self.box_width
        {
            let offset_p = Point::new(point.x - i, point.y);
            if self.boxes.contains(&offset_p)
            {
                return Some(offset_p);
            }
        }
        None
    }

    fn push(&mut self, src: &Point, dir: &Point)
    {
        let dest = *src + *dir;
        if let Some(box_point) = self.find_box(dest)
        {
            match *dir
            {
                Point{x: -1, y: 0} =>
                {
                    self.push(&box_point, dir);
                },
                Point{x: 1, y: 0} =>
                {
                    self.push(&(box_point + Point::new(self.box_width - 1, 0)), dir);
                },
                Point{x: 0, y: _ } =>
                {
                    for i in 0..self.box_width
                    {
                        self.push(&(box_point + Point::new(i, 0)), dir);
                    }
                },
                _ => unreachable!(),
            }
            self.boxes.remove(&box_point);
            self.boxes.insert(box_point + *dir);
        }
    }

    fn gps_sum(&self) -> i64
    {
        self.boxes.iter()
            .map(|p| p.y * 100 + p.x)
            .sum()
    }
}

fn part_1(input: &str) -> i64 {
    let mut warehouse = Warehouse::new(input, 1);
    warehouse.do_moves();
    warehouse.gps_sum()
}

fn part_2(input: &str) -> i64 {
    let mut warehouse = Warehouse::new(input, 2);
    warehouse.do_moves();
    warehouse.gps_sum()
}

pub fn puzzles() -> PuzzleDay {
    puzzle_day(15)
        .example(|| Answer {
            calculated: part_1(EXAMPLE_SMALL),
            expected: 2028,
        })
        .example(|| Answer {
            calculated: part_1(EXAMPLE_LARGE),
            expected: 10092,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 1294459,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE_LARGE),
            expected: 9021,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 1319212,
        })
}
