use std::str::FromStr;
use std::collections::HashSet;
use crate::support::*;

const EXAMPLE: &str = "x=495, y=2..7\ny=7, x=495..501\nx=501, y=3..7\nx=498, y=2..4\nx=506, y=1..2\nx=498, y=10..13\nx=504, y=10..13\ny=13, x=498..504";

enum Input
{
    Horiz{y: i64, x1: i64, x2: i64},
    Vert{x: i64, y1: i64, y2: i64},
}

impl Input
{
    fn min_x(&self) -> i64
    {
        match self
        {
            Input::Horiz{x1, ..} => *x1,
            Input::Vert{x, ..} => *x,
        }
    }

    fn max_x(&self) -> i64
    {
        match self
        {
            Input::Horiz{x2, ..} => *x2,
            Input::Vert{x, ..} => *x,
        }
    }

    fn min_y(&self) -> i64
    {
        match self
        {
            Input::Horiz{y, ..} => *y,
            Input::Vert{y1, ..} => *y1,
        }
    }

    fn max_y(&self) -> i64
    {
        match self
        {
            Input::Horiz{y, ..} => *y,
            Input::Vert{y2, ..} => *y2,
        }
    }
}

impl FromStr for Input
{
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err>
    {
        let (c1, v, _c2, r1, r2) = scan(input)
            .take(1).parse::<char>()
            .skip_str("=")
            .until(", ").parse::<i64>()
            .take(1).parse::<char>()
            .skip_str("=")
            .until("..").parse::<i64>()
            .remaining().parse::<i64>();

        match c1
        {
            'x' => Ok(Input::Vert{x: v, y1: r1, y2: r2}),
            'y' => Ok(Input::Horiz{y: v, x1: r1, x2: r2}),
            _ => unreachable!(),
        }
    }
}

struct Ground
{
    grid: CharGrid,
    min_x: i64,
    min_y: i64,
    max_y: i64,
}

impl Ground
{
    fn new(input: &str) -> Self
    {
        let inputs = input_to_lines_parsed::<Input>(input);

        // Find out the range.
        // X needs extra range to handle overflows down the side.
        // Y can't have any extra range as we only count these rows.

        let min_x = inputs.iter().map(|p| p.min_x()).min().unwrap() - 2;
        let max_x = inputs.iter().map(|p| p.max_x()).max().unwrap() + 2;
        let min_y = inputs.iter().map(|p| p.min_y()).min().unwrap();
        let max_y = inputs.iter().map(|p| p.max_y()).max().unwrap();

        let mut grid = CharGrid::new_from_fill(
            (max_x - min_x + 1) as usize,
            (max_y - min_y + 1) as usize,
            '.');

        for input in inputs
        {
            match input
            {
                Input::Horiz{y, x1, x2} =>
                {
                    for x in x1..(x2 + 1)
                    {
                        grid.put_char(&Point::new(x - min_x, y - min_y), '#');
                    }
                },
                Input::Vert{x, y1, y2} =>
                {
                    for y in y1..(y2 + 1)
                    {
                        grid.put_char(&Point::new(x - min_x, y - min_y), '#');
                    }
                },
            }
        }

        Self { grid, min_x, min_y, max_y }
    }

