use aoc2019::*;

const INPUT: &str = include_str!("input_8.txt");

fn layers(input: &str, width: usize, height: usize) -> Vec<String>
{
    let per_layer = width * height;
    let num_layers = input.len() / per_layer;

    (0..num_layers).map(|i| input.chars().skip(i * per_layer).take(per_layer).collect::<String>()).collect()
}

fn get_1_x_2_for_min_0(input: &str, width: usize, height: usize) -> usize
{
    let layers = layers(input, width, height);

    let mut pairs = layers.iter().map(|l| 
        (
            l.chars().filter(|c| *c == '0').count(),
            l.chars().filter(|c| *c == '1').count() * l.chars().filter(|c| *c == '2').count(),
        )).collect::<Vec<_>>();
    pairs.sort();
    pairs[0].1
}

fn render_image(input: &str, width: usize, height: usize) -> String
{
    let mut x: usize = 0;
    let mut y: usize = 0;

    let mut paints = Vec::new();

    for ch in input.chars()
    {
        let pch = match ch
        {
            '0' => Some(' '),
            '1' => Some('*'),
            '2' => None,
            _ => Some('?'),
        };

        paints.push(PaintPoint::new(Point::new(x as i64, y as i64), pch));

        x += 1;
        if x >= width
        {
            x = 0;
            y += 1;

            if y >= height
            {
                // New layer
                y = 0;
            }
        }
    }

    paints.reverse();

    render(&paints)
}

fn part_1() -> usize
{
    get_1_x_2_for_min_0(INPUT, 25, 6)
}

fn part_2()
{
    println!("{}", render_image(INPUT, 25, 6));
}

fn main()
{
    assert_eq!(get_1_x_2_for_min_0("123456789012", 3, 2), 1);

    let answer_1 = part_1();
    println!("Answer #1={}", answer_1);
    assert_eq!(answer_1, 2286);

    assert_eq!(render_image("0222112222120000", 2, 2), " *\n* \n");

    // Expected output: CJZLP
    println!("Answer #2=");
    part_2();
}