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
}

impl std::fmt::Debug for Point3
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        fmt.write_fmt(format_args!("({}, {}, {})", self.x, self.y, self.z))
    }
}
