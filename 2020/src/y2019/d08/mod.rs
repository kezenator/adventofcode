use crate::support::*;

const INPUT: &str = include_str!("input.txt");

//  **    ** **** *    ***
// *  *    *    * *    *  *
// *       *   *  *    *  *
// *       *  *   *    ***
// *  * *  * *    *    *
//  **   **  **** **** *
//
// i.e. CJZLP

const PART_2_ANSWER: &str = " **    ** **** *    ***  \n*  *    *    * *    *  * \n*       *   *  *    *  * \n*       *  *   *    ***  \n*  * *  * *    *    *    \n **   **  **** **** *    \n";

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

pub fn render_paint_points(paint_points: &Vec<PaintPoint>) -> String
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

    chars.drain(..).collect()
}

fn layers(input: &str, width: usize, height: usize) -> Vec<String>
{
    let per_layer = width * height;
    let num_layers = input.len() / per_layer;

    (0..num_layers).map(|i| input.chars().skip(i * per_layer).take(per_layer).collect::<String>()).collect()
}

fn get_1_x_2_for_min_0(input: &str, width: usize, height: usize) -> usize
{
    let layers = layers(input, width, height);

    let mut pairs = layers.iter().map(|l| 
        (
            l.chars().filter(|c| *c == '0').count(),
            l.chars().filter(|c| *c == '1').count() * l.chars().filter(|c| *c == '2').count(),
        )).collect::<Vec<_>>();
    pairs.sort();
    pairs[0].1
}

fn render_image(input: &str, width: usize, height: usize) -> String
{
    let mut x: usize = 0;
    let mut y: usize = 0;

    let mut paints = Vec::new();

    for ch in input.chars()
    {
        let pch = match ch
        {
            '0' => Some(' '),
            '1' => Some('*'),
            '2' => None,
            _ => Some('?'),
        };

        paints.push(PaintPoint::new(Point::new(x as i64, y as i64), pch));

        x += 1;
        if x >= width
        {
            x = 0;
            y += 1;

            if y >= height
            {
                // New layer
                y = 0;
            }
        }
    }

    paints.reverse();

    render_paint_points(&paints)
}

fn part_1() -> usize
{
    get_1_x_2_for_min_0(INPUT, 25, 6)
}

fn part_2() -> String
{
    render_image(INPUT, 25, 6)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(8)
        .example(|| Answer {
            calculated: get_1_x_2_for_min_0("123456789012", 3, 2),
            expected: 1,
        })
        .part_1(|| Answer {
            calculated: part_1(),
            expected: 2286,
        })
        .example(|| Answer {
            calculated: render_image("0222112222120000", 2, 2),
            expected: " *\n* \n",
        })
        .part_2(|| Answer {
            calculated: part_2(),
            expected: PART_2_ANSWER,
        })
}
