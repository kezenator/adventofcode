use crate::support::*;
use std::collections::HashSet;

const EXAMPLE_1: &str = include_str!("example_1.txt");
const EXAMPLE_2: &str = include_str!("example_2.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Moon
{
    pos: Point3,
    vel: Point3,
}

impl Moon
{
    pub fn new(pos: Point3) -> Self
    {
        Moon { pos, vel: Point3::new(0, 0, 0) }
    }

    pub fn energy(&self) -> i64
    {
        self.pos.manhatten_size() * self.vel.manhatten_size()
    }

    pub fn update_velocity_against(&mut self, other: &Moon)
    {
        self.vel.x += (other.pos.x - self.pos.x).signum();
        self.vel.y += (other.pos.y - self.pos.y).signum();
        self.vel.z += (other.pos.z - self.pos.z).signum();
    }

    pub fn step(&mut self)
    {
        self.pos = self.pos + self.vel;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct System
{
    moons: Vec<Moon>,
}

impl System
{
    pub fn new(input: &str) -> Self
    {
        let lines = input_to_lines(input);

        let moons = lines.iter()
            .map(|s|
            {
                let (x, y, z) = scan(s)
                    .skip_str("<x=")
                    .until(", y=").parse::<i64>()
                    .until(", z=").parse::<i64>()
                    .until(">").parse::<i64>()
                    .remaining().ignore();

                Moon::new(Point3::new(x, y, z))
            })
            .collect();

        System { moons }
    }

    pub fn one_step(&mut self)
    {
        let len = self.moons.len();
        for i in 0..len
        {
            for j in i..len
            {
                if i != j
                {
                    let a = self.moons[i].clone();
                    let b = self.moons[j].clone();

                    self.moons[i].update_velocity_against(&b);
                    self.moons[j].update_velocity_against(&a);
                }
            }
        }

        for i in 0..len
        {
            self.moons[i].step();
        }
    }

    pub fn steps(&mut self, num: usize)
    {
        for _ in 0..num
        {
            //println!("{:?}", self.moons[0]);
            self.one_step();
        }

        //println!("{:?}", self.moons);
    }

    pub fn energy(&self) -> i64
    {
        self.moons.iter()
            .map(|m| m.energy())
            .sum()
    }
}

fn part_1(input: &str, steps: usize) -> i64
{
    let mut system = System::new(input);
    system.steps(steps);
    system.energy()
}

fn count_until_coordinate_repeats<F>(input: &str, get_coord: F) -> usize
    where F: Fn(&Point3) -> i64
{
    let system_to_state = |system: &System| -> Vec<(i64, i64)>
    {
        system.moons.iter()
            .map(|m| (get_coord(&m.pos), get_coord(&m.vel)))
            .collect()
    };

    let mut system = System::new(input);
    let mut count = 0;
    let mut seen = HashSet::new();

    loop
    {
        if !seen.insert(system_to_state(&system))
        {
            return count;
        }

        system.one_step();
        count += 1;
    }
}

fn part_2(input: &str) -> u64
{
    let x_repeat = count_until_coordinate_repeats(input, |p| p.x);
    let y_repeat = count_until_coordinate_repeats(input, |p| p.y);
    let z_repeat = count_until_coordinate_repeats(input, |p| p.z);

    lcm(lcm(x_repeat as u64, y_repeat as u64), z_repeat as u64)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(12)
        .example(|| Answer
        {
            calculated: part_1(EXAMPLE_1, 10),
            expected: 179,
        })
        .example(|| Answer
        {
            calculated: part_1(EXAMPLE_2, 100),
            expected: 1940,
        })
        .part_1(|| Answer
        {
            calculated: part_1(INPUT, 1000),
            expected: 10944,
        })
        .example(|| Answer
        {
            calculated: part_2(EXAMPLE_1),
            expected: 2772,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 484244804958744i64,
        })
}
