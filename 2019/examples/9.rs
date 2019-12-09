const INPUT: &str = include_str!("input_9.txt");

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

async fn run_prog_future(prog: &'static str, inputs: Receiver<i64>, outputs: Sender<i64>)
{
    let mut comp = IntCode::new(prog, inputs, outputs);
    comp.run().await;
}

fn run_prog(prog: &'static str, inputs: Vec<i64>) -> Vec<i64>
{
    let mut pool = LocalPool::new();
    let spawner = pool.spawner();

    let (itx, irx) = channel("inputs".to_owned());
    let (otx, orx) = channel("outputs".to_owned());

    for input in inputs
    {
        itx.send(input);
    }

    spawner.spawn_local(run_prog_future(prog, irx, otx)).unwrap();

    pool.run();

    orx.remainder()
}

fn part_1() -> i64
{
    let outputs = run_prog(INPUT, vec!(1));
    assert_eq!(outputs.len(), 1);
    outputs[0]
}

fn part_2() -> i64
{
    let outputs = run_prog(INPUT, vec!(2));
    assert_eq!(outputs.len(), 1);
    outputs[0]
}

fn main()
{
    assert_eq!(run_prog(
        "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99\n",
        vec!()),
        vec!(109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99));

    assert_eq!(run_prog(
        "1102,34915192,34915192,7,4,7,99,0\n",
        vec!()),
        vec!(1219070632396864));

    assert_eq!(run_prog(
        "104,1125899906842624,99\n",
        vec!()),
        vec!(1125899906842624));

    let answer_1 = part_1();
    println!("Answer #1={}", answer_1);
    assert_eq!(answer_1, 3518157894);

    let answer_2 = part_2();
    println!("Answer #2={}", answer_2);
    assert_eq!(answer_2, 80379);
}