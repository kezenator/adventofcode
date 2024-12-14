
use itertools::*;
use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");

struct Game
{
    button_a: Point,
    button_b: Point,
    prize: Point,
}

impl Game
{
    fn new_from_group(lines: Vec<String>) -> Self
    {
        let (ax, ay) = scan(&lines[0])
            .skip_str("Button A: X+")
            .take_digits().parse()
            .skip_str(", Y+")
            .take_digits().parse()
            .remaining().ignore();
        let (bx, by) = scan(&lines[1])
            .skip_str("Button B: X+")
            .take_digits().parse()
            .skip_str(", Y+")
            .take_digits().parse()
            .remaining().ignore();
        let (px, py) = scan(&lines[2])
            .skip_str("Prize: X=")
            .take_digits().parse()
            .skip_str(", Y=")
            .take_digits().parse()
            .remaining().ignore();

        Game
        {
            button_a: Point::new(ax, ay),
            button_b: Point::new(bx, by),
            prize: Point::new(px, py),
        }
    }

    fn try_find_min_cost(&self) -> Option<i64>
    {
        // For a solution:
        //     px = a * ax + b * bx   (1)
        // and py = a * ay + b * by
        //
        // Re-aranging:
        //      b = (px - a * ax) / bx
        //      b = (py - a * ay) / by (2)
        //
        // Now - substituting (2) into (1)
        //
        // px = a.ax + ((py - a.ay)/by).bx
        // px.by = a.ax.by + (py - a.ay).bx
        // px.by = a.ax.by + py.bx - a.ay.bx
        // px.by = a(ax.by - ay.bx) + py.bx
        // 0 = a(ax.by - ay.bx) + py.bx - px.by
        //
        // So - it turns out there is only one
        // possible solution of button presses -
        // we need to find this, and see if it's valid...

        // From the above equation - find the coefficients for
        // ax^2 + bx + c = 0

        let quad_a = 0;
        let quad_b = (self.button_a.x * self.button_b.y) - (self.button_a.y * self.button_b.x);
        let quad_c = (self.prize.y * self.button_b.x) - (self.prize.x * self.button_b.y);

        let solution = diophantine::Quadratic::new(quad_a, quad_b, quad_c).solve();

        assert!(solution.zeros.count() <= 1);

        if let Some(a) = solution.zeros.values().next()
        {
            // Now - solve for b

            let eq_2_num = self.prize.y - a * self.button_a.y;
            assert!(eq_2_num  % self.button_b.y == 0);
            let b = eq_2_num / self.button_b.y;

            // Can only take a positive number
            // of button presses

            if (a >= 0) && (b >= 0)
            {
                // Token cost based on button presses
                return Some(3 * a + b);
            }
        }
        None
    }
}

fn parse_games(input: &str) -> Vec<Game>
{
    input_to_groups(input).into_iter()
        .map(|g| Game::new_from_group(g))
        .collect_vec()
}

fn part_1(input: &str) -> i64
{
    let games = parse_games(input);

    games.into_iter()
        .map(|g| g.try_find_min_cost())
        .filter_map(|r| r)
        .sum()
}

fn part_2(input: &str) -> i64
{
    let mut games = parse_games(input);

    for g in games.iter_mut()
    {
        // you discover that the claw is nowhere near where you expected it would be
        // ...
        // due to a unit conversion error

        g.prize.x += 10000000000000;
        g.prize.y += 10000000000000;
    }

    games.into_iter()
        .map(|g| g.try_find_min_cost())
        .filter_map(|r| r)
        .sum()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(13)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 480,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 32067,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 92871736253789i64,
        })
}
