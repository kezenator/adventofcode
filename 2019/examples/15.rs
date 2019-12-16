use aoc2019::*;
use std::collections::{HashMap, HashSet, VecDeque};
use futures::executor::LocalPool;
use futures::task::LocalSpawnExt;

fn dir_to_input(dir: &Point) -> i64
{
    assert_eq!(dir.manhatten_dist_to(&Point::new(0, 0)), 1);

    if dir.y == 0
    {
        if dir.x < 0
        {
            3
        }
        else
        {
            4
        }
    }
    else if dir.y < 0
    {
        1
    }
    else
    {
        2
    }
}

fn successors_path(map: &HashMap<Point, char>, p: &Point) -> Vec<(Point, i64)>
{
    vec![
        Point::new(p.x + 1, p.y),
        Point::new(p.x - 1, p.y),
        Point::new(p.x, p.y + 1),
        Point::new(p.x, p.y - 1),
    ]
    .drain(..)
    .filter(|tp|
    {
        match map.get(tp)
        {
            Some('.') => true,
            Some('X') => true,
            _ => false,
        }
    })
    .map(|tp| (tp, 1 as i64))
    .collect()
}

fn successors_no_oxy(map: &HashMap<Point, char>, p: &Point) -> Vec<Point>
{
    vec![
        Point::new(p.x + 1, p.y),
        Point::new(p.x - 1, p.y),
        Point::new(p.x, p.y + 1),
        Point::new(p.x, p.y - 1),
    ]
    .drain(..)
    .filter(|tp|
    {
        match map.get(tp)
        {
            Some('.') => true,
            _ => false,
        }
    })
    .collect()
}

fn shortest_path(map: &HashMap<Point, char>, from: &Point, to: &Point) -> Vec<Point>
{
    match pathfinding::directed::dijkstra::dijkstra(
        from,
        |p| successors_path(map, p),
        |p| *p == *to)
    {
        Some((path, _cost)) => path.iter().map(|p| (*p).clone()).collect::<Vec<Point>>(),
        None => { assert!(false); unreachable!(); }
    }
}

#[allow(unused)]
fn draw(map: &HashMap<Point, char>, pos: &Point)
{
    let mut points = map.iter().map(|(p, ch)| PaintPoint::new(p.clone(), Some(*ch))).collect::<Vec<PaintPoint>>();
    points.push(PaintPoint::new(Point::new(0, 0), Some('+')));
    points.push(PaintPoint::new(pos.clone(), Some('D')));
    println!("=============");
    println!("{}", render(&points));
}

