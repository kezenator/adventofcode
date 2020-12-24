use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

struct Tile
{
    num: u64,
    grid: CharGrid,
}

impl Tile
{
    fn new(input: &Vec<String>) -> Self
    {
        let (num,_) = scan(&input[0]).skip(5).take_digits().parse::<u64>().remaining().parse::<String>();

        let grid = CharGrid::new_from_input(&input[1..].join("\n"), '.');

        Tile { num, grid }
    }

    fn borders(&self) -> Vec<String>
    {
        let mut a = String::new();
        let mut b = String::new();
        let mut c = String::new();
        let mut d = String::new();

        for i in 0..10
        {
            a.push(self.grid.get_char(&Point::new(0, i)));
            b.push(self.grid.get_char(&Point::new(9, i)));
            c.push(self.grid.get_char(&Point::new(i, 0)));
            d.push(self.grid.get_char(&Point::new(i, 9)));
        }

        vec![a, b, c, d]
    }
}



fn part_1(input: &str) -> u64
{
    let tiles = input_to_groups(input)
        .iter()
        .map(|v| Tile::new(v))
        .collect::<Vec<_>>();

    println!("Num tiles: {}", tiles.len());

    let mut corners = Vec::new();

    for t1 in tiles.iter()
    {
        let borders = t1.borders();
        let borders_rev = borders.iter().map(|b| b.chars().rev().collect::<String>()).collect::<Vec<String>>();

        let mut matches = 0;

        for t2 in tiles.iter()
        {
            if t1.num != t2.num
            {
                for t2b in t2.borders()
                {
                    if borders.iter().filter(|&t1b| *t1b == t2b).count() != 0
                        || borders_rev.iter().filter(|&t1b| *t1b == t2b).count() != 0
                    {
                        matches += 1;
                    }
                }
            }
        }

        if matches == 2
        {
            corners.push(t1.num);
        }
    }

    assert_eq!(corners.len(), 4);

    corners.into_iter().product()
}

fn part_2(_input: &str) -> u64
{
    0
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(20)
        .example(|| Answer { calculated: part_1(EXAMPLE), expected: 20899048083289u64, })
        .part_1(|| Answer { calculated: part_1(INPUT), expected: 0, })
        .example(|| Answer { calculated: part_2(EXAMPLE), expected: 0, })
        .part_2(|| Answer { calculated: part_2(INPUT), expected: 0, })
}
