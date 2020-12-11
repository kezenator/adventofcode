use itertools::Itertools;
use crate::support::*;

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

        let width = lines[0].len() as i64;
        let height = lines.len() as i64;

        let chars = lines.iter().map(|s| s.chars()).flatten().collect::<Vec<char>>();

        assert_eq!(chars.len() as i64, width * height);

        CharGrid
        {
            chars,
            width,
            height,
            default,
        }
    }

    #[allow(dead_code)]
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
