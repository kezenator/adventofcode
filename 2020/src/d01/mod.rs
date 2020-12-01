use crate::support::*;

const EXAMPLE: &str = "1721\n979\n366\n299\n675\n1456\n";
const INPUT: &str = include_str!("input.txt");

fn part_1(input: &str) -> i64
{
    let nums = input_to_lines_parsed::<i64>(input);

    for i in 0..nums.len()
    {
        for j in 0..nums.len()
        {
            if (i != j) && (nums[i] + nums[j] == 2020)
            {
                return nums[i] * nums[j];
            }
        }
    }
    assert!(false);
    return 0;
}

fn part_2(input: &str) -> i64
{
    let nums = input_to_lines_parsed::<i64>(input);

    for i in 0..nums.len()
    {
        for j in 0..nums.len()
        {
            for k in 0..nums.len()
            {
                if (i != j) && (j != k) && (i != k) && (nums[i] + nums[j] + nums[k] == 2020)
                {
                    return nums[i] * nums[j] * nums[k];
                }
            }
        }
    }
    assert!(false);
    return 0;
}

pub fn register(puzzles: &mut PuzzleSet)
{
    puzzles.register("d01.e01", || Answer {
        calculated: part_1(EXAMPLE),
        expected: 514579,
    });

    puzzles.register("d01.e02", || Answer {
        calculated: part_2(EXAMPLE),
        expected: 241861950,
    });

    puzzles.register("d01.p1", || Answer {
        calculated: part_1(INPUT),
        expected: 357504,
    });

    puzzles.register("d01.p2", || Answer {
        calculated: part_2(INPUT),
        expected: 12747392,
    });
}
