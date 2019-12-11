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

    pub fn rotate_90_clockwise(&self) -> Point
    {
        // 2D Rotation matix:
        // x = x cos w - y sin w
        // y = x sin w + y code w
        // For 90 clockwise (with the opposite y axis direction), sin w = 1, cos w = 0
        // so
        // x = -y
        // y = x
        Point::new(-self.y, self.x)
    }

    pub fn rotate_90_anticlockwise(&self) -> Point
    {
        // 2D Rotation matix:
        // x = x cos w - y sin w
        // y = x sin w + y code w
        // For 90 clockwise (with the opposite y axis direction), sin w = -1, cos w = 0
        // so
        // x = y
        // y = -x
        Point::new(self.y, -self.x)
    }

    pub fn rotate_180(&self) -> Point
    {
        Point::new(-self.x, -self.y)
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
        let mut points = Vec::new();

        let sign_x = (self.to.x - self.from.x).signum();
        let sign_y = (self.to.y - self.from.y).signum();

        let dist_x = (self.to.x - self.from.x).abs();
        let dist_y = (self.to.y - self.from.y).abs();

        let gcd = crate::gcd(dist_x, dist_y);

        if gcd > 0
        {
            let step_x = dist_x / gcd;
            let step_y = dist_y / gcd;

            let mut index = 1;
            while index < gcd
            {
                points.push(Point::new(
                    self.from.x + index * step_x * sign_x,
                    self.from.y + index * step_y * sign_y));

                index += 1;
            }
        }

        points
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
        // Up (0, -1) is zero, then clockwise
        // NOTE - graphics co-ordinates

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

    pub fn manhatten_len(&self) -> i64
    {
        self.from.manhatten_dist_to(&self.to)
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

        let check_rotation_cycle = |points: Vec<Point>|
        {
            for i in 0..4
            {
                let next = (i + 1) % 4;
                let prev = (i + 3) % 4;
                let opposite = (i + 2) % 4;

                assert_eq!(points[i].rotate_90_clockwise(), points[next]);
                assert_eq!(points[i].rotate_90_anticlockwise(), points[prev]);
                assert_eq!(points[i].rotate_180(), points[opposite]);
            }
        };

        check_rotation_cycle(vec!(Point::new(0, -1), Point::new(1, 0), Point::new(0, 1), Point::new(-1, 0)));
        check_rotation_cycle(vec!(Point::new(1, -1), Point::new(1, 1), Point::new(-1, 1), Point::new(-1, -1)));
        check_rotation_cycle(vec!(Point::new(7, -3), Point::new(3, 7), Point::new(-7, 3), Point::new(-3, -7)));
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

        assert_eq!(Line::from_coords(0, 0, 3, 0).points_along(), vec![Point::new(1, 0), Point::new(2, 0)]);
        assert_eq!(Line::from_coords(0, 0, -3, 0).points_along(), vec![Point::new(-1, 0), Point::new(-2, 0)]);
        assert_eq!(Line::from_coords(0, 0, 0, 3).points_along(), vec![Point::new(0, 1), Point::new(0, 2)]);
        assert_eq!(Line::from_coords(0, 0, 0, -3).points_along(), vec![Point::new(0, -1), Point::new(0, -2)]);

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
