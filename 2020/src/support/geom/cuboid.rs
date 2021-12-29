use itertools::Itertools;
use crate::support::geom::Point3;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cuboid
{
    min: Point3,
    max: Point3,
}

impl Cuboid
{
    pub fn new(min: Point3, max: Point3) -> Self
    {
        let min_x = i64::min(min.x, max.x);
        let min_y = i64::min(min.y, max.y);
        let min_z = i64::min(min.z, max.z);

        let max_x = i64::max(min.x, max.x);
        let max_y = i64::max(min.y, max.y);
        let max_z = i64::max(min.z, max.z);

        let min = Point3::new(min_x, min_y, min_z);
        let max = Point3::new(max_x, max_y, max_z);

        Cuboid { min, max }
    }

    pub fn volume(&self) -> i64
    {
        (self.max.x - self.min.x + 1)
        * (self.max.y - self.min.y + 1)
        * (self.max.z - self.min.z + 1)
    }

    pub fn completely_contains(&self, other: Cuboid) -> bool
    {
        (self.min.x <= other.min.x)
        && (self.min.y <= other.min.y)
        && (self.min.z <= other.min.z)
        && (other.max.x <= self.max.x)
        && (other.max.y <= self.max.y)
        && (other.max.z <= self.max.z)
    }

    pub fn completely_contained_within(&self, other: Cuboid) -> bool
    {
        other.completely_contains(*self)
    }

    pub fn intersection(&self, other: Cuboid) -> Option<Cuboid>
    {
        let max_min_x = i64::max(self.min.x, other.min.x);
        let max_min_y = i64::max(self.min.y, other.min.y);
        let max_min_z = i64::max(self.min.z, other.min.z);

        let min_max_x = i64::min(self.max.x, other.max.x);
        let min_max_y = i64::min(self.max.y, other.max.y);
        let min_max_z = i64::min(self.max.z, other.max.z);

        if (max_min_x <= min_max_x)
            && (max_min_y <= min_max_y)
            && (max_min_z <= min_max_z)
        {
            Some(Cuboid
            {
                min: Point3::new(max_min_x, max_min_y, max_min_z),
                max: Point3::new(min_max_x, min_max_y, min_max_z),
            })
        }
        else
        {
            None
        }
    }

    pub fn intersects(&self, other:Cuboid) -> bool
    {
        let max_min_x = i64::max(self.min.x, other.min.x);
        let max_min_y = i64::max(self.min.y, other.min.y);
        let max_min_z = i64::max(self.min.z, other.min.z);

        let min_max_x = i64::min(self.max.x, other.max.x);
        let min_max_y = i64::min(self.max.y, other.max.y);
        let min_max_z = i64::min(self.max.z, other.max.z);

        if (max_min_x <= min_max_x)
            && (max_min_y <= min_max_y)
            && (max_min_z <= min_max_z)
        {
            true
        }
        else
        {
            false
        }
    }

    pub fn difference(&self, other: Cuboid) -> Option<Vec<Cuboid>>
    {
        self.csg_operation(
            other,
            |trial| self.intersects(trial) && !other.intersects(trial))
    }

    pub fn union(&self, other: Cuboid) -> Vec<Cuboid>
    {
        self.csg_operation(
            other,
            |trial| self.intersects(trial) || other.intersects(trial))
        .unwrap()
    }

    fn csg_operation<F>(&self, other: Cuboid, test: F) -> Option<Vec<Cuboid>>
        where F: Fn(Cuboid) -> bool
    {
        let mut result = Vec::new();

        // 1) Find the overlapping sections on each axies,
        // 2) For each item in the cartesian product (up to 3^3 = 27 parts),
        // 3) Test them against the test function
        // 4) If they pass, include in the results

        let x_ranges = Cuboid::csg_axis_sections(self.min.x, self.max.x, other.min.x, other.max.x);
        let y_ranges = Cuboid::csg_axis_sections(self.min.y, self.max.y, other.min.y, other.max.y);
        let z_ranges = Cuboid::csg_axis_sections(self.min.z, self.max.z, other.min.z, other.max.z);

        for (((x1, x2), (y1, y2)), (z1, z2)) in x_ranges.iter().copied()
            .cartesian_product(y_ranges.iter().copied())
            .cartesian_product(z_ranges.iter().copied())
        {
            let sub_cuboid = Cuboid::new(Point3::new(x1, y1, z1), Point3::new(x2, y2, z2));

            if test(sub_cuboid)
            {
                result.push(sub_cuboid);
            }
        }

        if result.is_empty()
        {
            return None;
        }
        Some(result)
    }

