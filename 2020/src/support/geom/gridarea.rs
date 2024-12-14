use std::collections::HashSet;
use itertools::*;
use crate::support::Point;
use crate::support::RangeSet;

pub struct GridArea
{
    pub points: HashSet<Point>,
}

impl GridArea
{
    pub fn area(&self) -> usize
    {
        // Area is simply the number of points
        self.points.len()
    }

    pub fn perimeter(&self) -> usize
    {
        // The perimeter is the sum (across each point)
        // of how many neighbours it has that are not
        // in the area

        self.points.iter()
            .map(|p|
            {
                p.neighbours_4()
                    .filter(|n| !self.points.contains(n))
                    .count()
            })
            .sum()
    }

    pub fn num_edges(&self) -> usize
    {
        // For each of the 4 directions:
        // 1) Find the points with an edge in that direction (e.g. neighbour is not in area)
        //    1.1) Group by one co-ordinate, then for each group:
        //       1.1.1) Collect by their other co-ordinate into a range-set
        //       1.1.2) Determine the number of ranges (to separate co-linear edges)
        // The number of sizes is the sum across 4 directions.

        self.num_edges_in_dir(Point::new(-1, 0), |p| p.x, |p| p.y, )
            + self.num_edges_in_dir(Point::new(1, 0), |p| p.x, |p| p.y)
            + self.num_edges_in_dir(Point::new(0, -1), |p| p.y, |p| p.x)
            + self.num_edges_in_dir(Point::new(0, 1), |p| p.y, |p| p.x)
    }

    fn num_edges_in_dir<F1, F2>(&self, dir: Point, group_coord: F1, range_coord: F2) -> usize
        where F1: Fn(&Point) -> i64,
            F2: Fn(&Point) -> i64
    {
        let edge_points = self.points.iter()
            .filter(|&p| !self.points.contains(&(*p + dir)))
            .collect_vec();

        let grouped = edge_points.into_iter()
            .into_group_map_by(|p| group_coord(p));

        let ranges = grouped.into_iter()
            .map(|(_, v)|
            {
                let mut range_set = RangeSet::new();
                for p in v
                {
                    range_set.insert_value(range_coord(p));
                }                
                range_set
            })
            .collect_vec();

        ranges.into_iter()
            .map(|r| r.ranges().count())
            .sum()
    }
}