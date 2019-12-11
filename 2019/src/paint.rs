use super::geom::Point;

#[derive(Debug)]
pub struct PaintPoint
{
    pub point: Point,
    pub ch: Option<char>,
}

impl PaintPoint
{
    pub fn new(point: Point, ch: Option<char>) -> Self
    {
        PaintPoint { point, ch }
    }
}

pub fn render(paint_points: &Vec<PaintPoint>) -> String
{
    assert!(paint_points.len() != 0);

    // Work out the min/max co-ordinates

    let mut xs = paint_points.iter().map(|paint| paint.point.x).collect::<Vec<_>>();
    let mut ys = paint_points.iter().map(|paint| paint.point.y).collect::<Vec<_>>();

    xs.sort();
    ys.sort();

    let min_x = xs.first().unwrap();
    let max_x = xs.last().unwrap();
    let min_y = ys.first().unwrap();
    let max_y = ys.last().unwrap();

    // Now we know the size

    let size_x = ((max_x - min_x) + 1) as usize;
    let size_y = ((max_y - min_y) + 1) as usize;

    // Setup a set of characters - each row needs
    // the size_x characters plus one extra for a new line

    let mut chars = vec![' '; size_y * (size_x + 1)];

    // Put a new line at the end of each row

    for y in 0..size_y
    {
        chars[y * (size_x + 1) + size_x] = '\n';
    }

    // Perform all of the paint operations

    for paint in paint_points.iter()
    {
        if let Some(ch) = paint.ch
        {
            let off_x = (paint.point.x - min_x) as usize;
            let off_y = (paint.point.y - min_y) as usize;

            chars[off_y * (size_x + 1) + off_x] = ch;
        }
    }

    // Finally, collect the characters into a string

    chars.drain(..).collect::<String>()
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_render()
    {
        let points = vec![
            PaintPoint::new(Point::new(-1, -1), Some('*')),
            PaintPoint::new(Point::new(1, 1), Some('*')),
            PaintPoint::new(Point::new(-1, -1), Some('a')),
            PaintPoint::new(Point::new(1, -1), Some('b')),
            PaintPoint::new(Point::new(0, 0), Some('c')),
            PaintPoint::new(Point::new(-1, 1), Some('d')),
            PaintPoint::new(Point::new(0, 1), Some('e')),
            PaintPoint::new(Point::new(1, 1), Some('f')),
            PaintPoint::new(Point::new(1, 1), None),
        ];

        let result = render(&points);

        assert_eq!(result, "a b\n c \ndef\n".to_owned());
    }
}
