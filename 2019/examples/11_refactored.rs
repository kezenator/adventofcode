use aoc2019::*;

use futures::executor::LocalPool;
use futures::task::LocalSpawnExt;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input_11.txt");

async fn async_robot(inputs: Receiver<i64>, outputs: Sender<i64>)
{
    async_int_code(INPUT, inputs, outputs.clone()).await;

    // Send special color 2 to terminate the camera
    outputs.send(2);
}

async fn async_camera(origin_white: bool, inputs: Sender<i64>, outputs: Receiver<i64>, paints: Sender<PaintPoint>)
{
    let mut painted: HashMap<Point, i64> = HashMap::new();
    let mut pos = Point::new(0, 0);
    let mut dir = Point::new(0, -1);

    // Setup and input the color of the origin

    if origin_white
    {
        painted.insert(Point::new(0, 0), 1);
        inputs.send(1);
    }
    else
    {
        inputs.send(0);
    }

    loop
    {
        // Get the paint (or terminate) and turn outputs

        let new_col = outputs.clone().await;
        if new_col == 2
        {
            return;
        }
        let turn = outputs.clone().await;

        // Paint and turn

        paints.send(PaintPoint::new(pos, Some(if new_col == 0 { ' ' } else { '*' })));
        painted.insert(pos, new_col);

        if turn == 0
        {
            // Turn left 90 degress
            dir = dir.rotate_90_anticlockwise();
        }
        else
        {
            // Turn right 90 degress
            dir = dir.rotate_90_clockwise();
        }

        // Move foward 1

        pos = Point::new(pos.x + dir.x, pos.y + dir.y);

        // Input color of the new location

        match painted.get(&pos)
        {
            None => inputs.send(0), // Unpainted - still black
            Some(col) => inputs.send(*col),
        };
    }
}

fn test_camera(inputs: Vec<i64>, outputs: Vec<i64>, num_paints: usize)
{
    let (inputs_tx, inputs_rx) = channel::<i64>("inputs".to_owned(), false);
    let (outputs_tx, outputs_rx) = channel::<i64>("outputs".to_owned(), false);
    let (paints_tx, paints_rx) = channel::<PaintPoint>("paints".to_owned(), false);

    let mut pool = LocalPool::new();
    let spawner = pool.spawner();

    for i in outputs
    {
        outputs_tx.send(i);
    }

    spawner.spawn_local(async_camera(false, inputs_tx, outputs_rx, paints_tx)).unwrap();

    pool.run();

    assert_eq!(inputs_rx.remainder(), inputs);

    let mut points = HashSet::new();
    for paint in paints_rx.remainder().drain(..)
    {
        points.insert(paint.point);
    }
    assert_eq!(points.len(), num_paints);
}

fn run_robot(origin_white: bool) -> Vec<PaintPoint>
{
    let (inputs_tx, inputs_rx) = channel::<i64>("inputs".to_owned(), false);
    let (outputs_tx, outputs_rx) = channel::<i64>("outputs".to_owned(), false);
    let (paints_tx, paints_rx) = channel::<PaintPoint>("paints".to_owned(), false);

    let mut pool = LocalPool::new();
    let spawner = pool.spawner();

    spawner.spawn_local(async_robot(inputs_rx, outputs_tx)).unwrap();
    spawner.spawn_local(async_camera(origin_white, inputs_tx, outputs_rx, paints_tx)).unwrap();

    pool.run();

    paints_rx.remainder()
}

fn part_1() -> usize
{
    let mut unique_points = HashSet::new();
    for paint in run_robot(false).drain(..)
    {
        unique_points.insert(paint.point);
    }
    unique_points.len()
}

fn part_2() -> String
{
    render(&run_robot(true))
}

fn main()
{
    test_camera(
        vec![0, 0, 0, 0, 1, 0, 0, 0],
        vec![1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 2],
        6);

    let answer_1 = part_1();
    println!("Answer #1={}", answer_1);
    assert_eq!(answer_1, 2184);

    let answer_2 = part_2();
    println!("Answer #2:\n{}", answer_2);
    //answer_2 prints: AHCHZEPK
}