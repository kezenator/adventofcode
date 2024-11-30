use crate::support::*;
use itertools::*;
use std::collections::{HashMap, HashSet};

const EXAMPLE: &str = include_str!("example.txt");

struct Rock
{
    points: Vec<Point>
}

impl Rock
{
    fn all() -> Vec<Self>
    {
        vec![
            vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            vec![(0, 1), (1, 2), (1, 1), (1, 0), (2, 1)],
            vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        ].into_iter()
        .map(|v|
        {
            Rock
            {
                points: v.into_iter()
                    .map(|(x, y)| Point::new(x, y))
                    .collect_vec()
            }
        })
        .collect_vec()
    }
}

#[derive(PartialEq, Eq, Hash)]
struct SearchKey
{
    rock_0_jet_index: usize,
    rock_offset: Vec<Point>,
}

struct SearchValue
{
    num_rocks_placed: u64,
    post_rock_n_height: i64,
}

struct Board
{
    filled: HashSet<Point>,
    rocks: Vec<Rock>,
    jet_patten: Vec<char>,
    rock_index: usize,
    num_rocks_placed: u64,
    jet_index: usize,
    pre_rock_0_height: i64,
    rock_0_jet_index: usize,
    rock_0_pos: Point,
    rock_offsets: Vec<Point>,
    cycle_search: HashMap<SearchKey, SearchValue>,
    cycle_time_in_blocks: Option<u64>,
    cycle_height: Option<i64>,
    faked_height: i64,
}

impl Board
{
    fn new(input: &str) -> Self
    {
        Board
        {
            filled: HashSet::new(),
            rocks: Rock::all(),
            jet_patten: input_to_lines(input)[0].chars().collect_vec(),
            rock_index: 0,
            num_rocks_placed: 0,
            jet_index: 0,
            pre_rock_0_height: 0,
            rock_0_jet_index: 0,
            rock_0_pos: Point::new(0, 0),
            rock_offsets: Vec::new(),
            cycle_search: HashMap::new(),
            cycle_time_in_blocks: None,
            cycle_height: None,
            faked_height: 0,
        }
    }

    fn height(&self) -> i64
    {
        self.faked_height
            +  self.filled.iter()
                    .map(|p| p.y)
                    .max()
                    .unwrap_or(0)
    }

    #[allow(unused)]
    fn to_char_grid(&self) -> CharGrid
    {
        CharGrid::new_from_points(self.filled
            .iter()
            .map(|p| Point::new(p.x, -p.y))
            .collect())
    }

    fn can_place_rock(&self, rock_index: usize, pos: Point) -> bool
    {
        self.rocks[rock_index].points.iter()
            .map(|p| *p + pos)
            .all(|p|
            {
                p.x >= 0
                    && p.x <= 6
                    && p.y >= 1
                    && !self.filled.contains(&p)
            })
    }

    fn place_single_rock(&mut self)
    {
        let start_height = self.height() - self.faked_height;
        let mut cur_pos = Point::new(2, 4 + start_height);

        let rock_index = self.rock_index;
        self.rock_index = (self.rock_index + 1) % self.rocks.len();

        let start_jet_index = self.jet_index;

        loop
        {
            // Push by jet
            let jet_char = self.jet_patten[self.jet_index];
            self.jet_index = (self.jet_index + 1) % self.jet_patten.len();

            let x_change = match jet_char
            {
                '<' => -1,
                '>' => 1,
                _ => unreachable!(),
            };
            let trial_jet_pos = Point::new(cur_pos.x + x_change, cur_pos.y);

            if self.can_place_rock(rock_index, trial_jet_pos)
            {
                cur_pos = trial_jet_pos;
            }

            // Fall
            let trial_fall_pos = Point::new(cur_pos.x, cur_pos.y - 1);
            
            if self.can_place_rock(rock_index, trial_fall_pos)
            {
                cur_pos = trial_fall_pos;
            }
            else
            {
                // This block has landed

                for p in &self.rocks[rock_index].points
                {
                    self.filled.insert(cur_pos + *p);
                }

                self.num_rocks_placed += 1;

                if rock_index == 0
                {
                    self.rock_0_jet_index = start_jet_index;
                    self.rock_0_pos = cur_pos;
                    self.rock_offsets.clear();
                }
                else if self.cycle_time_in_blocks.is_none()
                {
                    // Perform the search for a cycle

                    self.rock_offsets.push(cur_pos - self.rock_0_pos);

                    if rock_index + 1 == self.rocks.len()
                    {
                        self.pre_rock_0_height = self.height();

                        match self.cycle_search.insert(
                            SearchKey
                            {
                                rock_0_jet_index: self.rock_0_jet_index,
                                rock_offset: self.rock_offsets.clone(),
                            },
                            SearchValue
                            {
                                num_rocks_placed: self.num_rocks_placed,
                                post_rock_n_height: self.pre_rock_0_height,
                            })
                        {
                            None =>
                            {
                                // OK - no cycle found - keep searching
                            },
                            Some(search_value) =>
                            {
                                // We've found a cycle!

                                self.cycle_time_in_blocks = Some(
                                    self.num_rocks_placed - search_value.num_rocks_placed);

                                self.cycle_height = Some(
                                    self.pre_rock_0_height - search_value.post_rock_n_height);

                                //println!("Cycle found: {} .. {}, {} blocks, {} height",
                                //    search_value.num_rocks_placed,
                                //    self.num_rocks_placed,
                                //    self.cycle_time_in_blocks.unwrap(),
                                //    self.cycle_height.unwrap());
                            },
                        }
                    }

                    self.pre_rock_0_height = self.height();
                }

                //println!("Placed block {} at {:?}", rock_index, cur_pos);
                //println!("{}", self.to_char_grid().to_string());
                return;
            }
        }
    }

    fn place_n_rocks(&mut self, n: u64)
    {
        let mut remaining = n;

        while remaining >= 5 && self.cycle_time_in_blocks.is_none()
        {
            for _ in 0..5 { self.place_single_rock(); }
            remaining -= 5;
        }

        if self.cycle_time_in_blocks.is_some()
        {
            let num_cycles = remaining / self.cycle_time_in_blocks.unwrap();
            remaining = remaining % self.cycle_time_in_blocks.unwrap();

            self.faked_height += (num_cycles as i64) * self.cycle_height.unwrap();

            //println!("Faked {} cycles to add {} height => {} remaining",
            //    num_cycles, self.faked_height, remaining);
        }

        for _ in 0..remaining
        {
            self.place_single_rock();
        }
    }
}

fn height_after_rocks(input: &str, num_rocks: u64) -> i64
{
    let mut board = Board::new(input);
    board.place_n_rocks(num_rocks);
    board.height()
}

fn part_1(input: &str) -> i64
{
    height_after_rocks(input, 2022)
}

fn part_2(input: &str) -> i64
{
    height_after_rocks(input, 1000000000000u64)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(17)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 3068,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 3090,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 1514285714288i64,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 1530057803453i64,
        })
}
