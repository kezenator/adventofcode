use crate::support::*;
use itertools::*;

const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

fn hash(seq: &str) -> usize
{
    let mut value = 0;
    for ch in seq.chars()
    {
        value = ((value + (ch as usize)) * 17) % 256;
    }
    value
}

#[derive(Clone)]
struct SingleBox
{
    slots: Vec<(String, usize)>,
}

impl SingleBox
{
    fn new() -> Self
    {
        SingleBox { slots: Vec::new() }
    }

    fn total_focusing_power(&self, box_index: usize) -> usize
    {
        self.slots.iter().enumerate().map(|(slot_index, lense)|
            {
                (box_index + 1) * (slot_index + 1) * lense.1
            })
            .sum()
    }

    fn insert(&mut self, label: String, focal_length: usize)
    {
        if let Some(i) = self.slots.iter().enumerate().filter(|s| s.1.0 == label).map(|s| s.0).next()
        {
            self.slots[i] = (label, focal_length);
        }
        else
        {
            self.slots.push((label, focal_length));
        }
    }

    fn remove_by_label(&mut self, label: String)
    {
        self.slots = self.slots.iter()
            .filter(|slot| slot.0 != label)
            .cloned()
            .collect_vec();
    }
}

struct Boxes
{
    boxes: Vec<SingleBox>,
}

impl Boxes
{
    fn new() -> Self
    {
        Boxes { boxes: vec![SingleBox::new(); 256] }
    }

    fn initialize(&mut self, input: &str)
    {
        let steps = input.to_string().replace('\n', "").split(',').map(|s| s.to_string()).collect_vec();
        for step in steps.into_iter()
        {
            let i = step.find(|c| c == '=' || c == '-').unwrap();
            let label = step[0..i].to_string();
            let box_index = hash(&label);
            let ch = step.chars().skip(i).next().unwrap();

            match ch
            {
                '=' =>
                {
                    let focal_length = step[(i+1)..].parse::<usize>().unwrap();
                    self.boxes[box_index].insert(label, focal_length);
                },
                '-' =>
                {
                    self.boxes[box_index].remove_by_label(label);
                },
                _ => unreachable!(),
            }
        }
    }

    fn total_focusing_power(&self) -> usize
    {
        self.boxes.iter().enumerate().map(|(i, b)|
        {
            b.total_focusing_power(i)
        })
        .sum()
    }
}

fn part_1(input: &str) -> usize
{
    input.to_string().replace('\n', "")
        .split(',')
        .map(|seq| hash(seq))
        .sum()
}

fn part_2(input: &str) -> usize
{
    let mut boxes = Boxes::new();
    boxes.initialize(input);
    boxes.total_focusing_power()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(15)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 1320,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 521434,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 145,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 248279,
        })
}
