use aoc2019::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec3
{
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Vec3
{
    pub fn new(x: i64, y: i64, z: i64) -> Self
    {
        Vec3 { x, y, z }
    }

    pub fn parse(input: &str) -> Self
    {
        let csv_str = input.chars()
            .filter(|c| (*c == ',') || (*c == '-') || ((*c >= '0') && (*c <= '9')))
            .collect::<String>();
        
        let parts = csv_str
            .split(",")
            .collect::<Vec<_>>();

        assert_eq!(parts.len(), 3);

        Vec3::new(
            parts[0].parse().unwrap(),
            parts[1].parse().unwrap(),
            parts[2].parse().unwrap())
    }

    pub fn sum(&self, other: &Vec3) -> Self
    {
        Vec3 { 
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z }
    }

    pub fn manhatten_mag(&self) -> i64
    {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Moon
{
    pos: Vec3,
    vel: Vec3,
}

impl Moon
{
    pub fn new(pos: Vec3) -> Self
    {
        Moon { pos, vel: Vec3::new(0, 0, 0) }
    }

    pub fn energy(&self) -> i64
    {
        self.pos.manhatten_mag() * self.vel.manhatten_mag()
    }

    pub fn update_velocity_against(&mut self, other: &Moon)
    {
        self.vel.x += (other.pos.x - self.pos.x).signum();
        self.vel.y += (other.pos.y - self.pos.y).signum();
        self.vel.z += (other.pos.z - self.pos.z).signum();
    }

    pub fn step(&mut self)
    {
        self.pos = self.pos.sum(&self.vel);
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
        let moons = input
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|l| Moon::new(Vec3::parse(l)))
            .collect::<Vec<_>>();

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
            .fold(0, |a, b| a + b)
    }
}

fn part_1(input: &str, steps: usize) -> i64
{
    let mut system = System::new(input);
    system.steps(steps);
    system.energy()
}

fn count_until_repeat<S, V, F1, F2>(state: S, step: F1, val: F2) -> usize
    where V: Eq + std::hash::Hash,
        F1: Fn(&mut S),
        F2: Fn(&S) -> V
{
    let mut state = state;
    
    let mut seen = HashSet::new();
    seen.insert(val(&state));

    let mut count = 0;

    loop
    {
        step(&mut state);
        count += 1;

        let new = val(&state);
        
        if seen.contains(&new)
        {
            return count;
        }
        seen.insert(new);
    }
}

fn part_2(input: &str) -> usize
{
    let get_x = |system: &System|
    {
        system.moons.iter().map(|m| (m.pos.x, m.vel.x)).collect::<Vec<_>>()
    };

    let get_y = |system: &System|
    {
        system.moons.iter().map(|m| (m.pos.y, m.vel.y)).collect::<Vec<_>>()
    };

    let get_z = |system: &System|
    {
        system.moons.iter().map(|m| (m.pos.z, m.vel.z)).collect::<Vec<_>>()
    };

    let x = count_until_repeat(
        System::new(input),
        |system| system.one_step(),
        |system| get_x(system));
    //println!("x={}", x);

    let y = count_until_repeat(
        System::new(input),
        |system| system.one_step(),
        |system| get_y(system));
    //println!("y={}", y);

    let z = count_until_repeat(
        System::new(input),
        |system| system.one_step(),
        |system| get_z(system));
    //println!("z={}", z);

    lcm(lcm(x as i64, y as i64), z as i64) as usize
}

fn main()
{
    const INPUT: &str = include_str!("input_12.txt");

    const EXAMPLE_1: &str = "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>\n";
    const EXAMPLE_2: &str = "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>\n";

    assert_eq!(part_1(EXAMPLE_1, 10), 179);
    assert_eq!(part_1(EXAMPLE_2, 100), 1940);

    let answer_1 = part_1(INPUT, 1000);
    println!("Answer #1={}", answer_1);
    assert_eq!(answer_1, 10944);

    assert_eq!(part_2(EXAMPLE_1), 2772);
    assert_eq!(part_2(EXAMPLE_2), 4686774924);

    let answer_2 = part_2(INPUT);
    println!("Answer #2={}", answer_2);
    assert_eq!(answer_2, 484244804958744);
}