    fn flood(&mut self)
    {
        let mut springs = vec![Point::new(500 - self.min_x, 0 - self.min_y)];

        let height = self.max_y - self.min_y + 1;

        loop
        {
            let mut new_springs = HashSet::new();
            let mut filled_fixed_water = false;

            for spring in springs.iter()
            {
                let spring = spring.clone();

                if self.grid.get_char(&spring) == '~'
                {
                    // This spring is no longer required.
                    // A previous spring had hit a box-within-a-box
                    // but filled up the outer box - so the overflow over the
                    // inner box is no longer required - don't re-add this
                    // spring to the list of new springs.
                    //
                    // e.g. the two 'x' points below
                    //
                    //       |
                    //  #    |    #
                    //  #~x~~~~~x~#
                    //  #~~#####~~#
                    //  #~~#   #~~#
                    //  #~~#   #~~#
                    //  #~~#####~~#
                    //  ###########
                }
                else
                {
                    // Flow water down from the spring

                    self.grid.put_char(&spring, '|');

                    let mut bottom = spring.clone();

                    loop
                    {
                        let below = Point::new(bottom.x, bottom.y + 1);

                        if below.y > height
                        {
                            // The spring has fallen off the bottom
                            break;
                        }

                        let ch_below = self.grid.get_char(&below);

                        if ch_below == '#' || ch_below == '~'
                        {
                            // We've reached clay or the top of the water -
                            // this is the new bottom of the spring

                            break;
                        }

                        // Empty space or spring water - keep trying to move down

                        self.grid.put_char(&below, '|');
                        bottom = below;
                    }

                    if bottom.y >= height
                    {
                        // This spring flows off the bottom of the screen.
                        // It's a stable spring - but no need to do any
                        // horizontal flow

                        new_springs.insert(spring);
                    }
                    else
                    {
                        // This spring hits clay '#' or fixed water '~'.
                        // Flow water out horizontally from here.

                        let flow_horiz = |start: Point, dir: Point| -> (Point, bool)
                        {
                            let mut pos = start;
                            loop
                            {
                                let next = pos + dir;
                                let next_ch = self.grid.get_char(&next);

                                if next_ch == '#'
                                {
                                    // Hit clay - stop
                                    return (pos, false);
                                }

                                let below = next + Point::new(0, 1);
                                let below_ch = self.grid.get_char(&below);

                                if below_ch == '.' || below_ch == '|'
                                {
                                    // Fall down from here
                                    return (next, true);
                                }
                                pos = next;
                            }
                        };

                        let (left, left_fall) = flow_horiz(bottom.clone(), Point::new(-1, 0));
                        let (right, right_fall) = flow_horiz(bottom.clone(), Point::new(1, 0));

                        if left_fall || right_fall
                        {
                            // This row cascades over an edge - fill this row as
                            // un-fixed water '|'.
                            // Keep this stable spring, and also start additional
                            // springs at the edges

                            new_springs.insert(spring);

                            for x in left.x..(right.x + 1)
                            {
                                self.grid.put_char(&Point::new(x, bottom.y), '|');
                            }

                            if left_fall
                            {
                                new_springs.insert(left);
                            }

                            if right_fall
                            {
                                new_springs.insert(right);
                            }
                        }
                        else
                        {
                            // This row is bounded by clay.
                            // Fill with fixed water
                            // and keep this stable spring to
                            // fill the next layer

                            new_springs.insert(spring);

                            for x in left.x..(right.x + 1)
                            {
                                self.grid.put_char(&Point::new(x, bottom.y), '~');
                            }

                            filled_fixed_water = true;
                        }
                    }
                }
            }

            let mut new_springs = new_springs.into_iter()
                .collect::<Vec<Point>>();

            new_springs.sort_by(|a, b|
            {
                let mut result = a.y.cmp(&b.y);
                if result == std::cmp::Ordering::Equal
                {
                    result = a.x.cmp(&b.x);
                }
                result
            });

            if springs == new_springs && !filled_fixed_water
            {
                // Reached a stable state
                break;
            }

            // Loop again with the new springs
            springs = new_springs;
        }
    }

    fn num_water_touched(&self) -> usize
    {
        self.grid.all_chars().into_iter()
            .filter(|&c| c == '|' || c == '~')
            .count()
    }

    fn num_water_fixed(&self) -> usize
    {
        self.grid.all_chars().into_iter()
            .filter(|&c| c == '~')
            .count()
    }
}

pub fn part_1(input: &str) -> usize
{
    let mut ground = Ground::new(input);

    ground.flood();

    //println!("{}", ground.grid.to_string());

    ground.num_water_touched()
}

fn part_2(input: &str) -> usize
{
    let mut ground = Ground::new(input);

    ground.flood();

    ground.num_water_fixed()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(17)
        .example(|| Answer { calculated: part_1(EXAMPLE), expected: 57, })
        .part_1(|input| Answer { calculated: part_1(input), expected: 31412, })
        .example(|| Answer { calculated: part_2(EXAMPLE), expected: 29, })
        .part_2(|input| Answer { calculated: part_2(input), expected: 25857, })
}
