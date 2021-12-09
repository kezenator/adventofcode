use std::collections::HashSet;
use crate::support::geom::point::Point;
use crate::support::geom::line_eq::{LineEquation, Points};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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

    #[allow(dead_code)]
    pub fn new_from_coords(start_x: i64, start_y: i64, end_x: i64, end_y: i64) -> Self
    {
        Line{ start: Point::new(start_x, start_y), end: Point::new(end_x, end_y) }
    }

    pub fn equation(&self) -> LineEquation
    {
        (*self).into()
    }

    pub fn num_points(&self) -> i64
    {
        self.equation().num_points()
    }

    pub fn manhatten_len(&self) -> i64
    {
        (self.start - self.end).manhatten_size()
    }

    pub fn crossing_point(&self, other: Line) -> Option<Point>
    {
        // Returns the single point, if one exists, that is:
        // 1) On both lines
        // 2) Not the endpoint of either line
        //
        // i.e. the two lines pass through each other at a point
        // and continue on past the crossing point.
        //
        // If the lines are co-linear (i.e. touch at many points), None is returned.
        // If the lines touch at an endpoint, None is returned.

        if let Some(touch) = self.touching_point(other)
        {
            if (touch != self.start)
                && (touch != self.end)
                && (touch != other.start)
                && (touch != other.end)
            {
                return Some(touch);
            }
        }
        None
    }

    pub fn touching_point(&self, other: Line) -> Option<Point>
    {
        // Returns the single point, if one exists, that is
        // on both lines. It may be an endpoint of either line.

        let a = self.points_exactly_on_line_inclusive().collect::<HashSet<Point>>();
        let b = other.points_exactly_on_line_inclusive().collect::<HashSet<Point>>();
        
        let c = a.intersection(&b).map(|a| *a).collect::<Vec<Point>>();

        match c.len()
        {
            0 => None,          // Don't intersect
            1 => Some(c[0]),    // Intersect at a point
            _ => None,          // Parallel
        }
    }

    pub fn points_exactly_on_line_inclusive(&self) -> Points
    {
        self.equation().points_inclusive()
    }

    pub fn points_exactly_on_line_exclusive(&self) -> Points
    {
        self.equation().points_exclusive()
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

    fn check_bin_op<V1, V2, R>(v1: V1, op: &str, v2: V2, result: R, expected: R)
        where V1: Copy + std::fmt::Debug,
            V2: Copy + std::fmt::Debug,
            R: Copy + PartialEq + std::fmt::Debug,
    {
        if result != expected
        {
            panic!("{:?} {} {:?} = {:?} != {:?}", v1, op, v2, result, expected);
        }
        assert_eq!(result, expected);
    }

    #[test]
    fn test_line_common_points()
    {
        let check_common_points = |l1: Line, l2: Line, crossing: Option<Point>, touching: Option<Point>|
        {
            check_bin_op(l1, "crossing_point", l2, l1.crossing_point(l2), crossing);
            check_bin_op(l2, "crossing_point", l1, l2.crossing_point(l1), crossing);

            check_bin_op(l1, "touching_point", l2, l1.touching_point(l2), touching);
            check_bin_op(l2, "touching_point", l1, l2.touching_point(l1), touching);

            let l1 = Line::new(l1.end, l1.start);
            let l2 = Line::new(l2.end, l2.start);

            check_bin_op(l1, "crossing_point", l2, l1.crossing_point(l2), crossing);
            check_bin_op(l2, "crossing_point", l1, l2.crossing_point(l1), crossing);

            check_bin_op(l1, "touching_point", l2, l1.touching_point(l2), touching);
            check_bin_op(l2, "touching_point", l1, l2.touching_point(l1), touching);
        };

        // Same point

        check_common_points(
            Line::new_from_coords(1, 1, 1, 1),
            Line::new_from_coords(1, 1, 1, 1),
            None,
            Some(Point::new(1, 1)));

        // Different points

        check_common_points(
            Line::new_from_coords(1, 1, 1, 1),
            Line::new_from_coords(2, 2, 2, 2),
            None,
            None);

        // Point on horizontal

        check_common_points(
            Line::new_from_coords(2, 2, 2, 2),
            Line::new_from_coords(0, 2, 10, 2),
            None,
            Some(Point::new(2, 2)));

        // Point at start of horizontal

        check_common_points(
            Line::new_from_coords(-2, 2, -2, 2),
            Line::new_from_coords(-2, 2, 10, 2),
            None,
            Some(Point::new(-2, 2)));

        // Point at end of horizontal

        check_common_points(
            Line::new_from_coords(2, -2, 2, -2),
            Line::new_from_coords(0, -2, 2, -2),
            None,
            Some(Point::new(2, -2)));

        // Horizontal lines at different heights

        check_common_points(
            Line::new_from_coords(0, 1, 10, 1),
            Line::new_from_coords(0, 5, 10, 5),
            None,
            None);

        // Horizontal lines that join at the ends

        check_common_points(
            Line::new_from_coords(0, 10, 10, 10),
            Line::new_from_coords(10, 10, 20, 10),
            None,
            Some(Point::new(10, 10)));

        // Horizontal lines that overlap by mulitple points

        check_common_points(
            Line::new_from_coords(0, 10, 10, 10),
            Line::new_from_coords(5, 10, 20, 10),
            None,
            None);

        // Point on vertical

        check_common_points(
            Line::new_from_coords(-2, -2, -2, -2),
            Line::new_from_coords(-2, -20, -2, 0),
            None,
            Some(Point::new(-2, -2)));

        // Point at start of vertical

        check_common_points(
            Line::new_from_coords(5, 5, 5, 5),
            Line::new_from_coords(5, 5, 5, 10),
            None,
            Some(Point::new(5, 5)));

        // Point at end of vertical

        check_common_points(
            Line::new_from_coords(-3, 5, -3, 5),
            Line::new_from_coords(-3, -10, -3, 5),
            None,
            Some(Point::new(-3, 5)));

        // Vertical lines at different offset

        check_common_points(
            Line::new_from_coords(5, 0, 5, 10),
            Line::new_from_coords(6, 0, 6, 6),
            None,
            None);

        // Vertical lines that join at the ends

        check_common_points(
            Line::new_from_coords(-5, -5, -5, 0),
            Line::new_from_coords(-5, 0, -5, 10),
            None,
            Some(Point::new(-5, 0)));

        // Vertical lines that overlap by mulitple points

        check_common_points(
            Line::new_from_coords(-1, 5, -1, -2),
            Line::new_from_coords(-1, 1, -1, -6),
            None,
            None);

        // Horizontal and vertical lines, crossing

        check_common_points(
            Line::new_from_coords(0, 4, 5, 4),
            Line::new_from_coords(3, 0, 3, 5),
            Some(Point::new(3, 4)),
            Some(Point::new(3, 4)));

        // Horizontal and vertical lines, touching

        check_common_points(
            Line::new_from_coords(0, 4, 5, 4),
            Line::new_from_coords(3, -2, 3, 4),
            None,
            Some(Point::new(3, 4)));

        // Horizontal and vertical lines, not touching

        check_common_points(
            Line::new_from_coords(0, 4, 5, 4),
            Line::new_from_coords(3, -2, 3, 3),
            None,
            None);

        // Horizontal and vertical in both directions

        let l_04_54 = Line::new(Point::new(0, 4), Point::new(5, 4));
        let l_30_35 = Line::new(Point::new(3, 0), Point::new(3, 5));

        assert_eq!(l_04_54.crossing_point(l_30_35), Some(Point::new(3, 4)));
        assert_eq!(l_30_35.crossing_point(l_04_54), Some(Point::new(3, 4)));

        let l_54_04 = Line::new(Point::new(5, 4), Point::new(0, 4));
        let l_35_30 = Line::new(Point::new(3, 5), Point::new(3, 0));

        assert_eq!(l_54_04.crossing_point(l_35_30), Some(Point::new(3, 4)));
        assert_eq!(l_35_30.crossing_point(l_54_04), Some(Point::new(3, 4)));

        // Horizontal lines that don't cross

        assert_eq!(Line::new_from_coords(0, 0, 10, 0).crossing_point(Line::new_from_coords(0, 0, 10, 0)), None);
        assert_eq!(Line::new_from_coords(0, 1, 10, 1).crossing_point(Line::new_from_coords(0, 1, 10, 1)), None);

        // Some diagonal lines that cross at an exact point

        assert_eq!(Line::new_from_coords(0, 0, 10, 10).crossing_point(Line::new_from_coords(0, 10, 10, 0)), Some(Point::new(5, 5)));
        assert_eq!(Line::new_from_coords(0, 5, 10, 5).crossing_point(Line::new_from_coords(5, 0, 5, 10)), Some(Point::new(5, 5)));

        // These lines cross, but not at an exact point...
        assert_eq!(Line::new_from_coords(0, 0, 1, 1).crossing_point(Line::new_from_coords(0, 1, 1, 0)), None);
        assert_eq!(Line::new_from_coords(0, 0, 3, 3).crossing_point(Line::new_from_coords(0, 3, 3, 0)), None);
    }

    #[test]
    fn test_line_points_exactly_on_line()
    {
        let test_line_one_dir = |line: Line, points_inclusive: Vec<Point>|
        {
            assert_eq!(
                line.points_exactly_on_line_inclusive().collect::<Vec<Point>>(),
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
                line.points_exactly_on_line_exclusive().collect::<Vec<Point>>(),
                points_exclusive);
        };

        let test_line = |line: Line, points_inclusive: Vec<Point>|
        {
            test_line_one_dir(line.clone(), points_inclusive.clone());
            test_line_one_dir(Line::new(line.end, line.start), points_inclusive.iter().copied().rev().collect());
        };

        // Points

        test_line(
            Line::new_from_coords(0, 0, 0, 0),
            vec![Point::new(0, 0)]);

        test_line(
            Line::new_from_coords(2, 0, 2, 0),
            vec![Point::new(2, 0)]);

        test_line(
            Line::new_from_coords(-5, 0, -5, 0),
            vec![Point::new(-5, 0)]);

        test_line(
            Line::new_from_coords(0, 7, 0, 7),
            vec![Point::new(0, 7)]);

        test_line(
            Line::new_from_coords(0, -12, 0, -12),
            vec![Point::new(0, -12)]);

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
