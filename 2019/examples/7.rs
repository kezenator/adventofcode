const INPUT: &str = include_str!("input_7.txt");

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
                0 => self.memory[contents as usize],
                1 => contents,
                _ => { assert!(false); unreachable!(); },
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

            assert_eq!(mode, 0);

            let contents = self.memory[self.ip + offset];
            self.memory[contents as usize] = value;
        }
    }
}

mod perms
{
    use std::iter::Iterator;

    pub struct PermIterator<T>
        where T: Clone
    {
        items: Vec<T>,
        state: Vec<usize>,
    }

    impl<T> PermIterator<T>
        where T: Clone
    {
        pub fn new(items: Vec<T>) -> Self
        {
            let len = items.len();
            PermIterator{ items, state: vec![0; len]}
        }

        fn increment(&mut self) -> bool
        {
            let mut index = self.state.len() - 1;
            loop
            {
                self.state[index] += 1;
                if self.state[index] >= self.state.len()
                {
                    if index == 0
                    {
                        return false;
                    }
                    self.state[index] = 0;
                    index -= 1;
                }
                else
                {
                    return true;
                }
            }
        }

        fn indexes_unique(&mut self) -> bool
        {
            for i in self.state.iter()
            {
                if self.state.iter().filter(|&a| *a == *i).count() != 1
                {
                    return false;
                }
            }
            true
        }
    }

    impl<T> Iterator for PermIterator<T>
        where T: Clone
    {
        type Item = Vec<T>;

        fn next(&mut self) -> Option<Vec<T>>
        {
            loop
            {
                if !self.increment()
                {
                    return None;
                }

                if self.indexes_unique()
                {
                    return Some(self.state.iter()
                        .map(|a| self.items[*a].clone())
                        .collect());
                }
            }
        }
    }

    pub fn permutations<T>(items: Vec<T>) -> PermIterator<T>
        where T: Clone
    {
        PermIterator::new(items)
    }
}

use mod_ch::{channel, Sender, Receiver};
use perms::permutations;
use int_code::IntCode;
use futures::executor::LocalPool;
use futures::task::LocalSpawnExt;

async fn run_single_amp(input: &'static str, tx: Sender<i64>, rx: Receiver<i64>) -> ()
{
    let mut comp = IntCode::new(input, rx, tx);
    comp.run().await
}

fn run_amps(input: &'static str, settings: &Vec<i64>) -> i64
{
    let num_amps = settings.len();

    let channels = (0..num_amps).map(|i| channel::<i64>(i.to_string())).collect::<Vec<_>>();

    // Input the settings

    for i in 0..num_amps
    {
        channels[i].0.send(settings[i]);
    }

    // Input the first zero into the first amp

    channels[0].0.send(0);

    // Run each amp

    let mut pool = LocalPool::new();
    let spawner = pool.spawner();

    for i in 0..num_amps
    {
        let tx = channels[(i + 1) % num_amps].0.clone();
        let rx = channels[i].1.clone();
        spawner.spawn_local(run_single_amp(input, tx, rx)).unwrap();
    }

    pool.run();

    // The final output is the remainder that has
    // been sent back to the first amp
    
    let output_vec = channels[0].1.remainder();
    assert_eq!(output_vec.len(), 1);
    return output_vec[0];
}

fn part_1() -> i64
{
    let mut values = Vec::new();

    for settings in permutations(vec![0, 1, 2, 3, 4])
    {
        values.push(run_amps(INPUT, &settings));
    }

    values.sort();
    *values.last().unwrap()
}

fn part_2() -> i64
{
    let mut values = Vec::new();

    for settings in permutations(vec![5, 6, 7, 8, 9])
    {
        values.push(run_amps(INPUT, &settings));
    }

    values.sort();
    *values.last().unwrap()
}

fn main()
{
    assert_eq!(permutations(vec![0, 1, 2]).collect::<Vec<Vec<usize>>>(), vec![vec![0, 1, 2], vec![0, 2, 1], vec![1, 0, 2], vec![1, 2, 0], vec![2, 0, 1], vec![2, 1, 0]]);

    assert_eq!(run_amps("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0\n", &vec!(4,3,2,1,0)), 43210);
    assert_eq!(run_amps("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0\n", &vec!(0,1,2,3,4)), 54321);
    assert_eq!(run_amps("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0\n", &vec!(1,0,4,3,2)), 65210);

    let answer_1 = part_1();
    println!("Answer #1={}", answer_1);
    assert_eq!(answer_1, 21760);

    assert_eq!(run_amps("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5\n", &vec!(9,8,7,6,5)), 139629729);
    assert_eq!(run_amps("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10\n", &vec!(9,7,8,5,6)), 18216);

    let answer_2 = part_2();
    println!("Answer #2={}", answer_2);
    assert_eq!(answer_2, 69816958);
}