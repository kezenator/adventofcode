use itertools::Itertools;
use crate::support::*;
use std::collections::{HashSet, VecDeque};

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct CharGrid
{
    chars: Vec<char>,
    width: i64,
    height: i64,
    default: char,
}

impl CharGrid
{
    pub fn new_from_input(input: &str, default: char) -> Self
    {
        let lines = input_to_lines(input);

        assert!(lines.len() > 0);

        let width = lines.iter().map(|l| l.len()).max().unwrap() as i64;
        let height = lines.len() as i64;

        let chars = lines.iter()
            .map(|s| 
            {
                let mut chars = s.chars().collect_vec();
                while chars.len() < (width as usize)
                {
                    chars.push(default);
                }
                chars
            })
            .flatten()
            .collect_vec();

        assert_eq!(chars.len() as i64, width * height);

        CharGrid
        {
            chars,
            width,
            height,
            default,
        }
    }

    pub fn new_from_fill(width: usize, height: usize, default: char) -> Self
    {
        let chars = std::iter::repeat(default).take(width * height).collect();
        let width = width as i64;
        let height = height as i64;

        CharGrid
        {
            chars,
            width,
            height,
            default,
        }
    }

    pub fn new_from_points(points: Vec<Point>) -> Self
    {
        let min_x = points.iter().map(|p| p.x).min().unwrap();
        let max_x = points.iter().map(|p| p.x).max().unwrap();
        let min_y = points.iter().map(|p| p.y).min().unwrap();
        let max_y = points.iter().map(|p| p.y).max().unwrap();

        let mut image = CharGrid::new_from_fill((max_x - min_x + 1) as usize, (max_y - min_y + 1) as usize, '.');

        for p in points
        {
            image.put_char(
                &Point::new(p.x - min_x, p.y - min_y),
                '#');
        }

        image
    }

    #[allow(dead_code)]
    pub fn get_width(&self) -> i64
    {
        self.width
    }

    #[allow(dead_code)]
    pub fn get_height(&self) -> i64
    {
        self.height
    }

    #[allow(dead_code)]
    pub fn get_default(&self) -> char
    {
        self.default
    }

    pub fn is_point_in_bounds(&self, point: &Point) -> bool
    {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height        
    }

    pub fn get_char(&self, point: &Point) -> char
    {
        if self.is_point_in_bounds(point)
        {
            self.chars[(point.y * self.width + point.x) as usize]
        }
        else
        {
            self.default
        }
    }

    pub fn put_char(&mut self, point: &Point, ch: char)
    {
        if self.is_point_in_bounds(point)
        {
            self.chars[(point.y * self.width + point.x) as usize] = ch;
        }
    }

    pub fn all_points(&self) -> Vec<Point>
    {
        (0..(self.width * self.height))
            .map(|i| Point::new(i % self.width, i / self.width))
            .collect()
    }

    pub fn all_chars(&self) -> Vec<char>
    {
        self.chars.clone()
    }

    pub fn rotate_cw_90(&self) -> CharGrid
    {
        let mut result = CharGrid::new_from_fill(self.height as usize, self.width as usize, self.default);

        for y in 0..self.height
        {
            for x in 0..self.width
            {
                let ch = self.get_char(&Point::new(x, y));
                result.put_char(&Point::new(self.height - 1 - y, x), ch);
            }
        }
        
        result
    }

    pub fn flip_horizontally(&self) -> CharGrid
    {
        let mut result = CharGrid::new_from_fill(self.width as usize, self.height as usize, self.default);

        for y in 0..self.height
        {
            for x in 0..self.width
            {
                let ch = self.get_char(&Point::new(x, y));
                result.put_char(&Point::new(self.width - 1 - x, y), ch);
            }
        }
        
        result
    }

    #[allow(unused)]
    pub fn find_inside_point(&self) -> Point
    {
        for test_p in self.all_points()
        {
            if self.get_char(&test_p) == self.default
            {
                let search_result = pathfinding::directed::astar::astar(
                    &test_p,
                    |p|
                    {
                        p.neighbours_4()
                            .filter(|p| self.get_char(p) == self.default)
                            .map(|p| (p, 1))
                    },
                    |p| p.x.min(p.y).min(self.width - p.x).min(self.height - p.y),
                    |p| !self.is_point_in_bounds(p));

                if search_result.is_none()
                {
                    // This point starts on a default char,
                    // and there is path off the image only passing
                    // through inside points
                    return test_p;
                }
            }
        }
        unreachable!();
    }

    pub fn find_flood_fill_points(&self, start_point: &Point) -> GridArea
    {
        let target_char = self.get_char(start_point);
        let mut points = HashSet::new();
        let mut to_check = VecDeque::new();
        to_check.push_back(start_point.clone());

        while let Some(next_to_check) = to_check.pop_front()
        {
            if (self.get_char(&next_to_check) == target_char)
                && points.insert(next_to_check.clone())
            {
                for n in next_to_check.neighbours_4()
                {
                    to_check.push_back(n);
                }
            }
        }

        GridArea { points }
    }


    #[allow(unused)]
    pub fn flood_fill(&mut self, point: &Point, fill: char)
    {
        for p in self.find_flood_fill_points(point).points
        {
            self.put_char(&p, fill);
        }
    }
}

impl ToString for CharGrid
{
    fn to_string(&self) -> String
    {
        self.chars.iter()
            .chunks(self.width as usize).into_iter()
            .map(|ci| ci.collect::<String>())
            .join("\n")
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_char_grid()
    {
        assert_eq!(CharGrid::new_from_input("12\n34\n56", ' ').get_width(), 2);
        assert_eq!(CharGrid::new_from_input("12\n34\n56", ' ').get_height(), 3);
        assert_eq!(CharGrid::new_from_input("12\n34\n56", ' ').to_string(), "12\n34\n56");
        assert_eq!(CharGrid::new_from_fill(3, 2, '*').to_string(), "***\n***");
    }
}
