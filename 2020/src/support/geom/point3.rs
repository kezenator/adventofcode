#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point3
{
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point3
{
    pub fn new(x: i64, y: i64, z: i64) -> Self
    {
        Point3 { x, y, z }
    }

    pub fn manhatten_size(&self) -> i64
    {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl std::fmt::Debug for Point3
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        fmt.write_fmt(format_args!("({}, {}, {})", self.x, self.y, self.z))
    }
}

impl std::ops::Add for Point3
{
    type Output = Point3;
    
    fn add(self, rhs: Point3) -> Point3
    {
        Point3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::AddAssign for Point3
{
    fn add_assign(&mut self, rhs: Point3)
    {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Sub for Point3
{
    type Output = Point3;
    
    fn sub(self, rhs: Point3) -> Point3
    {
        Point3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::SubAssign for Point3
{
    fn sub_assign(&mut self, rhs: Point3)
    {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

