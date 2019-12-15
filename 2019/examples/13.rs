use aoc2019::*;
use std::collections::HashMap;
use futures::executor::LocalPool;
use futures::task::LocalSpawnExt;

fn part_1(input: &'static str) -> i64
{
    let outputs = run_int_code(input, vec![]);
    let len = outputs.len();
    let mut index = 0;
    let mut tiles = HashMap::new();
    loop
    {
        let point = Point::new(outputs[index], outputs[index + 1]);
        let tile = outputs[index + 2];

        tiles.insert(point, tile);

        index += 3;
        if index >= len
        {
            break;
        }
    }

    let mut count = 0;
    for (_, tile) in tiles.iter()
    {
        if *tile == 2
        {
            count += 1;
        }
    }
    return count;
}

pub async fn async_joystick(inputs: Sender<i64>, outputs: Receiver<i64>, scores: Sender<i64>)
{
    let mut score = 0;
    //let mut ball = Point::new(0, 0);
    let mut paddle = Point::new(0, 0);

    let mut points = HashMap::new();

    let paint = |p: &HashMap<Point, char>|
    {
        let mut paints = Vec::new();
        for (key, value) in p
        {
            paints.push(PaintPoint::new(*key, Some(*value)));
        }
        println!("{}", render(&paints));
    };

    // Keep drawing the ball

    let mut do_paint = true;

    loop
    {
        //inputs.send(-1);

        if do_paint && points.len() > 0
        {
            println!("Score={}", score);
            paint(&points);
            do_paint = false;
            //std::thread::sleep_ms(100);
        }
        //do_paint = true;

        let x = outputs.clone().await;
        let y = outputs.clone().await;
        let z = outputs.clone().await;

        if x == -1 && y == -1 && z == -1
        {
            println!("JOYSTICK TERMINATED!");
            break;
        }

        if x == -1 && y == 0
        {
            score = z;
            scores.send(z);
            do_paint = true;
        }
        else
        {
            let tile = match z
            {
                0 => ' ',
                1 => 'X',
                2 => '*',
                3 => '-',
                4 => '.',
                _ => '?',
            };

            if tile == '_'
            {
                do_paint = true;
            }
            else if tile == '.'
            {
                inputs.send((x - paddle.x).signum());
                //ball = Point::new(x, y);
            }
            else if tile == '-'
            {
                paddle = Point::new(x, y);
            }

            points.insert(Point::new(x, y), tile);
        }
    }

    paint(&points);
}

pub async fn async_coin_int_code(prog: &'static str, inputs: Receiver<i64>, outputs: Sender<i64>)
{
    let mut comp = IntCode::new(prog, inputs, outputs.clone());
    comp.memory[0] = 2;
    comp.run().await;

    // Send -1, -1, -1 to terminate
    outputs.send(-1);
    outputs.send(-1);
    outputs.send(-1);
}

pub fn run_coin_int_code(prog: &'static str) -> i64
{
    let mut pool = LocalPool::new();
    let spawner = pool.spawner();

    let (itx, irx) = channel("inputs".to_owned(), false);
    let (otx, orx) = channel("outputs".to_owned(), false);
    let (stx, srx) = channel("scores".to_owned(), false);

    spawner.spawn_local(async_coin_int_code(prog, irx, otx)).unwrap();
    spawner.spawn_local(async_joystick(itx, orx, stx)).unwrap();

    pool.run();

    *srx.remainder().last().unwrap()
}

fn part_2(input: &'static str) -> i64
{
    run_coin_int_code(input)
}

fn main()
{
    const INPUT: &str = include_str!("input_13.txt");

    let answer_1 = part_1(INPUT);
    println!("Answer #1={}", answer_1);
    assert_eq!(answer_1, 233);

    let answer_2 = part_2(INPUT);
    println!("Answer #2={}", answer_2);
    //assert_eq!(answer_2, 0);
}