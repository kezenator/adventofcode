use std::collections::VecDeque;
use crate::support::{input_to_lines, scan};

#[derive(Debug, PartialEq, Eq)]
pub enum IntcodePause
{
    MoreInputRequired,
    Halted,
}

pub struct Intcode
{
    mem: Vec<i64>,
    pc: usize,
    rel_base: i64,
    input_buffer: VecDeque<i64>,
    outputs: VecDeque<i64>,
    halted: bool,
}

impl Intcode
{
    pub fn new_from_input(input: &str) -> Self
    {
        let line = input_to_lines(input)[0].clone();

        let (mem,) = scan(&line)
            .remaining().parse_vec::<i64>(",");

        Self::new(mem)
    }

    pub fn new(mem: Vec<i64>) -> Self
    {
        Intcode
        {
            mem: mem,
            pc: 0,
            rel_base: 0,
            input_buffer: VecDeque::new(),
            outputs: VecDeque::new(),
            halted: false,
        }
    }

    pub fn write_mem(&mut self, index: usize, value: i64)
    {
        while self.mem.len() <= index
        {
            self.mem.push(0);
        }

        self.mem[index] = value;
    }

    pub fn read_mem(&self, index: usize) -> i64
    {
        if index >= self.mem.len()
        {
            0
        }
        else
        {
            self.mem[index]
        }
    }

    pub fn get_mem(&self) -> Vec<i64>
    {
        self.mem.clone()
    }

    pub fn input(&mut self, input: i64)
    {
        self.input_buffer.push_back(input);
    }

    pub fn is_input_buffer_empty(&self) -> bool
    {
        self.input_buffer.is_empty()
    }

    pub fn output_len(&self) -> usize
    {
        self.outputs.len()
    }

    pub fn pop_output(&mut self) -> i64
    {
        self.outputs.pop_front().expect("No more Intcode outputs")
    }

    pub fn all_output(&mut self) -> Vec<i64>
    {
        self.outputs.drain(..).collect()
    }

    pub fn is_halted(&self) -> bool
    {
        self.halted
    }

    pub fn run_until_halt(&mut self)
    {
        let pause = self.run_until_halt_or_input_required();

        assert_eq!(pause, IntcodePause::Halted);
    }

    pub fn run_until_halt_or_input_required(&mut self) -> IntcodePause
    {
        if self.halted
        {
            return IntcodePause::Halted;
        }

        loop
        {
            let mut inst = self.read_mem(self.pc);

            match self.instruction_opcode(&mut inst)
            {
                1 =>
                {
                    let a = self.read_param(&mut inst, 1);
                    let b = self.read_param(&mut inst, 2);
                    let index_c = self.read_index(&mut inst, 3);

                    self.write_mem(index_c, a + b);

                    self.pc += 4;
                },
                2 =>
                {
                    let a = self.read_param(&mut inst, 1);
                    let b = self.read_param(&mut inst, 2);
                    let index_c = self.read_index(&mut inst, 3);

                    self.write_mem(index_c, a * b);

                    self.pc += 4;
                },
                3 =>
                {
                    if self.input_buffer.is_empty()
                    {
                        return IntcodePause::MoreInputRequired;
                    }

                    let val = self.input_buffer.pop_front().expect("No inputs remaining");
                    let index = self.read_index(&mut inst, 1);

                    self.write_mem(index, val);

                    self.pc += 2;
                },
                4 =>
                {
                    let val = self.read_param(&mut inst, 1);

                    self.outputs.push_back(val);

                    self.pc += 2;
                },
                5 =>
                {
                    let val = self.read_param(&mut inst, 1);
                    let new_pc = self.read_param(&mut inst, 2);

                    if val != 0
                    {
                        self.pc = new_pc as usize;
                    }
                    else
                    {
                        self.pc += 3;
                    }
                },
                6 =>
                {
                    let val = self.read_param(&mut inst, 1);
                    let new_pc = self.read_param(&mut inst, 2);

                    if val == 0
                    {
                        self.pc = new_pc as usize;
                    }
                    else
                    {
                        self.pc += 3;
                    }
                },
                7 =>
                {
                    let a = self.read_param(&mut inst, 1);
                    let b = self.read_param(&mut inst, 2);
                    let index_c = self.read_index(&mut inst, 3);

                    let result = if a < b { 1 } else { 0 };

                    self.write_mem(index_c, result);

                    self.pc += 4;
                },
                8 =>
                {
                    let a = self.read_param(&mut inst, 1);
                    let b = self.read_param(&mut inst, 2);
                    let index_c = self.read_index(&mut inst, 3);

                    let result = if a == b { 1 } else { 0 };

                    self.write_mem(index_c, result);

                    self.pc += 4;
                },
                9 =>
                {
                    self.rel_base += self.read_param(&mut inst, 1);

                    self.pc += 2;
                },
                99 =>
                {
                    self.halted = true;
                    return IntcodePause::Halted;
                },
                _ =>
                {
                    unreachable!();
                },
            }
        }
    }

    fn instruction_opcode(&self, inst: &mut i64) -> i64
    {
        let result = *inst % 100;
        *inst /= 100;
        result
    }

    fn read_param(&self, inst: &mut i64, offset: usize) -> i64
    {
        let mode = *inst % 10;
        *inst /= 10;

        let val = self.read_mem(self.pc + offset);

        match mode
        {
            0 => self.read_mem(val as usize),
            1 => val,
            2 => self.read_mem((val + self.rel_base) as usize),
            _ => unreachable!(),
        }
    }

    fn read_index(&self, inst: &mut i64, offset: usize) -> usize
    {
        let mode = *inst % 10;
        *inst /= 10;

        match mode
        {
            0 => self.read_mem(self.pc + offset) as usize,
            1 => unreachable!(),
            2 => (self.rel_base + self.read_mem(self.pc + offset)) as usize,
            _ => unreachable!(),
        }
    }
}