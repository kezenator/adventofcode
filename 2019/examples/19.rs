use aoc2019::*;

const INPUT: &str = include_str!("input_19.txt");

fn part_1() -> usize
{
    let size = 50;
    let mut outputs = Vec::new();

    for y in 0..size
    {
        for x in 0..size
        {
            let mut inputs = Vec::new();
            inputs.push(x as i64);
            inputs.push(y as i64);
            outputs.push(*run_int_code(INPUT, inputs).first().unwrap());
        }
    }

    /*let mut display = String::new();
    for (index, &output) in outputs.iter().enumerate()
    {
        display.push(if output == 0 { '.' } else { '#' });
        if index % size == (size - 1)
        {
            display.push('\n');
        }
    }
    println!("{}", display);*/

    outputs.iter().filter(|&o| *o == 1).count()
}

fn in_beam(x: usize, y: usize) -> bool
{
    let mut inputs = Vec::new();
    inputs.push(x as i64);
    inputs.push(y as i64);
    *run_int_code(INPUT, inputs).first().unwrap() != 0
}

fn part_2() -> usize
{
    let mut first_y = 0;
    let mut points = Vec::new();

    for x in 1350..100000
    {
        //println!("X = {}", x);

        let mut next_first_y = None;

        for y in first_y..100000
        {
            if in_beam(x, y)
            {
                if next_first_y.is_none()
                {
                    next_first_y = Some(y);

                    if !in_beam(x, y + 100)
                    {
                        break;
                    }
                }

                if in_beam(x + 99, y)
                    && in_beam(x, y + 99)
                    && !in_beam(x + 100, y)
                    && !in_beam(x, y + 100)
                {
                    //println!("Found {},{}", x, y);
                    points.push(Point::new(x as i64, y as i64));
                }
            }
            else if next_first_y.is_some()
            {
                break;
            }
        }
        first_y = next_first_y.unwrap();

        if points.len() >= 3
        {
            break;
        }
    }

    points.sort_by(|a, b| a.manhatten_dist_to(&Point::new(0, 0)).cmp(&b.manhatten_dist_to(&Point::new(0, 0))));
    (points[0].x * 10000 + points[0].y) as usize
}

fn main()
{
    let answer_1 = part_1();
    println!("Answer #1={}", answer_1);
    assert_eq!(answer_1, 138);

    let answer_2 = part_2();
    println!("Answer #2={}", answer_2);
    assert_eq!(answer_2, 13530764);
}