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

fn flattern_layers(layers: Vec<String>) -> String
{
    let mut result = layers[0].chars().collect::<Vec<char>>();

    for i in 1..layers.len()
    {
        for j in 0..result.len()
        {
            if result[j] == '2'
            {
                result[j] = layers[i].chars().nth(j).unwrap();
            }
        }
    }

    result.iter().map(|a| *a).collect::<String>()
}

fn print(image: String)
{
    let image = image.chars().map(|a| if a == '0' { ' ' } else { '*' }).collect::<String>();

    for i in 0..6
    {
        println!("{}", image.chars().skip(i * 25).take(25).collect::<String>());
    }
}

fn part_1() -> usize
{
    get_1_x_2_for_min_0(INPUT, 25, 6)
}

fn part_2()
{
    let image = flattern_layers(layers(INPUT, 25, 6));
    print(image);
}

fn main()
{
    assert_eq!(get_1_x_2_for_min_0("123456789012", 3, 2), 1);

    let answer_1 = part_1();
    println!("Answer #1={}", answer_1);
    assert_eq!(answer_1, 2286);

    assert_eq!(flattern_layers(layers("0222112222120000", 2, 2)), "0110");

    // Expected output: CJZLP
    println!("Answer #2=");
    part_2();
}