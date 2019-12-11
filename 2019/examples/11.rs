mod mod_ch
{
    use std::fmt::Debug;
    use std::collections::VecDeque;
    use std::task::{Context, Poll, Waker};
    use std::sync::{Arc, Mutex};
    use std::future::Future;
    use std::pin::Pin;

    struct Inner<T>
        where T: Debug
    {
        #[allow(unused)]
        name: String,
        queue: VecDeque<T>,
        tasks: Vec<Waker>,
    }

    impl<T> Inner<T>
        where T: Debug
    {
        fn new(name: String) -> Self
        {
            Inner{ name, queue: VecDeque::new(), tasks: Vec::new(), }
        }
    }

    #[derive(Clone)]
    pub struct Sender<T>
        where T: Debug
    {
        inner: Arc<Mutex<Inner<T>>>,
    }

    impl<T> Sender<T>
        where T: Debug
    {
        pub fn send(&self, val: T)
        {
            let mut inner = self.inner.lock().unwrap();
            //println!("{}: TX: {:?}", inner.name, val);
            inner.queue.push_back(val);
            for task in inner.tasks.iter()
            {
                //println!("{}: W+", inner.name);
                task.clone().wake();
            }
            inner.tasks.truncate(0);
        }
    }

    #[derive(Clone)]
    pub struct Receiver<T>
        where T: Debug
    {
        inner: Arc<Mutex<Inner<T>>>,
    }

    impl<T> Receiver<T>
        where T: Debug
    {
        pub fn remainder(&self) -> Vec<T>
        {
            let mut inner = self.inner.lock().unwrap();
            let len = inner.queue.len();
            inner.queue.drain(0..len).collect()
        }
    }

    impl<T> Future for Receiver<T>
        where T: Debug
    {
        type Output = T;

        fn poll(self: Pin<&mut Self>, cx: &mut Context)-> Poll<Self::Output>
        {
            let mut inner = self.inner.lock().unwrap();
            if inner.queue.is_empty()
            {
                //println!("{}: W-", inner.name);
                inner.tasks.push(cx.waker().clone());
                return Poll::Pending;
            }
            let val = inner.queue.pop_front().unwrap();
            //println!("{}: RX: {:?}", inner.name, val);
            return Poll::Ready(val);
        }
    }

    pub fn channel<T>(name: String) -> (Sender<T>, Receiver<T>)
        where T: Debug
    {
        let inner = Arc::new(Mutex::new(Inner::new(name)));
        let sender = Sender{ inner: inner.clone() };
        let receiver = Receiver{ inner: inner.clone() };
        return (sender, receiver);
    }
}

mod int_code
{
    use super::mod_ch::{Sender, Receiver};

    enum StepResult
    {
        Halt,
        StepIp(usize),
    }

    pub struct IntCode
    {
        memory: Vec<i64>,
        ip: usize,
        relative_base: i64,
        inputs: Receiver<i64>,
        outputs: Sender<i64>,
    }

    impl IntCode
    {
        pub fn new(input: &str, inputs: Receiver<i64>, outputs: Sender<i64>) -> Self
        {
            let memory = input
                .split("\n")
                .filter(|a| !a.is_empty())
                .nth(0).unwrap()
                .split(",")
                .map(|a| a.parse::<i64>().unwrap())
                .collect();

            IntCode
            {
                memory,
                ip: 0,
                relative_base: 0,
                inputs,
                outputs,
            }
        }

