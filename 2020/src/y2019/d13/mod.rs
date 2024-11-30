use std::collections::HashMap;
use crate::support::*;
use crate::y2019::intcode::*;

struct Arcade
{
    comp: Intcode,
    display: HashMap<Point, i64>,
    score: i64,
}

impl Arcade
{
    pub fn new(input: &str, part_2: bool) -> Self
    {
        let mut result = Arcade
        {
            comp: Intcode::new_from_input(input),
            display: HashMap::new(),
            score: 0,
        };

        if part_2
        {
            result.comp.write_mem(0, 2);
        }

        result
    }

    pub fn run(&mut self)
    {
        loop
        {
            let pause = self.comp.run_until_halt_or_input_required();

            while self.comp.output_len() >= 3
            {
                let x = self.comp.pop_output();
                let y = self.comp.pop_output();
                let tile_id = self.comp.pop_output();

                if (x == -1) && (y == 0)
                {
                    self.score = tile_id;
                }
                else
                {
                    self.display.insert(Point::new(x, y), tile_id);
                }
            }

            match pause
            {
                IntcodePause::Halted =>
                {
                    return;
                },
                IntcodePause::MoreInputRequired =>
                {
                    let x_paddle = self.display.iter()
                        .filter(|(_, &tile_id)| tile_id == 3)
                        .map(|(p, _)| p.x)
                        .next()
                        .unwrap();

                    let x_ball = self.display.iter()
                        .filter(|(_, &tile_id)| tile_id == 4)
                        .map(|(p, _)| p.x)
                        .next()
                        .unwrap();

                    if x_paddle < x_ball
                    {
                        self.comp.input(1);
                    }
                    else if x_paddle > x_ball
                    {
                        self.comp.input(-1);
                    }
                    else
                    {
                        self.comp.input(0);
                    }
                },
            }
        }
    }
}

fn part_1(input: &str) -> usize
{
    let mut arcade = Arcade::new(input, false);
    arcade.run();
    arcade.display.iter()
        .filter(|(_, &tile_id)| tile_id == 2)
        .count()
}

fn part_2(input: &str) -> i64
{
    let mut arcade = Arcade::new(input, true);
    arcade.run();
    arcade.score
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(13)
        .part_1(|input| Answer { calculated: part_1(input), expected: 233, })
        .part_2(|input| Answer { calculated: part_2(input), expected: 11991, })
}
