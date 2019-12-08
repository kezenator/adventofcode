use std::collections::VecDeque;

const INPUT: &str = include_str!("input_5.txt");

enum StepResult
{
    Halt,
    StepIp(usize),
}

struct IntCode
{
    memory: Vec<i64>,
    ip: usize,
    inputs: VecDeque<i64>,
    outputs: Vec<i64>,
}

impl IntCode
{
    fn new(input: &str, inputs: Vec<i64>) -> Self
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
            inputs: inputs.iter().map(|a| *a).collect(),
            outputs: Vec::new(),
        }
    }

    fn step(&mut self) -> StepResult
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
                let val = self.inputs.pop_front().unwrap();
                self.write(1, val);
                return StepResult::StepIp(2);
            },
            4 =>
            {
                // Output
                let val = self.read(1);
                self.outputs.push(val);
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

    fn run(&mut self)
    {
        loop
        {
            match self.step()
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

fn part_1() -> i64
{
    let mut comp = IntCode::new(INPUT, vec!(1));
    comp.run();
    *comp.outputs.last().unwrap()
}

fn part_2() -> i64
{
    let mut comp = IntCode::new(INPUT, vec!(5));
    comp.run();
    *comp.outputs.last().unwrap()
}

fn main()
{
    let answer_1 = part_1();
    println!("Answer #1={}", answer_1);
    assert_eq!(answer_1, 13787043);

    let answer_2 = part_2();
    println!("Answer #2={}", answer_2);
    assert_eq!(answer_2, 3892695);
}