        async fn step(&mut self) -> StepResult
        {
            let opcode = self.memory[self.ip] % 100;

            match opcode
            {
                1 =>
                {
                    // Add
                    let a = self.read(1);
                    let b = self.read(2);
                    self.write(3, a + b);
                    return StepResult::StepIp(4);
                },
                2 =>
                {
                    // Multiply
                    let a = self.read(1);
                    let b = self.read(2);
                    self.write(3, a * b);
                    return StepResult::StepIp(4);
                },
                3 =>
                {
                    // Input
                    let val = self.inputs.clone().await;
                    self.write(1, val);
                    return StepResult::StepIp(2);
                },
                4 =>
                {
                    // Output
                    let val = self.read(1);
                    self.outputs.send(val);
                    return StepResult::StepIp(2);
                },
                5 =>
                {
                    // Jump If True
                    let test = self.read(1);
                    let dest = self.read(2);

                    if test != 0
                    {
                        self.ip = dest as usize;
                        return StepResult::StepIp(0);
                    }
                    else
                    {
                        return StepResult::StepIp(3);
                    }
                },
                6 =>
                {
                    // Jump If False
                    let test = self.read(1);
                    let dest = self.read(2);

                    if test == 0
                    {
                        self.ip = dest as usize;
                        return StepResult::StepIp(0);
                    }
                    else
                    {
                        return StepResult::StepIp(3);
                    }
                },
                7 =>
                {
                    // Less Than
                    let a = self.read(1);
                    let b = self.read(2);

                    self.write(3, if a < b { 1 } else { 0 });
                    return StepResult::StepIp(4);
                },
                8 =>
                {
                    // Equal
                    let a = self.read(1);
                    let b = self.read(2);

                    self.write(3, if a == b { 1 } else { 0 });
                    return StepResult::StepIp(4);
                },
                9 =>
                {
                    // Adjust Relative Base
                    let a = self.read(1);

                    self.relative_base += a;

                    return StepResult::StepIp(2);
                },
                99 =>
                {
                    return StepResult::Halt;
                },
                _ =>
                {
                    assert!(false);
                    unreachable!();
                }
            }
        }

        pub async fn run(&mut self)
        {
            loop
            {
                match self.step().await
                {
                    StepResult::Halt => { return; },
                    StepResult::StepIp(dist) => { self.ip += dist; },
                }
            }
        }

        fn read(&mut self, offset: usize) ->i64
        {
            let mut factor = 10;
            for _ in 0..offset
            {
                factor *= 10;
            }
            let opcode = self.memory[self.ip];
            let mode = (opcode / factor) % 10;

            let contents = self.memory[self.ip + offset];

            match mode
            {
                0 => self.read_index(contents as usize),
                1 => contents,
                2 => self.read_index((self.relative_base + contents) as usize),
                _ => { assert!(false); unreachable!(); },
            }
        }

        fn read_index(&mut self, index: usize) -> i64
        {
            if index > self.memory.len()
            {
                0
            }
            else
            {
                self.memory[index]
            }
        }

        fn write(&mut self, offset: usize, value: i64)
        {
            let mut factor = 10;
            for _ in 0..offset
            {
                factor *= 10;
            }
            let opcode = self.memory[self.ip];
            let mode = (opcode / factor) % 10;

            let contents = self.memory[self.ip + offset];

            match mode
            {
                0 => self.write_index(contents as usize, value),
                1 => { assert!(false); unreachable!(); },
                2 => self.write_index((self.relative_base + contents) as usize, value),
                _ => { assert!(false); unreachable!(); },
            }
        }

        fn write_index(&mut self, index: usize, value: i64)
        {
            if index >= self.memory.len()
            {
                self.memory.resize(index + 1, 0);
            }
            self.memory[index] = value;
        }
    }
}

use mod_ch::{channel, Sender, Receiver};
use int_code::IntCode;
use futures::executor::LocalPool;
use futures::task::LocalSpawnExt;
use aoc2019::*;
use std::collections::*;

#[derive(Debug)]
struct Paint(Point, i64);

async fn run_prog_future(prog: &'static str, inputs: Receiver<i64>, outputs: Sender<i64>)
{
    let mut comp = IntCode::new(prog, inputs, outputs.clone());
    comp.run().await;

    // Send new col 2 to terminate the camera
    outputs.send(2);
}

async fn camera_future(origin_white: bool, inputs: Sender<i64>, outputs: Receiver<i64>, paints: Sender<Paint>)
{
    let mut painted: HashMap<Point, i64> = HashMap::new();
    let mut pos = Point::new(0, 0);

    let dirs = vec![
        Point::new(0, -1),
        Point::new(1, 0),
        Point::new(0, 1),
        Point::new(-1, 0)
    ];
    let mut dir_index = 0;

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
        let new_col = outputs.clone().await;
        if new_col == 2
        {
            return;
        }
        let turn = outputs.clone().await;

        paints.send(Paint(pos, new_col));
        painted.insert(pos, new_col);

        if turn == 0
        {
            // Turn left 90 degress
            dir_index = (dir_index  + 3) % 4;
        }
        else
        {
            // Turn right 90 degress
            dir_index = (dir_index  + 1) % 4;
        }

        // Move foward 1

        pos = Point::new(pos.x + dirs[dir_index].x, pos.y + dirs[dir_index].y);

        // Input color of the new location
        match painted.get(&pos)
        {
            None => inputs.send(0), // Still black
            Some(col) => inputs.send(*col),
        };
    }
}

