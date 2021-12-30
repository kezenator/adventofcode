use crate::support::geom::Point;
use crate::support::geom::Line;

use std::convert::TryFrom;

pub struct LineEquation
{
    start: Point,
    dir: Point,
    num_points: i64,
}

impl From<Line> for LineEquation
{
    fn from(line: Line) -> Self
    {
        if line.start == line.end
        {
            // Point

            LineEquation
            {
                start: line.start,
                dir: Point::new(0, 0),
                num_points: 1,
            }
        }
        else if line.start.x == line.end.x
        {
            // Vertical

            LineEquation
            {
                start: line.start,
                dir: Point::new(0, (line.end.y - line.start.y).signum()),
                num_points: (line.end.y - line.start.y).abs() + 1,
            }
        }
        else if line.start.y == line.end.y
        {
            // Horizontal

            LineEquation
            {
                start: line.start,
                dir: Point::new((line.end.x - line.start.x).signum(), 0),
                num_points: (line.end.x - line.start.x).abs() + 1,
            }
        }
        else
        {
            // Diagonal

            let sx = (line.end.x - line.start.x).signum();
            let sy = (line.end.y - line.start.y).signum();

            let lx = (line.end.x - line.start.x).abs();
            let ly = (line.end.y - line.start.y).abs();

            let div = crate::support::num::gcd(lx as u64, ly as u64) as i64;

            LineEquation
            {
                start: line.start,
                dir: Point::new(sx * (lx / div), sy * (ly / div)),
                num_points: div + 1,
            }
        }
    }
}

impl LineEquation
{
    #[allow(dead_code)]
    pub fn num_points(&self) -> i64
    {
        self.num_points
    }

    #[allow(dead_code)]
    pub fn point_at_index(&self, i: i64) -> Point
    {
        self.start + (i * self.dir)
    }

    pub fn points_inclusive(&self) -> Points
    {
        Points
        {
            point: self.start,
            dir: self.dir,
            remaining: self.num_points,
        }
    }

    pub fn points_exclusive(&self) -> Points
    {
        Points
        {
            point: self.start + self.dir,
            dir: self.dir,
            remaining: self.num_points - 2,
        }
    }
}

pub struct Points
{
    point: Point,
    dir: Point,
    remaining: i64,
}

impl Iterator for Points
{
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.remaining > 0
        {
            let result = self.point;

            self.point = self.point + self.dir;
            self.remaining -= 1;

            return Some(result);
        }
        else
        {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>)
    {
        if let Ok(len) = usize::try_from(self.remaining)
        {
            (len, Some(len))
        }
        else
        {
            (0, None)
        }
    }
}
