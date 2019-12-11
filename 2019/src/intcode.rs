use super::chan::{channel, Sender, Receiver};

use futures::executor::LocalPool;
use futures::task::LocalSpawnExt;

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

pub async fn async_int_code(prog: &'static str, inputs: Receiver<i64>, outputs: Sender<i64>)
{
    let mut comp = IntCode::new(prog, inputs, outputs.clone());
    comp.run().await;
}

pub fn run_int_code(prog: &'static str, inputs: Vec<i64>) -> Vec<i64>
{
    let mut pool = LocalPool::new();
    let spawner = pool.spawner();

    let (itx, irx) = channel("inputs".to_owned(), false);
    let (otx, orx) = channel("outputs".to_owned(), false);

    for input in inputs
    {
        itx.send(input);
    }

    spawner.spawn_local(async_int_code(prog, irx, otx)).unwrap();

    pool.run();

    orx.remainder()
}
