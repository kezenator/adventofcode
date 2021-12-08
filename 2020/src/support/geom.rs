use std::collections::HashSet;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point
{
    // Points are cartesian co-ordinates.
    // Positive X is to the right.
    // Positive Y is up.
    // Rotating (0, 1) 90 degrees "right" will give (1, 0)

    pub x: i64,
    pub y: i64,
}

impl Point
{
    pub fn new(x: i64, y: i64) -> Self
    {
        Point { x, y }
    }

    pub fn manhatten_size(&self) -> i64
    {
        self.x.abs() + self.y.abs()
    }

    pub fn directions_4() -> Vec<Point>
    {
        vec![
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(-1, 0),
            Point::new(0, -1),
        ]
    }

    pub fn directions_8() -> Vec<Point>
    {
        vec![
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(1, 1),

            Point::new(-1, 0),
            Point::new(0, -1),
            Point::new(-1, -1),

            Point::new(1, -1),
            Point::new(-1, 1),
        ]
    }

    pub fn rotate_90_left(&self) -> Point
    {
        Point::new(-self.y, self.x)
    }

    pub fn rotate_90_right(&self) -> Point
    {
        Point::new(self.y, -self.x)
    }

    pub fn rotate_180(&self) -> Point
    {
        self.invert()
    }

    pub fn invert(&self) -> Point
    {
        Point::new(-self.x, -self.y)
    }

