use aoc2019::*;

const INPUT: &str = include_str!("input_9.txt");

fn part_1() -> i64
{
    let outputs = run_int_code(INPUT, vec!(1));
    assert_eq!(outputs.len(), 1);
    outputs[0]
}

fn part_2() -> i64
{
    let outputs = run_int_code(INPUT, vec!(2));
    assert_eq!(outputs.len(), 1);
    outputs[0]
}

fn main()
{
    assert_eq!(run_int_code(
        "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99\n",
        vec!()),
        vec!(109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99));

    assert_eq!(run_int_code(
        "1102,34915192,34915192,7,4,7,99,0\n",
        vec!()),
        vec!(1219070632396864));

    assert_eq!(run_int_code(
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