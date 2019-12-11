use std::collections::HashSet;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
pub struct Point
{
    pub x: i64,
    pub y: i64,
}

impl Point
{
    pub fn new(x: i64, y: i64) -> Self
    {
        Point { x, y }
    }

    pub fn manhatten_dist_to(&self, other: &Point) -> i64
    {
        (self.x - other.x).abs()
            + (self.y - other.y).abs()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
pub struct Line
{
    pub from: Point,
    pub to: Point,
}

impl Line
{
    pub fn new(from: Point, to: Point) -> Self
    {
        Line{ from, to }
    }

    pub fn from_coords(from_x: i64, from_y: i64, to_x: i64, to_y: i64) -> Self
    {
        Line{ from: Point::new(from_x, from_y), to: Point::new(to_x, to_y) }
    }

    pub fn points_along(&self) -> Vec<Point>
    {
        let sign_x = (self.to.x - self.from.x).signum();
        let sign_y = (self.to.y - self.from.y).signum();

        let dist_x = (self.to.x - self.from.x).abs();
        let dist_y = (self.to.y - self.from.y).abs();

        let mut points = HashSet::new();

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

                points.insert(Point::new(
                    self.from.x + dx * sign_x,
                    self.from.y + dy * sign_y));
            }
        }

        for dy in 1..dist_y
        {
            let num = dy * dist_x;
            if num % dist_y == 0
            {
                let dx = num / dist_y;

                points.insert(Point::new(
                    self.from.x + dx * sign_x,
                    self.from.y + dy * sign_y));
            }
        }

        // Ensure we don't include either end - due to rouding errors

        points.remove(&self.from);
        points.remove(&self.to);

        // Collect the points into a vector
        // and sort in increasing distance from our source

        let mut result: Vec<Point> = points.drain().collect();

        result.sort_by(|a, b| a.manhatten_dist_to(&self.from).cmp(&b.manhatten_dist_to(&self.from)));

        result
    }

    pub fn intersection(&self, other: &Line) -> Option<Point>
    {
        let a = self.points_along().drain(..).collect::<HashSet<Point>>();
        let b = other.points_along().drain(..).collect::<HashSet<Point>>();
        
        let c = a.intersection(&b).map(|a| *a).collect::<Vec<Point>>();

        match c.len()
        {
            0 => None,          // Don't intersect
            1 => Some(c[0]),    // Intersect at a point
            _ => None,          // Parallel
        }
    }

    pub fn degrees_clockwise_from_up(&self) -> f64
    {
        // Up is zero, then clockwise

        let dx = (self.to.x - self.from.x) as f64;
        let dy = (self.to.y - self.from.y) as f64;

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

        assert!(ang >= 0.0 && ang < (2.0 * std::f64::consts::PI));

        ang.to_degrees()
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_points()
    {
        assert_eq!(Point::new(0, 0).manhatten_dist_to(&Point::new(0, 0)), 0);
        assert_eq!(Point::new(0, 0).manhatten_dist_to(&Point::new(3, 0)), 3);
        assert_eq!(Point::new(0, 0).manhatten_dist_to(&Point::new(0, 4)), 4);
        assert_eq!(Point::new(0, 0).manhatten_dist_to(&Point::new(3, 4)), 7);
    }

    #[test]
    fn test_lines()
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
        assert_eq!(Line::new(Point::new(0, 0), points[0].clone()).degrees_clockwise_from_up(), 0.0);
        for i in 1..16
        {
            let a = Line::new(Point::new(0, 0), points[i - 1].clone()).degrees_clockwise_from_up();
            let b = Line::new(Point::new(0, 0), points[i].clone()).degrees_clockwise_from_up();
            assert!(b > a);
        }

        let check_angle = |x: i64, y: i64, angle: f64|
        {
            assert_eq!(Line::new(Point::new(0, 0), Point::new(x, y)).degrees_clockwise_from_up(), angle);
        };

        check_angle(0, -1, 0.0);
        check_angle(1, -1, 45.0);
        check_angle(1, 0, 90.0);
        check_angle(1, 1, 135.0);
        check_angle(0, 1, 180.0);
        check_angle(-1, 1, 225.0);
        check_angle(-1, 0, 270.0);
        check_angle(-1, -1, 315.0);

        assert_eq!(Line::from_coords(0, 0, 0, 0).points_along(), vec![]);
        assert_eq!(Line::from_coords(0, 0, 1, 0).points_along(), vec![]);
        assert_eq!(Line::from_coords(0, 0, 2, 0).points_along(), vec![Point::new(1, 0)]);
        assert_eq!(Line::from_coords(0, 0, 0, 1).points_along(), vec![]);
        assert_eq!(Line::from_coords(0, 0, 0, 2).points_along(), vec![Point::new(0, 1)]);
        assert_eq!(Line::from_coords(0, 0, 1, 1).points_along(), vec![]);
        assert_eq!(Line::from_coords(0, 0, 2, 2).points_along(), vec![Point::new(1, 1)]);
        assert_eq!(Line::from_coords(0, 0, 2, -2).points_along(), vec![Point::new(1, -1)]);
        assert_eq!(Line::from_coords(0, 0, -2, 2).points_along(), vec![Point::new(-1, 1)]);
        assert_eq!(Line::from_coords(0, 0, -2, -2).points_along(), vec![Point::new(-1, -1)]);
        assert_eq!(Line::from_coords(0, 0, 4, -6).points_along(), vec![Point::new(2, -3)]);

        // X=30 is 2x3x5, Y=105 is 3x5x7, should be 3x5=15 points, X spaced by 2, Y spaced by 7, minus the two ends, for 13 points
        assert_eq!(Line::from_coords(0, 0, -30, 105).points_along(), (1..15).map(|i| Point::new(-2*i, 7*i)).collect::<Vec<_>>());
        // Now offset by (9, 11), and in a different direction
        assert_eq!(Line::from_coords(9, -11, 39, -116).points_along(), (1..15).map(|i| Point::new(9+2*i, -11-7*i)).collect::<Vec<_>>());

        assert_eq!(Line::from_coords(0, 0, 10, 0).intersection(&Line::from_coords(0, 0, 10, 0)), None);
        assert_eq!(Line::from_coords(0, 1, 10, 1).intersection(&Line::from_coords(0, 1, 10, 1)), None);
        assert_eq!(Line::from_coords(0, 0, 10, 10).intersection(&Line::from_coords(0, 10, 10, 0)), Some(Point::new(5, 5)));
        assert_eq!(Line::from_coords(0, 5, 10, 5).intersection(&Line::from_coords(5, 0, 5, 10)), Some(Point::new(5, 5)));

        // These lines cross, but not at an exact point...
        assert_eq!(Line::from_coords(0, 0, 1, 1).intersection(&Line::from_coords(0, 1, 1, 0)), None);
    }
}