    pub fn degrees_clockwise_from_up(&self) -> f64
    {
        // Up (0, 1) is up - i.e. zero degrees, then clockwise
        // So (1, 0) is 90 degrees, (0, -1) is 180, and (-1, 0) is 270

        let ang = (self.y as f64).atan2(self.x as f64);

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

impl std::ops::Add for Point
{
    type Output = Point;
    
    fn add(self, rhs: Point) -> Point
    {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::AddAssign for Point
{
    fn add_assign(&mut self, rhs: Point)
    {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub for Point
{
    type Output = Point;
    
    fn sub(self, rhs: Point) -> Point
    {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::SubAssign for Point
{
    fn sub_assign(&mut self, rhs: Point)
    {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::Mul<i64> for Point
{
    type Output = Point;
    
    fn mul(self, rhs: i64) -> Point
    {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl std::ops::Mul<Point> for i64
{
    type Output = Point;
    
    fn mul(self, rhs: Point) -> Point
    {
        Point::new(self * rhs.x, self * rhs.y)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Line
{
    pub start: Point,
    pub end: Point,
}

impl Line
{
    pub fn new(start: Point, end: Point) -> Self
    {
        Line { start, end }
    }

    pub fn new_from_coords(start_x: i64, start_y: i64, end_x: i64, end_y: i64) -> Self
    {
        Line{ start: Point::new(start_x, start_y), end: Point::new(end_x, end_y) }
    }

    pub fn manhatten_len(&self) -> i64
    {
        (self.start - self.end).manhatten_size()
    }

    pub fn intersection(&self, other: &Line) -> Option<Point>
    {
        if (self.start.x == self.end.x)
            && (other.start.y == other.end.y)
        {
            // Vertical line and other is horizontal

            let our_x = self.start.x;
            let other_y = other.start.y;

            let our_min_y = std::cmp::min(self.start.y, self.end.y);
            let our_max_y = std::cmp::max(self.start.y, self.end.y);

            let other_min_x = std::cmp::min(other.start.x, other.end.x);
            let other_max_x = std::cmp::max(other.start.x, other.end.x);

            if (our_min_y <= other_y) && (other_y <= our_max_y)
                && (other_min_x < our_x) && (our_x <= other_max_x)
            {
                return Some(Point::new(our_x, other_y));
            }
            return None;
        }
        else if (self.start.y == self.end.y)
            && (other.start.x == other.end.x)
        {
            // Horizontal line and other is vertical -
            // can be solved by calling in the reverse order

            return other.intersection(self);
        }

        // Diagonal lines - find points that
        // are on both lines

        let a = self.points_exactly_on_line_exclusive().drain(..).collect::<HashSet<Point>>();
        let b = other.points_exactly_on_line_exclusive().drain(..).collect::<HashSet<Point>>();
        
        let c = a.intersection(&b).map(|a| *a).collect::<Vec<Point>>();

        match c.len()
        {
            0 => None,          // Don't intersect
            1 => Some(c[0]),    // Intersect at a point
            _ => None,          // Parallel
        }
    }

    fn points_exactly_on_line(&self, inclusive: bool) -> Vec<Point>
    {
        let dir_x = (self.end.x - self.start.x).signum();
        let dir_y = (self.end.y - self.start.y).signum();

        let len_x = (self.end.x - self.start.x).abs();
        let len_y = (self.end.y - self.start.y).abs();

        let (steps, step_x, step_y) = if (len_x == len_y) || (len_x == 0) || (len_y == 0)
        {
            // 1. Diagnoals at 45 degress (x and y lengths are the same)
            // 2. Vertical (x length is zero)
            // 3. Horizontal (y length is zero)

            (i64::max(len_x, len_y), 1, 1)
        }
        else
        {
            // Other non-standard angles.
            // Find the GCD of the length and use it to
            // find the step size

            let gcd = crate::gcd(len_x as u64, len_y as u64) as i64;
            assert!(gcd >= 1);

            (gcd, len_x / gcd, len_y / gcd)
        };

        let mut result = Vec::new();

        if inclusive
        {
            result.push(self.start);
        }

        for i in 1..steps
        {
            result.push(Point::new(
                self.start.x + i * step_x * dir_x,
                self.start.y + i * step_y * dir_y));
        }

        if inclusive && steps > 0
        {
            result.push(self.end);
        }

        result
    }

    pub fn points_exactly_on_line_inclusive(&self) -> Vec<Point>
    {
        self.points_exactly_on_line(true)
    }

    pub fn points_exactly_on_line_exclusive(&self) -> Vec<Point>
    {
        self.points_exactly_on_line(false)
    }

    pub fn degrees_clockwise_from_up(&self) -> f64
    {
        (self.end - self.start).degrees_clockwise_from_up()
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_point_manhatten_size()
    {
        assert_eq!(Point::new(0, 0).manhatten_size(), 0);
        assert_eq!(Point::new(5, 0).manhatten_size(), 5);
        assert_eq!(Point::new(0, 6).manhatten_size(), 6);
        assert_eq!(Point::new(7, 8).manhatten_size(), 15);
        assert_eq!(Point::new(-1, 2).manhatten_size(), 3);
        assert_eq!(Point::new(3, -4).manhatten_size(), 7);
    }

    #[test]
    fn test_point_rotations()
    {
        let check_rotation_cycle = |points: Vec<Point>|
        {
            for i in 0..4
            {
                let next = (i + 1) % 4;
                let prev = (i + 3) % 4;
                let opposite = (i + 2) % 4;

                assert_eq!(points[i].rotate_90_right(), points[next]);
                assert_eq!(points[i].rotate_90_left(), points[prev]);
                assert_eq!(points[i].rotate_180(), points[opposite]);
                assert_eq!(points[i].invert(), points[opposite]);
            }
        };

        check_rotation_cycle(vec!(Point::new(0, -1), Point::new(-1, 0), Point::new(0, 1), Point::new(1, 0)));
        check_rotation_cycle(vec!(Point::new(1, -1), Point::new(-1, -1), Point::new(-1, 1), Point::new(1, 1)));
        check_rotation_cycle(vec!(Point::new(7, -3), Point::new(-3, -7), Point::new(-7, 3), Point::new(3, 7)));
    }

    #[test]
    fn test_line_degrees_clockwise_from_up()
    {
        let points = vec![
            Point::new(0, 1),
            Point::new(1, 2),
            Point::new(1, 1),
            Point::new(2, 1),
            Point::new(1, 0),
            Point::new(2, -1),
            Point::new(1, -1),
            Point::new(1, -2),
            Point::new(0, -1),
            Point::new(-1, -2),
            Point::new(-1, -1),
            Point::new(-2, -1),
            Point::new(-1, -0),
            Point::new(-2, 1),
            Point::new(-1, 1),
            Point::new(-1, 2),
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

        check_angle(0, 1, 0.0);
        check_angle(1, 1, 45.0);
        check_angle(1, 0, 90.0);
        check_angle(1, -1, 135.0);
        check_angle(0, -1, 180.0);
        check_angle(-1, -1, 225.0);
        check_angle(-1, 0, 270.0);
        check_angle(-1, 1, 315.0);
    }

    #[test]
    fn test_line_intersection()
    {
        // Horizontal and vertical in both directions

        let l_04_54 = Line::new(Point::new(0, 4), Point::new(5, 4));
        let l_30_35 = Line::new(Point::new(3, 0), Point::new(3, 5));

        assert_eq!(l_04_54.intersection(&l_30_35), Some(Point::new(3, 4)));
        assert_eq!(l_30_35.intersection(&l_04_54), Some(Point::new(3, 4)));

        let l_54_04 = Line::new(Point::new(5, 4), Point::new(0, 4));
        let l_35_30 = Line::new(Point::new(3, 5), Point::new(3, 0));

        assert_eq!(l_54_04.intersection(&l_35_30), Some(Point::new(3, 4)));
        assert_eq!(l_35_30.intersection(&l_54_04), Some(Point::new(3, 4)));

        // Horizontal lines that don't cross

        assert_eq!(Line::new_from_coords(0, 0, 10, 0).intersection(&Line::new_from_coords(0, 0, 10, 0)), None);
        assert_eq!(Line::new_from_coords(0, 1, 10, 1).intersection(&Line::new_from_coords(0, 1, 10, 1)), None);

        // Some diagonal lines that cross at an exact point

        assert_eq!(Line::new_from_coords(0, 0, 10, 10).intersection(&Line::new_from_coords(0, 10, 10, 0)), Some(Point::new(5, 5)));
        assert_eq!(Line::new_from_coords(0, 5, 10, 5).intersection(&Line::new_from_coords(5, 0, 5, 10)), Some(Point::new(5, 5)));

        // These lines cross, but not at an exact point...
        assert_eq!(Line::new_from_coords(0, 0, 1, 1).intersection(&Line::new_from_coords(0, 1, 1, 0)), None);
    }

    #[test]
    fn test_line_points_exactly_on_line()
    {
        let test_line = |line: Line, points_inclusive: Vec<Point>|
        {
            assert_eq!(
                line.points_exactly_on_line_inclusive(),
                points_inclusive);

            let mut points_exclusive = points_inclusive;
            if points_exclusive.len() > 0
            {
                points_exclusive.pop();
            }
            if points_exclusive.len() > 0
            {
                points_exclusive.remove(0);
            }

            assert_eq!(
                line.points_exactly_on_line_exclusive(),
                points_exclusive);
        };

        // Empty line

        test_line(
            Line::new_from_coords(0, 0, 0, 0),
            vec![Point::new(0, 0)]);

        // Horizontal lines

        test_line(
            Line::new(Point::new(0, 0), Point::new(2, 0)),
            vec![Point::new(0, 0), Point::new(1, 0), Point::new(2, 0)]);

        test_line(
            Line::new(Point::new(0, 0), Point::new(-2, 0)),
            vec![Point::new(0, 0), Point::new(-1, 0), Point::new(-2, 0)]);

        // Vertical lines

        test_line(
            Line::new(Point::new(0, 0), Point::new(0, 2)),
            vec![Point::new(0, 0), Point::new(0, 1), Point::new(0, 2)]);

        test_line(
            Line::new(Point::new(0, 0), Point::new(0, -2)),
            vec![Point::new(0, 0), Point::new(0, -1), Point::new(0, -2)]);

        // Diagonal lines @ 45 degrees

        test_line(
            Line::new(Point::new(0, 0), Point::new(2, 2)),
            vec![Point::new(0, 0), Point::new(1, 1), Point::new(2, 2)]);
        test_line(
            Line::new(Point::new(0, 0), Point::new(-2, 2)),
            vec![Point::new(0, 0), Point::new(-1, 1), Point::new(-2, 2)]);
        test_line(
            Line::new(Point::new(0, 0), Point::new(2, -2)),
            vec![Point::new(0, 0), Point::new(1, -1), Point::new(2, -2)]);
        test_line(
            Line::new(Point::new(0, 0), Point::new(-2, -2)),
            vec![Point::new(0, 0), Point::new(-1, -1), Point::new(-2, -2)]);

        // Even more tests....

        test_line(Line::new_from_coords(0, 0, 0, 0), vec![Point::new(0, 0)]);
        test_line(Line::new_from_coords(0, 0, 1, 0), vec![Point::new(0, 0), Point::new(1, 0)]);
        test_line(Line::new_from_coords(0, 0, 2, 0), vec![Point::new(0, 0), Point::new(1, 0), Point::new(2, 0)]);
        test_line(Line::new_from_coords(0, 0, 0, 1), vec![Point::new(0, 0), Point::new(0, 1)]);
        test_line(Line::new_from_coords(0, 0, 0, 2), vec![Point::new(0, 0), Point::new(0, 1), Point::new(0, 2)]);
        test_line(Line::new_from_coords(0, 0, 1, 1), vec![Point::new(0, 0), Point::new(1, 1)]);
        test_line(Line::new_from_coords(0, 0, 2, 2), vec![Point::new(0, 0), Point::new(1, 1), Point::new(2, 2)]);
        test_line(Line::new_from_coords(0, 0, 2, -2), vec![Point::new(0, 0), Point::new(1, -1), Point::new(2, -2)]);
        test_line(Line::new_from_coords(0, 0, -2, 2), vec![Point::new(0, 0), Point::new(-1, 1), Point::new(-2, 2)]);
        test_line(Line::new_from_coords(0, 0, -2, -2), vec![Point::new(0, 0), Point::new(-1, -1), Point::new(-2, -2)]);
        test_line(Line::new_from_coords(0, 0, 4, -6), vec![Point::new(0, 0), Point::new(2, -3), Point::new(4, -6)]);

        test_line(Line::new_from_coords(0, 0, 3, 0), vec![Point::new(0, 0), Point::new(1, 0), Point::new(2, 0), Point::new(3, 0)]);
        test_line(Line::new_from_coords(0, 0, -3, 0), vec![Point::new(0, 0), Point::new(-1, 0), Point::new(-2, 0), Point::new(-3, 0)]);
        test_line(Line::new_from_coords(0, 0, 0, 3), vec![Point::new(0, 0), Point::new(0, 1), Point::new(0, 2), Point::new(0, 3)]);
        test_line(Line::new_from_coords(0, 0, 0, -3), vec![Point::new(0, 0), Point::new(0, -1), Point::new(0, -2), Point::new(0, -3)]);

        // X=30 is 2x3x5, Y=105 is 3x5x7, should be 3x5=15 points, X spaced by 2, Y spaced by 7
        test_line(Line::new_from_coords(0, 0, -30, 105), (0..16).map(|i| Point::new(-2*i, 7*i)).collect::<Vec<_>>());
        // Now offset by (9, 11), and in a different direction
        test_line(Line::new_from_coords(9, -11, 39, -116), (0..16).map(|i| Point::new(9+2*i, -11-7*i)).collect::<Vec<_>>());
    }
}
