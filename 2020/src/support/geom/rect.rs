use crate::support::geom::Point;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Rect
{
    min_x: i64,
    min_y: i64,
    max_x: i64,
    max_y: i64,
}

impl Rect
{
    pub fn new(p1: Point, p2: Point) -> Self
    {
        Rect
        {
            min_x: i64::min(p1.x, p2.x),
            min_y: i64::min(p1.y, p2.y),
            max_x: i64::max(p1.x, p2.x),
            max_y: i64::max(p1.y, p2.y),
        }
    }

    #[allow(dead_code)]
    pub fn new_from_coords(x1: i64, y1: i64, x2: i64, y2: i64) -> Self
    {
        Rect
        {
            min_x: i64::min(x1, x2),
            min_y: i64::min(y1, y2),
            max_x: i64::max(x1, x2),
            max_y: i64::max(y1, y2),
        }
    }

    pub fn get_min_x(&self) -> i64
    {
        self.min_x
    }

    pub fn get_min_y(&self) -> i64
    {
        self.min_y
    }

    pub fn get_max_x(&self) -> i64
    {
        self.max_x
    }

    pub fn get_max_y(&self) -> i64
    {
        self.max_y
    }

    pub fn area(&self) -> i64
    {
        (self.max_x - self.min_x) * (self.max_y - self.min_y)
    }

    pub fn perimeter(&self) -> i64
    {
        2 * (self.max_x - self.min_x)
            + 2 * (self.max_y - self.min_y)        
    }

    pub fn intersection(&self, other: Rect) -> Option<Self>
    {
        let max_min_x = i64::max(self.min_x, other.min_x);
        let max_min_y = i64::max(self.min_y, other.min_y);
        let min_max_x = i64::min(self.max_x, other.max_x);
        let min_max_y = i64::min(self.max_y, other.max_y);

        if (max_min_x <= min_max_x) && (max_min_y <= min_max_y)
        {
            Some(Rect
            {
                min_x: max_min_x,
                min_y: max_min_y,
                max_x: min_max_x,
                max_y: min_max_y,
            })
        }
        else
        {
            None
        }
    }

    pub fn does_point_intersect(&self, p: Point) -> bool
    {
        p.x >= self.min_x && p.x <= self.max_x && p.y >= self.min_y && p.y <= self.max_y
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_rect_intersection()
    {
        assert_eq!(Rect::new_from_coords(0, 0, 0, 0).intersection(Rect::new_from_coords(0, 0, 0, 0)),
            Some(Rect::new_from_coords(0, 0, 0, 0)));
    }
}
