use std::collections::HashSet;
use std::fmt::{Debug, Error, Formatter};
use crate::support::*;

const EXAMPLE: &str = "....#\n#..#.\n#..##\n..#..\n#....";

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Pos
{
    level: i64,
    x: i64,
    y: i64,
}

impl Pos
{
    pub fn new(level: i64, x: i64, y: i64) -> Self
    {
        Self { level, x, y }
    }

    pub fn neighbours(&self, part_2: bool) -> Vec<Self>
    {
        if part_2
        {
            let mut result = Vec::new();

            for dir in Point::directions_4()
            {
                let point = Point::new(self.x, self.y) + dir;

                if point.x == 2 && point.y == 2
                {
                    // Recurse into the lower level

                    if dir.x == 1
                    {
                        for i in 0..5
                        {
                            result.push(Pos::new(self.level + 1, 0, i));
                        }
                    }
                    else if dir.x == -1
                    {
                        for i in 0..5
                        {
                            result.push(Pos::new(self.level + 1, 4, i));
                        }
                    }
                    else if dir.y == 1
                    {
                        for i in 0..5
                        {
                            result.push(Pos::new(self.level + 1, i, 0));
                        }
                    }
                    else // if dir.y == -1
                    {
                        for i in 0..5
                        {
                            result.push(Pos::new(self.level + 1, i, 4));
                        }
                    }
                }
                else if point.x < 0
                {
                    result.push(Pos::new(self.level - 1, 1, 2));
                }
                else if point.x > 4
                {
                    result.push(Pos::new(self.level - 1, 3, 2));
                }
                else if point.y < 0
                {
                    result.push(Pos::new(self.level - 1, 2, 1));
                }
                else if point.y > 4
                {
                    result.push(Pos::new(self.level - 1, 2, 3));
                }
                else
                {
                    // Just a normal neighbour in this level
                    result.push(Pos::new(self.level, point.x, point.y));
                }
            }

            result
        }
        else // part_1
        {
            Point::directions_4()
                .map(|d| Pos::new(0, self.x + d.x, self.y + d.y))
                .filter(|p| p.x >= 0 && p.x < 5 && p.y >= 0 && p.y < 5)
                .collect()
        }
    }
}

struct Eris
{
    part_2: bool,
    bugs: HashSet<Pos>,
}

impl Eris
{
    pub fn new(input: &str, part_2: bool) -> Self
    {
        let mut bugs = HashSet::new();

        let image = CharGrid::new_from_input(input, '.');

        for point in image.all_points()
        {
            if image.get_char(&point) == '#'
            {
                bugs.insert(Pos::new(0, point.x, point.y));
            }
        }

        Eris { part_2, bugs }
    }

    pub fn evolve(self) -> Self
    {
        let mut new_bugs = HashSet::new();

        for pos in self.all_positions()
        {
            let cur_bug = self.bugs.contains(&pos);
            let neighbour_bugs = pos.neighbours(self.part_2).iter()
                .filter(|p| self.bugs.contains(p))
                .count();

            let new_bug = if cur_bug && neighbour_bugs != 1
            {
                false
            }
            else if !cur_bug && (neighbour_bugs == 1 || neighbour_bugs == 2)
            {
                true
            }
            else
            {
                cur_bug
            };

            if new_bug
            {
                new_bugs.insert(pos);
            }
        }

        Eris
        {
            part_2: self.part_2,
            bugs: new_bugs,
        }
    }

    pub fn all_positions(&self) -> Vec<Pos>
    {
        let mut result = Vec::new();

        let level_range = if self.part_2
        {
            (self.min_level() - 1) .. (self.max_level() + 2)
        }
        else // part_1
        {
            0..1
        };

        for level in level_range
        {
            for y in 0..5
            {
                for x in 0..5
                {
                    if !self.part_2 || x != 2 || y != 2
                    {
                        result.push(Pos::new(level, x, y));
                    }
                }
            }
        }

        result
    }

    pub fn biodiversity(&self) -> u64
    {
        assert!(!self.part_2);

        self.bugs.iter()
            .map(|pos| 1u64 << (pos.x + 5 * pos.y))
            .sum()
    }

    pub fn num_bugs(&self) -> usize
    {
        self.bugs.len()
    }

    pub fn min_level(&self) -> i64
    {
        self.bugs.iter()
            .map(|p| p.level)
            .min()
            .unwrap_or(0)
    }

    pub fn max_level(&self) -> i64
    {
        self.bugs.iter()
            .map(|p| p.level)
            .max()
            .unwrap_or(0)
    }
}

impl Debug for Eris
{
    fn fmt<'a>(&self, fmt: &mut Formatter<'a>) -> Result<(), Error>
    {
        for level in (self.min_level())..(self.max_level() + 1)
        {
            writeln!(fmt, "Level {}", level)?;
            writeln!(fmt, "=====")?;

            for y in 0..5
            {
                for x in 0..5
                {
                    if self.part_2 && x == 2 && y == 2
                    {
                        write!(fmt, "?")?;
                    }
                    else if self.bugs.contains(&Pos::new(0, x, y))
                    {
                        write!(fmt, "#")?;
                    }
                    else
                    {
                        write!(fmt, ".")?;
                    }
                }
                writeln!(fmt)?;
            }
        }
        Ok(())
    }
}

fn part_1(input: &str) -> u64
{
    let mut seen = HashSet::new();
    let mut state = Eris::new(input, false);

    loop
    {
        if !seen.insert(state.biodiversity())
        {
            return state.biodiversity();
        }
        
        state = state.evolve();
    }
}

fn part_2(input: &str, minutes: usize) -> usize
{
    let mut state = Eris::new(input, true);

    for _ in 0..minutes
    {
        state = state.evolve();
    }

    state.num_bugs()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(24)
        .example(|| Answer{ calculated: part_1(EXAMPLE), expected: 2129920, })
        .part_1(|input| Answer { calculated: part_1(input), expected: 17863741, })
        .example(|| Answer{ calculated: part_2(EXAMPLE, 10), expected: 99, })
        .part_2(|input| Answer { calculated: part_2(input, 200), expected: 2029, })
}
