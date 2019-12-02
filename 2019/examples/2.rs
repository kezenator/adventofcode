const INPUT: &str = include_str!("input_2.txt");

fn numbers(input: &str) -> Vec<usize>
{
    input
        .split("\n")
        .filter(|a| !a.is_empty())
        .nth(0).unwrap()
        .split(",")
        .map(|a| a.parse::<usize>().unwrap())
        .collect()
}

enum StepResult
{
    Halt,
    StepIp(usize),
}

struct IntCode
{
    memory: Vec<usize>,
    ip: usize,
}

impl IntCode
{
    fn new(input: &str) -> Self
    {
        IntCode
        {
            memory: numbers(input),
            ip: 0,
        }
    }

    fn test_string(&self) -> String
    {
        self.memory
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn step(&mut self) -> StepResult
    {
        let opcode = self.memory[self.ip];

        match opcode
        {
            1 =>
            {
                // Add
                let new_val = self.memory[self.memory[self.ip + 1]]
                    + self.memory[self.memory[self.ip + 2]];
                let output_idx = self.memory[self.ip + 3];
                self.memory[output_idx] = new_val;
                return StepResult::StepIp(4);
            },
            2 =>
            {
                // Multiply
                let new_val = self.memory[self.memory[self.ip + 1]]
                    * self.memory[self.memory[self.ip + 2]];
                let output_idx = self.memory[self.ip + 3];
                self.memory[output_idx] = new_val;
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
}

fn example(input: &str) -> String
{
    let mut comp = IntCode::new(input);
    comp.run();
    return comp.test_string();
}

fn run(noun: usize, verb: usize) -> usize
{
    let mut comp = IntCode::new(INPUT);
    comp.memory[1] = noun;
    comp.memory[2] = verb;
    comp.run();
    return comp.memory[0];
}

fn part_1() -> usize
{
    run(12, 2)
}

fn part_2() -> usize
{
    for noun in 0..100
    {
        for verb in 0..100
        {
            if run(noun, verb) == 19690720
            {
                return 100 * noun + verb;
            }
        }
    }
    assert!(false);
    unreachable!();
}

fn main()
{
    assert_eq!(example("1,0,0,0,99"), "2,0,0,0,99");
    assert_eq!(example("2,3,0,3,99"), "2,3,0,6,99");
    assert_eq!(example("2,4,4,5,99,0"), "2,4,4,5,99,9801");
    assert_eq!(example("1,1,1,4,99,5,6,0,99"), "30,1,1,4,2,5,6,0,99");
    
    let answer_1 = part_1();
    println!("Answer #1={}", answer_1);
    assert_eq!(answer_1, 4930687);

    let answer_2 = part_2();
    println!("Answer #2={}", answer_2);
    assert_eq!(answer_2, 5335);
}