pub async fn async_remote(inputs: Sender<i64>, outputs: Receiver<i64>, map_output: Sender<HashMap<Point, char>>)
{
    let mut map = HashMap::new();
    let mut to_explore = VecDeque::new();
    let mut marked_to_explore = HashSet::new();
    let mut pos = Point::new(0, 0);
    to_explore.push_back((Point::new(0, 0), Point::new(1, 0)));
    to_explore.push_back((Point::new(0, 0), Point::new(-1, 0)));
    to_explore.push_back((Point::new(0, 0), Point::new(0, 1)));
    to_explore.push_back((Point::new(0, 0), Point::new(0, -1)));

    marked_to_explore.insert(Point::new(0, 0));
    for p in to_explore.iter()
    {
        marked_to_explore.insert(p.0.clone());
    }

    map.insert(Point::new(0, 0), '.');

    while !to_explore.is_empty()
    {
        //draw(&map, &pos);

        let (exp_from, exp_to) = to_explore.front().unwrap().clone();

        //println!("Pos = {:?}, ToExplore = {:?} => {:?}", pos, exp_from, exp_to);

        let mut next = exp_to.clone();

        if pos != exp_from
        {
            next = shortest_path(&map, &pos, &exp_from).iter().nth(1).unwrap().clone();
            assert_eq!(map.get(&next), Some(&'.'));
        }
        else
        {
            to_explore.pop_front().unwrap();
        }


        let dir = next.subtract(&pos);

        //println!("Pos = {:?}, Next = {:?}, Dir = {:?}", pos, next, dir);

        assert_eq!(dir.manhatten_dist_to(&Point::new(0, 0)), 1);
        inputs.send(dir_to_input(&dir));

        let output = outputs.clone().await;

        match output
        {
            0 =>
            {
                // Wall
                map.insert(next.clone(), '#');
            },
            1 =>
            {
                // Moved
                map.insert(next.clone(), '.');
                pos = next;

                let surrounds = vec![
                    Point::new(pos.x + 1, pos.y),
                    Point::new(pos.x - 1, pos.y),
                    Point::new(pos.x, pos.y + 1),
                    Point::new(pos.x, pos.y - 1),
                ];

                for s in surrounds
                {
                    if !map.contains_key(&s) && !marked_to_explore.contains(&s)
                    {
                        to_explore.push_front((pos.clone(), s.clone()));
                    }
                }
            },
            2 =>
            {
                // Moved to dest
                map.insert(next.clone(), 'X');
                pos = next;

                let surrounds = vec![
                    Point::new(pos.x + 1, pos.y),
                    Point::new(pos.x - 1, pos.y),
                    Point::new(pos.x, pos.y + 1),
                    Point::new(pos.x, pos.y - 1),
                ];

                for s in surrounds
                {
                    if !map.contains_key(&s) && !marked_to_explore.contains(&s)
                    {
                        to_explore.push_front((pos.clone(), s.clone()));
                    }
                }
            },
            _ => { assert!(false); unreachable!(); }
        }
    }

    // Explored the entire map
    map_output.send(map);
}

pub fn run_remote(prog: &'static str) -> HashMap<Point, char>
{
    let mut pool = LocalPool::new();
    let spawner = pool.spawner();

    let (itx, irx) = channel("inputs".to_owned(), false);
    let (otx, orx) = channel("outputs".to_owned(), false);
    let (mtx, mrx) = channel("map".to_owned(), false);

    spawner.spawn_local(async_int_code(prog, irx, otx)).unwrap();

    pool.run_until(async_remote(itx, orx, mtx));

    mrx.remainder().last().unwrap().clone()
}

fn part_1(input: &'static str) -> usize
{
    let map = run_remote(input);

    // The oxygen system is where there's a X

    let oxy_location = map.iter().filter(|(_p, ch)| **ch == 'X').map(|(p, _ch)| p.clone()).nth(0).unwrap();

    // The shortest path includes the start and end point -
    // so we need to subtract one as we're already at the start

    shortest_path(&map, &Point::new(0, 0), &oxy_location).len() - 1
}

fn part_2(input: &'static str) -> i64
{
    let mut map = run_remote(input);

    let oxy_location = map.iter().filter(|(_p, ch)| **ch == 'X').map(|(p, _ch)| p.clone()).nth(0).unwrap();

    let mut minutes = 0;

    let mut to_check = successors_no_oxy(&map, &oxy_location);

    while !to_check.is_empty()
    {
        minutes += 1;

        let check_this_time = to_check.drain(..).collect::<Vec<Point>>();
        to_check = Vec::new();

        for p in check_this_time
        {
            if *map.get(&p).unwrap() == '.'
            {
                map.insert(p.clone(), 'X');
                for s in successors_no_oxy(&map, &p)
                {
                    to_check.push(s.clone());
                }
            }
        }

        //draw(&map, &Point::new(0, 0));
        //println!("Minutes = {}", minutes);
    }

    minutes
}

fn main()
{
    const INPUT: &str = include_str!("input_15.txt");

    let answer_1 = part_1(INPUT);
    println!("Answer #1={}", answer_1);
    assert_eq!(answer_1, 354);

    let answer_2 = part_2(INPUT);
    println!("Answer #2={}", answer_2);
    assert_eq!(answer_2, 370);
}