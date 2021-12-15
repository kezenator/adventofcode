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

const DIR_4: [(i64, i64); 4] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
];

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

    pub fn directions_4() -> impl Iterator<Item = Point>
    {
        DIR_4.iter()
            .map(|(dx, dy)| Point::new(*dx, *dy))
    }

    pub fn neighbours_4(&self) -> impl Iterator<Item = Point>
    {
        let px = self.x.clone();
        let py = self.y.clone();

        DIR_4.iter()
            .map(move |(dx, dy)| Point::new(px + *dx, py + *dy))
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
}