    fn csg_axis_sections(s_min: i64, s_max: i64, o_min: i64, o_max: i64) -> Vec<(i64, i64)>
    {
        let mut result = Vec::with_capacity(3);

        if (s_min == o_min) && (s_max == o_max)
        {
            // Exactly the same span - just add this one
            result.push((s_min, s_max));
        }
        else if (o_max < s_min) || (s_max < o_min)
        {
            // They don't overlap - return two separate spans
            result.push((s_min, s_max));
            result.push((o_min, o_max));
        }
        else
        {
            // They overlap in some way.
            // First, sort so that s-min is the smallest

            let (s_min, s_max, o_min, o_max) = if s_min <= o_min
            {
                (s_min, s_max, o_min, o_max)
            }
            else
            {
                (o_min, o_max, s_min, s_max)
            };

            // If s_min and o_min are not the same,
            // the include this segment

            if s_min != o_min
            {
                result.push((s_min, o_min - 1));
            }
            
            // We're now up to o_min.
            // Include the next section to the minimum of
            // s_max and o_max

            result.push((o_min, i64::min(s_max, o_max)));

            // Now, if s_max and o_max aren't the same
            // we need to add the range after the min
            // up to the max

            if s_max != o_max
            {
                result.push((
                    i64::min(s_max, o_max) + 1,
                    i64::max(s_max, o_max)));
            }

            // There must be at least two entries.
            // Both s_min == o_min and s_max == o_max was
            // handled above as a special case.

            assert!(result.len() >= 2);
        }

        result
    }
}

