use std::collections::VecDeque;

const INPUT: &str = include_str!("input_7.txt");

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

fn run_amps(input: &str, settings: &Vec<i64>) -> i64
{
    let mut result = 0;

    for setting in settings.iter()
    {
        let mut comp = IntCode::new(input, vec!(*setting, result));
        comp.run();
        result = comp.outputs[0];
    }

    result
}

fn perms(from: i64, to: i64) -> Vec<Vec<i64>>
{
    let mut result = Vec::new();

    for a in from..(to+1)
    {
        for b in from..(to+1)
        {
            if a != b
            {
                for c in from..(to+1)
                {
                    if a != c && b != c
                    {
                        for d in from..(to+1)
                        {
                            if a != d && b != d && c != d
                            {
                                for e in from..(to+1)
                                {
                                    if a != e && b != e && c != e && d != e
                                    {
                                        result.push(vec!(a, b, c, d, e));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    result
}

fn part_1() -> i64
{
    let mut values = Vec::new();

    for settings in perms(0, 4)
    {
        values.push(run_amps(INPUT, &settings));
    }

    values.sort();
    *values.last().unwrap()
}

fn main()
{
    assert_eq!(run_amps("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0\n", &vec!(4,3,2,1,0)), 43210);
    assert_eq!(run_amps("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0\n", &vec!(0,1,2,3,4)), 54321);
    assert_eq!(run_amps("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0\n", &vec!(1,0,4,3,2)), 65210);

    let answer_1 = part_1();
    println!("Answer #1={}", answer_1);
    assert_eq!(answer_1, 21760);
}