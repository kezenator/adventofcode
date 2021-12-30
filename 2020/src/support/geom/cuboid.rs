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

    #[allow(dead_code)]
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
        let mut result = Vec::with_capacity(6);

        if !self.intersects(other)
        {
            // No intersection - just add ourselves

            result.push(*self);
        }
        else if self.completely_contained_within(other)
        {
            // We're completely contained within the
            // other item - nothing is left
        }
        else
        {
            let mut add_possibility = |x1: i64, x2: i64, y1: i64, y2: i64, z1: i64, z2: i64|
            {
                if (x1 <= x2) && (y1 <= y2) && (z1 <= z2)
                {
                    result.push(Cuboid::new(Point3::new(x1, y1, z1), Point3::new(x2, y2, z2)));
                }
            };

            // There are up to six possibilities
            // NOTE - stolen off the solutions mega thread :(
            // https://github.com/ropewalker/advent_of_code_2021/blob/master/src/day22.rs
            
            add_possibility(
                self.min.x, other.min.x - 1,
                self.min.y, self.max.y,
                self.min.z, self.max.z);

            add_possibility(
                other.max.x + 1, self.max.x,
                self.min.y, self.max.y,
                self.min.z, self.max.z);
            
            add_possibility(
                i64::max(self.min.x, other.min.x), i64::min(self.max.x, other.max.x),
                self.min.y, other.min.y - 1,
                self.min.z, self.max.z);

            add_possibility(
                i64::max(self.min.x, other.min.x), i64::min(self.max.x, other.max.x),
                other.max.y + 1, self.max.y,
                self.min.z, self.max.z);
            
            add_possibility(
                i64::max(self.min.x, other.min.x), i64::min(self.max.x, other.max.x),
                i64::max(self.min.y, other.min.y), i64::min(self.max.y, other.max.y),
                self.min.z, other.min.z - 1);

            add_possibility(
                i64::max(self.min.x, other.min.x), i64::min(self.max.x, other.max.x),
                i64::max(self.min.y, other.min.y), i64::min(self.max.y, other.max.y),
                other.max.z + 1, self.max.z);
        }
        
        if result.is_empty()
        {
            return None;
        }
        Some(result)
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