impl std::fmt::Debug for Cuboid
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        fmt.write_fmt(format_args!("({:?} => {:?})", self.min, self.max))
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_cuboid_volume()
    {
        assert_eq!(Cuboid::new(Point3::new(1, 1, 1), Point3::new(1, 1, 1)).volume(), 1);

        assert_eq!(Cuboid::new(Point3::new(-1, -1, -1), Point3::new(1, 1, 1)).volume(), 27);
        assert_eq!(Cuboid::new(Point3::new(1, 1, 1), Point3::new(-1, -1, -1)).volume(), 27);

        assert_eq!(Cuboid::new(Point3::new(1, 1, 1), Point3::new(2, 3, 5)).volume(), 30);
    }

    fn check_intersection(a: Cuboid, b: Cuboid, c: Option<Cuboid>)
    {
        assert_eq!(a.intersection(b), c);
        assert_eq!(b.intersection(a), c);
        assert_eq!(a.intersects(b), c.is_some());
        assert_eq!(b.intersects(a), c.is_some());
    }

    #[test]
    fn test_cuboid_intersection()
    {
        check_intersection(
            Cuboid::new(Point3::new(0, 0, 0), Point3::new(1, 1, 1)),
            Cuboid::new(Point3::new(2, 2, 2), Point3::new(3, 3, 3)),
            None);

        check_intersection(
            Cuboid::new(Point3::new(0, 0, 0), Point3::new(1, 1, 1)),
            Cuboid::new(Point3::new(1, 1, 1), Point3::new(3, 3, 3)),
            Some(Cuboid::new(Point3::new(1, 1, 1), Point3::new(1, 1, 1))));

        check_intersection(
            Cuboid::new(Point3::new(0, 0, 0), Point3::new(10, 10, 10)),
            Cuboid::new(Point3::new(1, 5, -10), Point3::new(2, 6, 20)),
            Some(Cuboid::new(Point3::new(1, 5, 0), Point3::new(2, 6, 10))));
    }

    fn check_difference(a: Cuboid, b: Cuboid, c: Option<Vec<Cuboid>>)
    {
        assert_eq!(
            a.difference(b),
            c);
    }

    #[test]
    fn test_cuboid_difference()
    {
        // Difference, unchanged

        check_difference(
            Cuboid::new(Point3::new(0, 0, 0), Point3::new(10, 10, 10)),
            Cuboid::new(Point3::new(11, 11, 11), Point3::new(12, 12, 12)),
            Some(vec![Cuboid::new(Point3::new(0, 0, 0), Point3::new(10, 10, 10))]));

        check_difference(
            Cuboid::new(Point3::new(0, 0, 0), Point3::new(10, 10, 10)),
            Cuboid::new(Point3::new(-11, -11, -11), Point3::new(-1, -1, -1)),
            Some(vec![Cuboid::new(Point3::new(0, 0, 0), Point3::new(10, 10, 10))]));

        // Difference leave nothing

        check_difference(
            Cuboid::new(Point3::new(0, 0, 0), Point3::new(10, 10, 10)),
            Cuboid::new(Point3::new(-1, -1, -1), Point3::new(11, 11, 11)),
            None);

        check_difference(
            Cuboid::new(Point3::new(0, 0, 0), Point3::new(10, 10, 10)),
            Cuboid::new(Point3::new(0, 0, 0), Point3::new(10, 10, 10)),
            None);

        // Cut off a whole side

        check_difference(
            Cuboid::new(Point3::new(0, 0, 0), Point3::new(10, 10, 10)),
            Cuboid::new(Point3::new(-1, -1, -1), Point3::new(5, 11, 11)),
            Some(vec![Cuboid::new(Point3::new(6, 0, 0), Point3::new(10, 10, 10))]));

        check_difference(
            Cuboid::new(Point3::new(0, 0, 0), Point3::new(10, 10, 10)),
            Cuboid::new(Point3::new(-1, -1, -1), Point3::new(11, 5, 11)),
            Some(vec![Cuboid::new(Point3::new(0, 6, 0), Point3::new(10, 10, 10))]));

        check_difference(
            Cuboid::new(Point3::new(0, 0, 0), Point3::new(10, 10, 10)),
            Cuboid::new(Point3::new(-1, -1, -1), Point3::new(11, 11, 5)),
            Some(vec![Cuboid::new(Point3::new(0, 0, 6), Point3::new(10, 10, 10))]));

        check_difference(
            Cuboid::new(Point3::new(0, 0, 0), Point3::new(10, 10, 10)),
            Cuboid::new(Point3::new(5, -1, -1), Point3::new(11, 11, 11)),
            Some(vec![Cuboid::new(Point3::new(0, 0, 0), Point3::new(4, 10, 10))]));

        check_difference(
            Cuboid::new(Point3::new(0, 0, 0), Point3::new(10, 10, 10)),
            Cuboid::new(Point3::new(-1, 5, -1), Point3::new(11, 11, 11)),
            Some(vec![Cuboid::new(Point3::new(0, 0, 0), Point3::new(10, 4, 10))]));

        check_difference(
            Cuboid::new(Point3::new(0, 0, 0), Point3::new(10, 10, 10)),
            Cuboid::new(Point3::new(-1, -1, 5), Point3::new(11, 11, 11)),
            Some(vec![Cuboid::new(Point3::new(0, 0, 0), Point3::new(10, 10, 4))]));

        // Cut out the whole middle of an axis

        check_difference(
            Cuboid::new(Point3::new(0, 0, 0), Point3::new(10, 10, 10)),
            Cuboid::new(Point3::new(5, -1, -1), Point3::new(6, 11, 11)),
            Some(vec![
                Cuboid::new(Point3::new(0, 0, 0), Point3::new(4, 10, 10)),
                Cuboid::new(Point3::new(7, 0, 0), Point3::new(10, 10, 10)),
            ]));

        check_difference(
            Cuboid::new(Point3::new(0, 0, 0), Point3::new(10, 10, 10)),
            Cuboid::new(Point3::new(-1, 5, -1), Point3::new(11, 6, 11)),
            Some(vec![
                Cuboid::new(Point3::new(0, 0, 0), Point3::new(10, 4, 10)),
                Cuboid::new(Point3::new(0, 7, 0), Point3::new(10, 10, 10)),
            ]));

        check_difference(
            Cuboid::new(Point3::new(0, 0, 0), Point3::new(10, 10, 10)),
            Cuboid::new(Point3::new(-1, -1, 5), Point3::new(11, 11, 6)),
            Some(vec![
                Cuboid::new(Point3::new(0, 0, 0), Point3::new(10, 10, 4)),
                Cuboid::new(Point3::new(0, 0, 7), Point3::new(10, 10, 10)),
            ]));
    }
}