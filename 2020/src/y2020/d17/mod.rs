use std::collections::HashSet;
use crate::support::*;

const EXAMPLE_1: &str = ".#.\n..#\n###";

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point4D
{
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub w: i64,
}

impl Point4D
{
    pub fn new(x: i64, y: i64, z: i64, w: i64) -> Self
    {
        Point4D { x, y, z, w }
    }

    pub fn neighbours_26(&self) -> Vec<Self>
    {
        let mut result = Vec::new();

        for dz in -1..2
        {
            for dy in -1..2
            {
                for dx in -1..2
                {
                    if dx != 0 || dy != 0 || dz != 0
                    {
                        result.push(Point4D::new(
                            self.x + dx,
                            self.y + dy,
                            self.z + dz,
                            self.w));
                    }
                }
            }
        }
        result
    }

    pub fn neighbours_80(&self) -> Vec<Self>
    {
        let mut result = Vec::new();

        for dw in -1..2
        {
            for dz in -1..2
            {
                for dy in -1..2
                {
                    for dx in -1..2
                    {
                        if dx != 0 || dy != 0 || dz != 0 || dw != 0
                        {
                            result.push(Point4D::new(
                                self.x + dx,
                                self.y + dy,
                                self.z + dz,
                                self.w + dw));
                        }
                    }
                }
            }
        }
        result
    }
}

struct ConwayCubes
{
    active: HashSet<Point4D>,
    part_2: bool,
}

impl ConwayCubes
{
    pub fn new(input: &str, part_2: bool) -> Self
    {
        let mut active = HashSet::new();

        let image = CharGrid::new_from_input(input, '.');
        for point in image.all_points()
        {
            if image.get_char(&point) == '#'
            {
                active.insert(Point4D::new(point.x, point.y, 0, 0));
            }
        }

        ConwayCubes { active, part_2 }
    }

    pub fn update(&mut self)
    {
        let min_x = self.active.iter().map(|p| p.x).min().unwrap_or(0);
        let max_x = self.active.iter().map(|p| p.x).max().unwrap_or(0);
        let min_y = self.active.iter().map(|p| p.y).min().unwrap_or(0);
        let max_y = self.active.iter().map(|p| p.y).max().unwrap_or(0);
        let min_z = self.active.iter().map(|p| p.z).min().unwrap_or(0);
        let max_z = self.active.iter().map(|p| p.z).max().unwrap_or(0);
        let min_w = self.active.iter().map(|p| p.w).min().unwrap_or(0);
        let max_w = self.active.iter().map(|p| p.w).max().unwrap_or(0);

        let mut new_state = HashSet::new();

        for w in (min_w - 1)..(max_w + 2)
        {
            for z in (min_z - 1)..(max_z + 2)
            {
                for y in (min_y - 1)..(max_y + 2)
                {
                    for x in (min_x - 1)..(max_x + 2)
                    {
                        let point = Point4D::new(x, y, z, w);
                        let is_active = self.active.contains(&point);

                        let neighbours = if self.part_2
                        {
                            point.neighbours_80()
                        }
                        else
                        {
                            point.neighbours_26()
                        };

                        let active_neigh = neighbours.iter()
                            .filter(|p| self.active.contains(p))
                            .count();

                        let new_active = if is_active
                        {
                            if active_neigh == 2 || active_neigh == 3
                            {
                                true
                            }
                            else
                            {
                                false
                            }
                        }
                        else // currently inactive
                        {
                            if active_neigh == 3
                            {
                                true
                            }
                            else
                            {
                                false
                            }
                        };

                        if new_active
                        {
                            new_state.insert(point);
                        }
                        else
                        {
                            new_state.remove(&point);
                        }
                    }
                }
            }
        }

        self.active = new_state;
    }

    pub fn num_active(&self) -> usize
    {
        self.active.len()
    }
}

pub fn run(input: &str, part_2: bool) -> usize
{
    let mut cube = ConwayCubes::new(input, part_2);

    for _ in 0..6
    {
        cube.update();
    }

    cube.num_active()
}

pub fn part_1(input: &str) -> usize
{
    run(input, false)
}

fn part_2(input: &str) -> usize
{
    run(input, true)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(17)
        .example(|| Answer { calculated: part_1(EXAMPLE_1), expected: 112, })
        .part_1(|input| Answer { calculated: part_1(input), expected: 317, })
        .example(|| Answer { calculated: part_2(EXAMPLE_1), expected: 848, })
        .part_2(|input| Answer { calculated: part_2(input), expected: 1692, })
}
