use std::collections::HashSet;
use crate::support::*;

const EXAMPLE: &str = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL";
const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Seats
{
    vals: Vec<Vec<char>>,
    max_distance: usize,
    occ_rule: usize,
}

impl Seats
{
    pub fn new_1(input: &str) -> Self
    {
        Seats
        {
            vals: input_to_lines(input)
                .iter()
                .map(|l| l.chars().collect())
                .collect(),
            max_distance: 1,
            occ_rule: 4,
        }
    }

    pub fn new_2(input: &str) -> Self
    {
        Seats
        {
            vals: input_to_lines(input)
                .iter()
                .map(|l| l.chars().collect())
                .collect(),
            max_distance: 100000000,
            occ_rule: 5,
        }
    }

    pub fn next(&self) -> Seats
    {
        let mut vals = self.vals.clone();

        for y in 0..vals.len()
        {
            for x in 0..vals[0].len()
            {
                let cur = self.vals[y][x];
                if cur != '.'
                {
                    let mut neib = Vec::<char>::new();
                    neib.push(self.get_range(x, -1, y, -1));
                    neib.push(self.get_range(x, 0, y, -1));
                    neib.push(self.get_range(x, 1, y, -1));
                    neib.push(self.get_range(x, 1, y, 0));
                    neib.push(self.get_range(x, 1, y, 1));
                    neib.push(self.get_range(x, 0, y, 1));
                    neib.push(self.get_range(x, -1, y, 1));
                    neib.push(self.get_range(x, -1, y, 0));

                    let occ = neib.iter().filter(|&c| *c == '#').count();

                    let mut next = cur;

                    if cur == 'L' && occ == 0
                    {
                        next = '#';
                    }
                    else if cur == '#' && occ >= self.occ_rule
                    {
                        next = 'L';
                    }

                    //println!("cur={}, occ={}, next={}", cur, occ, next);

                    vals[y][x] = next;
                }
            }
        }

        Seats
        { 
            vals,
            max_distance: self.max_distance,
            occ_rule: self.occ_rule,
         }
    }

    pub fn count_occupied(&self) -> usize
    {
        self.vals.iter()
            .map(|v| v.iter().map(|c| if *c == '#' { 1 } else { 0 }).sum::<usize>())
            .sum::<usize>()
    }

    pub fn get(&self, x: usize, y: usize) -> char
    {
        if y < self.vals.len()
        {
            if x < self.vals[y].len()
            {
                return self.vals[y][x];
            }
        }
        return '.';
    }

    pub fn get_range(&self, x: usize, x_diff: isize, y: usize, y_diff: isize) -> char
    {
        let max_x = self.vals[0].len() as isize;
        let max_y = self.vals.len() as isize;

        let mut x = x as isize;
        let mut y = y as isize;

        for _ in 0..self.max_distance
        {
            x += x_diff;
            y += y_diff;

            if x < 0 || x >= max_x || y < 0 || y >= max_y
            {
                return '.';
            }

            let ch = self.get(x as usize, y as usize);

            if ch != '.'
            {
                return ch;
            }
        }
        return '.';
    }
}

fn part_1(input: &str) -> usize
{
    let mut seen = HashSet::new();
    let mut cur = Seats::new_1(input);

    loop
    {
        //println!("{:?}", cur);

        if !seen.insert(cur.clone())
        {
            // Seen before
            return cur.count_occupied();
        }
        cur = cur.next();
    }
}

fn part_2(input: &str) -> usize
{
    let mut seen = HashSet::new();
    let mut cur = Seats::new_2(input);

    loop
    {
        //println!("{:?}", cur);

        if !seen.insert(cur.clone())
        {
            // Seen before
            return cur.count_occupied();
        }
        cur = cur.next();
    }
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(11)
        .example(|| Answer { calculated: part_1(EXAMPLE), expected: 37, })
        .part_1(|| Answer { calculated: part_1(INPUT), expected: 2489, })
        .example(|| Answer { calculated: part_2(EXAMPLE), expected: 26, })
        .part_2(|| Answer { calculated: part_2(INPUT), expected: 2180, })
}
