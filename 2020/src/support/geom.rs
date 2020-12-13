#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

    pub fn manhatten_size(&self) -> i64
    {
        self.x.abs() + self.y.abs()
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

    pub fn rotate_left(&self) -> Point
    {
        Point::new(-self.y, self.x)
    }

    pub fn rotate_right(&self) -> Point
    {
        Point::new(self.y, -self.x)
    }

    pub fn invert(&self) -> Point
    {
        Point::new(-self.x, -self.y)
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

    pub fn manhatten_len(&self) -> i64
    {
        (self.start - self.end).manhatten_size()
    }

    pub fn intersection(&self, other: &Line) -> Option<Point>
    {
        if self.start.x == self.end.x
        {
            // Vertical line - check if the other is horizontal

            if other.start.y == other.end.y
            {
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
            }
        }
        else if self.start.y == self.end.y
        {
            // Horizontal line - if the other line is vertical then it
            // can be solved by calling in the reverse order

            if other.start.x == other.end.x
            {
                return other.intersection(self);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_point()
    {
        assert_eq!(Point::new(0, 0).manhatten_size(), 0);
        assert_eq!(Point::new(5, 0).manhatten_size(), 5);
        assert_eq!(Point::new(0, 6).manhatten_size(), 6);
        assert_eq!(Point::new(7, 8).manhatten_size(), 15);
        assert_eq!(Point::new(-1, 2).manhatten_size(), 3);
        assert_eq!(Point::new(3, -4).manhatten_size(), 7);
    }

    #[test]
    fn test_line()
    {
        let l_04_54 = Line::new(Point::new(0, 4), Point::new(5, 4));
        let l_30_35 = Line::new(Point::new(3, 0), Point::new(3, 5));

        assert_eq!(l_04_54.intersection(&l_30_35), Some(Point::new(3, 4)));
        assert_eq!(l_30_35.intersection(&l_04_54), Some(Point::new(3, 4)));

        let l_54_04 = Line::new(Point::new(5, 4), Point::new(0, 4));
        let l_35_30 = Line::new(Point::new(3, 5), Point::new(3, 0));

        assert_eq!(l_54_04.intersection(&l_35_30), Some(Point::new(3, 4)));
        assert_eq!(l_35_30.intersection(&l_54_04), Some(Point::new(3, 4)));
    }
}
