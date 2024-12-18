use itertools::*;
use crate::support::*;
use std::collections::HashSet;

const EXAMPLE: &str = include_str!("example.txt");

struct Computer
{
    ic: i64,
    programme: Vec<i64>,
    registers: Vec<i64>,
    outputs: Vec<i64>,
}

impl Computer
{
    fn new(input: &str) -> Self
    {
        let groups = input_to_groups(input);

        let ic = 0;

        let programme = scan(&groups[1][0])
            .skip_str("Program: ")
            .remaining().parse_vec(",")
            .0;

        let mut registers = Vec::new();
        for i in 0..3
        {
            registers.push(scan(&groups[0][i])
                .skip_str("Register ")
                .skip(1)
                .skip_str(": ")
                .remaining().parse().0);
        }

        let outputs = Vec::new();

        Computer { ic, programme, registers, outputs }
    }

    fn run(&mut self)
    {
        while (self.ic as usize) < self.programme.len()
        {
            let instruction = self.programme[self.ic as usize];
            let operand = self.programme[self.ic as usize + 1];
            self.ic += 2;

            match instruction
            {
                0 => // adv
                {
                    self.registers[0] = self.registers[0] >> self.combo(operand);
                },
                1 => // bxl
                {
                    self.registers[1] = self.registers[1] ^ operand;
                },
                2 => // bst
                {
                    self.registers[1] = self.combo(operand) & 0x7;
                },
                3 => // jnz
                {
                    if self.registers[0] != 0
                    {
                        self.ic = operand;
                    }
                },
                4 => // bcx
                {
                    self.registers[1] = self.registers[1] ^ self.registers[2];
                },
                5 => // out
                {
                    self.outputs.push(self.combo(operand) & 0x7);
                },
                6 => // bdv
                {
                    self.registers[1] = self.registers[0] >> self.combo(operand);
                },
                7 => // cdv
                {
                    self.registers[2] = self.registers[0] >> self.combo(operand);
                },
                _ => unreachable!(),
            }
        }
    }

    fn combo(&self, operand: i64) -> i64
    {
        match operand
        {
            0 | 1 | 2 | 3 => operand,
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            7 => unreachable!(),
            _ => unreachable!(),
        }
    }

    fn output(&self) -> String
    {
        self.outputs.iter().cloned()
            .map(|o| o.to_string())
            .join(",")
    }
}

fn reverse(outputs: &Vec<i64>, oi: usize, a: i64, found_as: &mut HashSet<i64>)
{
    if oi == 0
    {
        found_as.insert(a);
        return;
    }
    let oi = oi - 1;
    let out = outputs[oi];

    for added_bits in 0..8
    {
        let prev_a = (a << 3) + added_bits;

        let b = (prev_a % 8) ^ 3;
        let c = prev_a >> b;
        let b = b ^ 5 ^ c;

        if (b & 7) == out
        {
            reverse(outputs, oi, prev_a, found_as);
        }
    }
}

fn part_1(input: &str) -> String
{
    let mut computer = Computer::new(input);
    computer.run();
    computer.output()
}

fn part_2(input: &str) -> i64
{
    // My input:
    //  0: 2,4,  BST: B = A % 8
    //  2: 1,3,  BXL: B = B ^ 3
    //  4: 7,5,  CDV: C = A / 2^B
    //  6: 0,3,  ADV: A = A / 2^3
    //  8: 1,5,  BXL: B = B ^ 5
    // 10: 4,1,  BCX: B = B ^ C
    // 12: 5,5,  OUT: <=  B & 7
    // 14: 3,0   JNZ: if A != 0 GOTO 0:
    //
    // Writing this a bit cleaner...
    //
    // A = input
    // B = 0
    // C = 0
    // while (A != 0)
    // {
    //    B = (A % 8) ^ 3
    //    C = A >> B
    //    A = A >> 3
    //    B = B ^ 5 ^ C
    //    OUT B & 7
    // }
    //
    // So - it kind of takes the lowest 3-6 bits, in turn, to
    // generate each output. There is are two key points:
    // 1) B starts the loop as 0..7 - but then is XORed so it has
    //    conditions on what it can be...
    // 2) C and A can have overlapping bits - so that's how we fill
    //    in the lower bits of A after the "reverse-divide"
    //
    // We should run the loop in reverse - for a range of possible
    // "final" C values (e.g. 0..7) - and see which generates a
    // possible solution. Then find the minimum.

    let orig_computer = Computer::new(input);
    let mut found_as= HashSet::new();
    reverse(&orig_computer.programme, orig_computer.programme.len(), 0, &mut found_as);

    // Check that each is a valid solution

    for a in found_as.iter().cloned()
    {
        let mut test_computer = Computer::new(input);
        test_computer.registers[0] = a;
        test_computer.run();
        assert!(test_computer.outputs == test_computer.programme);
    }

    // Return the minimum

    found_as.into_iter().min().unwrap()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(17)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: "4,6,3,5,6,3,5,2,1,0",
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: "1,6,7,4,3,0,5,0,6",
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 216148338630253i64,
        })
}