fn test_camera(inputs: Vec<i64>, outputs: Vec<i64>, num_paints: usize)
{
    let (inputs_tx, inputs_rx) = channel::<i64>("inputs".to_owned());
    let (outputs_tx, outputs_rx) = channel::<i64>("outputs".to_owned());
    let (paints_tx, paints_rx) = channel::<Paint>("paints".to_owned());

    let mut pool = LocalPool::new();
    let spawner = pool.spawner();

    for i in outputs
    {
        outputs_tx.send(i);
    }

    spawner.spawn_local(camera_future(false, inputs_tx, outputs_rx, paints_tx)).unwrap();

    pool.run();

    assert_eq!(inputs_rx.remainder(), inputs);

    let mut points = HashSet::new();
    for paint in paints_rx.remainder().drain(..)
    {
        points.insert(paint.0);
    }
    assert_eq!(points.len(), num_paints);
}

fn part_1(input: &'static str) -> usize
{
    let (inputs_tx, inputs_rx) = channel::<i64>("inputs".to_owned());
    let (outputs_tx, outputs_rx) = channel::<i64>("outputs".to_owned());
    let (paints_tx, paints_rx) = channel::<Paint>("paints".to_owned());

    let mut pool = LocalPool::new();
    let spawner = pool.spawner();

    spawner.spawn_local(run_prog_future(input, inputs_rx, outputs_tx)).unwrap();
    spawner.spawn_local(camera_future(false, inputs_tx, outputs_rx, paints_tx)).unwrap();

    pool.run();

    let mut unique_points = HashSet::new();
    for paint in paints_rx.remainder().drain(..)
    {
        unique_points.insert(paint.0);
    }
    unique_points.len()
}

fn part_2(input: &'static str) -> String
{
    let (inputs_tx, inputs_rx) = channel::<i64>("inputs".to_owned());
    let (outputs_tx, outputs_rx) = channel::<i64>("outputs".to_owned());
    let (paints_tx, paints_rx) = channel::<Paint>("paints".to_owned());

    let mut pool = LocalPool::new();
    let spawner = pool.spawner();

    spawner.spawn_local(run_prog_future(input, inputs_rx, outputs_tx)).unwrap();
    spawner.spawn_local(camera_future(true, inputs_tx, outputs_rx, paints_tx)).unwrap();

    pool.run();

    let mut result = vec![' '; 800];
    for i in 1..8
    {
        result[i * 100] = '\n';
    }

    for paint in paints_rx.remainder().drain(..)
    {
        if paint.0.x >= 0 && paint.0.y >= 0 && paint.0.x < 100 && paint.0.y < 10
        {
            let index = (paint.0.y * 100 + paint.0.x + paint.0.y.signum()) as usize;
            result[index] = if paint.1 == 0 { ' ' } else { '*' };
        }
        else
        {
            println!("Out of range point: {:?}", paint);
        }
    }
    result.drain(..).collect::<String>()
}

fn main()
{
    const INPUT: &str = include_str!("input_11.txt");

    test_camera(
        vec![0, 0, 0, 0, 1, 0, 0, 0],
        vec![1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 2],
        6);

    let answer_1 = part_1(INPUT);
    println!("Answer #1={}", answer_1);
    assert_eq!(answer_1, 2184);

    let answer_2 = part_2(INPUT);
    println!("Answer #2:\n{}", answer_2);
    //answer_2 prints: AHCHZEPK
}